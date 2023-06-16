import { AxiosError, AxiosRequestConfig, AxiosResponse } from 'axios'
import { getReasonPhrase } from 'http-status-codes'

import { getClient, Body, ResponseType } from '../client'

export function axiosAdapter(clientName: string) {
  return (config: AxiosRequestConfig): Promise<AxiosResponse<unknown>> => {
    return new Promise(
      async (resolve, reject): Promise<void> => {
        try {
          const response = await getClient(clientName).request(
            {
              method: config.method!,
              url: buildUrl(config),
              query: castParams(config),
              headers: config.headers,
              body: getBody(config),
              responseType: getResponseType(config)
            }
          )
          
          const axiosResponse: AxiosResponse<unknown> = {
            data: response.body,
            status: response.status,
            statusText: getReasonPhrase(response.status),
            headers: response.headers,
            config
          }
  
          if (response.ok) {
            resolve(axiosResponse)
          } else {
            const code = [AxiosError.ERR_BAD_REQUEST, AxiosError.ERR_BAD_RESPONSE][Math.floor(response.status / 100) - 4]
            const message = 'Request failed with status code ' + axiosResponse.status
  
            reject(
              new AxiosError(message, code, config, {}, axiosResponse)
            )
          }
        } catch (error) {
          reject(error)
        }
      }
    )
  }
}

function getBody(config: AxiosRequestConfig): Body | undefined {
  switch (typeof config.data) {
    case 'string': return Body.text(config.data)
    case 'object': return Body.json(config.data)
    default: return undefined
  }
}

function getResponseType(config: AxiosRequestConfig): ResponseType {
  switch (config.responseType) {
    case 'arraybuffer': return ResponseType.Binary
    case 'json': return ResponseType.Json 
    case 'text':
    default:
      return ResponseType.Text
  }
}

function buildUrl(config: AxiosRequestConfig): string {
  if (config.baseURL) {
    return config.baseURL + config.url
  }

  return config.url!
}

function castParams(config: AxiosRequestConfig): Record<string, string> {
  const params = config.params || {}

  for (const key in params) {
    if (typeof params[key] === 'number') {
      params[key] = params[key].toString()
    }
  }

  return params
}