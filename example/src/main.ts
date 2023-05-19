import { default as axios } from 'axios'
import { axiosAdapter } from 'tauri-plugin-http-ext/adapter/axios'

const client = axios.create({ adapter: axiosAdapter })

client
  .get('https://jsonplaceholder.typicode.com/todos/1', { headers: { 'User-Agent': 'Tauri' } })
  .then(console.log)
  .catch(console.error)