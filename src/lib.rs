mod error;

use std::collections::HashMap;

use reqwest::{header::HeaderMap, Certificate, Client, Identity};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use serde_repr::Deserialize_repr;
use tauri::plugin::{Builder, TauriPlugin};
use tauri::{Manager, Runtime, State};

use error::Result;

#[tauri::command]
async fn send(client: State<'_, Client>, request: Request) -> Result<Response> {
    let method = request.method.parse()?;
    let mut builder = client.request(method, &request.url);

    if let Some(query) = request.query {
        builder = builder.query(&query);
    }

    if let Some(headers) = request.headers {
        builder = builder.headers(HeaderMap::try_from(&headers)?);
    }

    if let Some(body) = request.body {
        builder = match body {
            Body::Text(text) => builder.body(text),
            Body::Json(json) => builder.json(&json),
            Body::Form => builder,
        }
    }

    println!("builder: {:#?}", &builder);

    let result = client.execute(builder.build()?).await?;
    let response = async {
        let status = result.status().as_u16();

        let mut headers: HashMap<String, String> = HashMap::new();
        for (key, value) in result.headers() {
            headers.insert(
                key.to_string(),
                String::from_utf8(value.as_bytes().to_vec())?,
            );
        }

        let body: JsonValue = match request.response_type.unwrap_or(ResponseType::Json) {
            ResponseType::Text => JsonValue::String(result.text().await?),
            ResponseType::Json => result.json().await?,
            ResponseType::Binary => serde_json::to_value(&*result.bytes().await?)?,
        };

        Ok(Response {
            status,
            headers,
            body,
        })
    };

    Ok((response.await as Result<Response>)?)
}

pub fn init<R: Runtime>(config: ClientConfig) -> TauriPlugin<R> {
    Builder::new("mtls")
        .invoke_handler(tauri::generate_handler![send])
        .setup(move |app| {
            app.manage({
                let certificate = Certificate::from_pem(config.ca).unwrap();
                let identity = Identity::from_pem(config.cert).unwrap();
                let client = Client::builder()
                    .add_root_certificate(certificate)
                    .identity(identity)
                    .build()
                    .unwrap();

                client
            });

            Ok(())
        })
        .build()
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Request {
    pub method: String,
    pub url: String,
    pub query: Option<HashMap<String, String>>,
    pub headers: Option<HashMap<String, String>>,
    pub body: Option<Body>,
    pub response_type: Option<ResponseType>,
}

#[derive(Debug, Serialize)]
struct Response {
    pub status: u16,
    pub headers: HashMap<String, String>,
    pub body: JsonValue,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", content = "payload")]
enum Body {
    Form,
    Json(JsonValue),
    Text(String),
}

#[derive(Debug, Deserialize_repr)]
#[repr(u16)]
enum ResponseType {
    Json = 1,
    Text,
    Binary,
}

#[derive(Debug)]
pub struct ClientConfig {
    pub ca: &'static [u8],
    pub cert: &'static [u8],
}
