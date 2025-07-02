import './assets/main.css'

import { createApp } from "vue";
import { createPinia } from 'pinia'
import App from "./App.vue";
import Toast from "vue-toastification";
import "vue-toastification/dist/index.css";

const app = createApp(App);
app.use(createPinia());

const options = {};
app.use(Toast, options);

app.mount('#app');
