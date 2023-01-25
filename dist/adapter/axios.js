import { AxiosError } from 'axios';
import { getReasonPhrase } from 'http-status-codes';
import { Body, Client, ResponseType } from '../client';
export function axiosAdapter(config) {
    return new Promise(async (resolve, reject) => {
        try {
            const response = await Client.request({
                method: config.method,
                url: config.url,
                query: config.params,
                headers: config.headers,
                body: getBody(config),
                responseType: getResponseType(config)
            });
            const axiosResponse = {
                data: response.body,
                status: response.status,
                statusText: getReasonPhrase(response.status),
                headers: response.headers,
                config
            };
            if (response.ok) {
                resolve(axiosResponse);
            }
            else {
                const code = [AxiosError.ERR_BAD_REQUEST, AxiosError.ERR_BAD_RESPONSE][Math.floor(response.status / 100) - 4];
                const message = 'Request failed with status code ' + axiosResponse.status;
                reject(new AxiosError(message, code, config, {}, axiosResponse));
            }
        }
        catch (error) {
            reject(error);
        }
    });
}
function getBody(config) {
    switch (typeof config.data) {
        case 'string': return Body.text(config.data);
        case 'object': return Body.json(config.data);
        default: return undefined;
    }
}
function getResponseType(config) {
    switch (config.responseType) {
        case 'arraybuffer': return ResponseType.Binary;
        case 'json': return ResponseType.Json;
        case 'text':
        default:
            return ResponseType.Text;
    }
}
