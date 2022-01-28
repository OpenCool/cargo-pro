import {createApp} from 'vue'
import './assets/tailwind.css'
import App from './App.vue'
import router from './router'
import store from './store'

import Card from './components/base/BaseCard.vue'

import PerfectScrollbar from 'vue3-perfect-scrollbar'
import 'vue3-perfect-scrollbar/dist/vue3-perfect-scrollbar.css'

const app = createApp(App)


app
    .component('Card', Card)
    .use(PerfectScrollbar)
    .use(store)
    .use(router)
app.mount('#app')
