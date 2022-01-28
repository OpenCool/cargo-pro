import axios from 'axios';
import config from './config';



axios.defaults.baseURL = config[import.meta.env.MODE].baseUrl

export default axios
