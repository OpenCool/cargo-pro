import {createApp} from 'vue'
import './assets/tailwind.css'
import App from './App.vue'
import router from './router'
import store from './store'

import Card from './components/base/BaseCard.vue'

import PerfectScrollbar from 'vue3-perfect-scrollbar'
import 'vue3-perfect-scrollbar/dist/vue3-perfect-scrollbar.css'

import VMdPreview from '@kangc/v-md-editor/lib/preview';
import '@kangc/v-md-editor/lib/style/preview.css';
import githubTheme from '@kangc/v-md-editor/lib/theme/github.js';
import '@kangc/v-md-editor/lib/theme/style/github.css';

// highlightjs
import hljs from 'highlight.js';
import 'highlight.js/styles/monokai.css';



const app = createApp(App)


VMdPreview.use(githubTheme, {
    Hljs: hljs,
});

app
    .component('Card', Card)
    .use(PerfectScrollbar)
    .use(store)
    .use(router)
    .use(VMdPreview)
app.mount('#app')
