import { invoke } from '@tauri-apps/api'

export interface RequestOptions {
  method: string
  url: string
  query?: Record<string, any>
  headers?: Record<string, any>
  body?: Body
  responseType?: ResponseType
}

export enum ResponseType {
  Json = 1,
  Text,
  Binary
}

export interface FilePart<T> {
  file: string | T
  mime?: string
  fileName?: string
}

export type Part = string | Uint8Array | FilePart<Uint8Array>

export interface Response<T> {
  status: number
  ok: boolean
  headers: Record<string, string>
  body: T
}

export class Body {
  constructor(
    public readonly type: string, 
    public readonly payload: unknown
  ) {}

  static text = (value: string): Body => new Body('Text', value)
  static json = (value: Record<string, any>): Body => new Body('Json', value)
  static form = (value: any): Body => new Body('Form', value)
}

export namespace Client {
  export async function request<T>(options: RequestOptions): Promise<Response<T>> {
    return invoke<Response<T>>('plugin:mtls|send', { request: options }).then(
      response => ({ ...response, ok: response.status >= 200 && response.status < 300 })
    )
  }

  export function get<T>(url: string, options: Omit<RequestOptions, 'method' | 'url'>): Promise<Response<T>> {
    return request<T>({ url, method: 'GET', ...options, })
  }

  export function post<T>(url: string, options: Omit<RequestOptions, 'method' | 'url'>): Promise<Response<T>> {
    return request<T>({ url, method: 'POST', ...options, })
  }

  export function put<T>(url: string, options: Omit<RequestOptions, 'method' | 'url'>): Promise<Response<T>> {
    return request<T>({ url, method: 'PUT', ...options, })
  }

  export function patch<T>(url: string, options: Omit<RequestOptions, 'method' | 'url'>): Promise<Response<T>> {
    return request<T>({ url, method: 'PATCH', ...options, })
  }

  export function del<T>(url: string, options: Omit<RequestOptions, 'method' | 'url'>): Promise<Response<T>> {
    return request<T>({ url, method: 'DELETE', ...options, })
  }
}