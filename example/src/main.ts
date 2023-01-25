import { default as axios } from 'axios'
import { axiosAdapter } from 'tauri-plugin-http-ext/adapter/axios'

const client = axios.create({ adapter: axiosAdapter })

client
  .get('https://mtls.ardyfeb.dev')
  .then(console.log)
  .catch(console.error)