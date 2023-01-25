export interface RequestOptions {
    method: string;
    url: string;
    query?: Record<string, any>;
    headers?: Record<string, any>;
    body?: Body;
    responseType?: ResponseType;
}
export declare enum ResponseType {
    Json = 1,
    Text = 2,
    Binary = 3
}
export interface FilePart<T> {
    file: string | T;
    mime?: string;
    fileName?: string;
}
export declare type Part = string | Uint8Array | FilePart<Uint8Array>;
export interface Response<T> {
    status: number;
    ok: boolean;
    headers: Record<string, string>;
    body: T;
}
export declare class Body {
    readonly type: string;
    readonly payload: unknown;
    constructor(type: string, payload: unknown);
    static text: (value: string) => Body;
    static json: (value: Record<string, any>) => Body;
    static form: (value: any) => Body;
}
export declare namespace Client {
    function request<T>(options: RequestOptions): Promise<Response<T>>;
    function get<T>(url: string, options: Omit<RequestOptions, 'method' | 'url'>): Promise<Response<T>>;
    function post<T>(url: string, options: Omit<RequestOptions, 'method' | 'url'>): Promise<Response<T>>;
    function put<T>(url: string, options: Omit<RequestOptions, 'method' | 'url'>): Promise<Response<T>>;
    function patch<T>(url: string, options: Omit<RequestOptions, 'method' | 'url'>): Promise<Response<T>>;
    function del<T>(url: string, options: Omit<RequestOptions, 'method' | 'url'>): Promise<Response<T>>;
}
