import { invoke } from '@tauri-apps/api';
export var ResponseType;
(function (ResponseType) {
    ResponseType[ResponseType["Json"] = 1] = "Json";
    ResponseType[ResponseType["Text"] = 2] = "Text";
    ResponseType[ResponseType["Binary"] = 3] = "Binary";
})(ResponseType || (ResponseType = {}));
export class Body {
    type;
    payload;
    constructor(type, payload) {
        this.type = type;
        this.payload = payload;
    }
    static text = (value) => new Body('Text', value);
    static json = (value) => new Body('Json', value);
    static form = (value) => new Body('Form', value);
}
export var Client;
(function (Client) {
    async function request(options) {
        return invoke('plugin:mtls|send', { request: options }).then(response => ({ ...response, ok: response.status >= 200 && response.status < 300 }));
    }
    Client.request = request;
    function get(url, options) {
        return request({ url, method: 'GET', ...options, });
    }
    Client.get = get;
    function post(url, options) {
        return request({ url, method: 'POST', ...options, });
    }
    Client.post = post;
    function put(url, options) {
        return request({ url, method: 'PUT', ...options, });
    }
    Client.put = put;
    function patch(url, options) {
        return request({ url, method: 'PATCH', ...options, });
    }
    Client.patch = patch;
    function del(url, options) {
        return request({ url, method: 'DELETE', ...options, });
    }
    Client.del = del;
})(Client || (Client = {}));
