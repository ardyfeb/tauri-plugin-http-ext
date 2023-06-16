mod error;

use std::collections::HashMap;

use http::Method;
use reqwest::{header::HeaderMap, Certificate, Client, Identity};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use serde_repr::Deserialize_repr;
use tauri::plugin::{Builder as PluginBuilder, TauriPlugin};
use tauri::{Manager, Runtime, State as TauriState, async_runtime::RwLock};

use error::Result;

#[tauri::command]
async fn send(state: TauriState<'_, State>, client_name: String, request: Request) -> Result<Response> {
    let state = state.read().await;
    let client = state.get(&client_name).unwrap();

    let method = Method::from_bytes(request.method.to_uppercase().as_bytes())?;

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

    let result = builder.send().await?;
    let response = async {
        let status = result.status().as_u16();

        let mut headers: HashMap<String, String> = HashMap::new();
        for (key, value) in result.headers() {
            headers.insert(key.to_string(), String::from_utf8(value.as_bytes().to_vec())?);
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

pub type Clients = HashMap<String, Client>;
pub type State = RwLock<Clients>;

pub struct Builder {
    clients: HashMap<String, Client>,
}

impl Builder {
    pub fn new() -> Self {
        Self {
            clients: HashMap::new(),
        }
    }

    pub fn add_client(mut self, name: &str, config: ClientConfig) -> Self {
        let mut client = Client::builder();

        if let Some(tls) = config.tls {
            client = client
                // .tls_built_in_root_certs(false)
                .use_rustls_tls()
                .add_root_certificate(Certificate::from_pem(tls.cert).unwrap());

            if let Some(key) = tls.key {
                client = client.identity(Identity::from_pem(key).unwrap());
            }
        }

        self.clients.insert(name.to_string(), client.build().unwrap());
        self
    }

    pub fn build<R: Runtime>(self) -> TauriPlugin<R> {
        let plugin = PluginBuilder::new("mtls")
            .invoke_handler(tauri::generate_handler![send])
            .setup(
                move |app| {
                    app.manage(RwLock::new(self.clients.clone())); Ok(())
                }
            )
            .build();

        plugin
    }
}

#[derive(Debug)]
pub struct ClientConfig {
    pub tls: Option<ClientTlsConfig>,
}

#[derive(Debug)]
pub struct ClientTlsConfig {
    pub cert: &'static [u8],
    pub key: Option<&'static [u8]>,
}

impl ClientConfig {
    pub fn new() -> Self {
        Self {
            tls: None,
        }
    }

    pub fn tls(mut self, cert: &'static [u8], key: Option<&'static [u8]>) -> Self {
        let tls = ClientTlsConfig {
            cert,
            key,
        };

        self.tls = Some(tls);
        self
    }
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