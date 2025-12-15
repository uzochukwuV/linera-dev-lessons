import { createApp } from 'vue';
import { Quasar, Notify, Dialog, Loading } from 'quasar';
import '@quasar/extras/roboto-font/roboto-font.css';
import '@quasar/extras/material-icons/material-icons.css';
import 'quasar/dist/quasar.css';

import App from './App.vue';
import router from './router';

const app = createApp(App);

app.use(Quasar, {
  plugins: {
    Notify,
    Dialog,
    Loading,
  },
  config: {
    dark: true,
    brand: {
      primary: '#0070f3',
      secondary: '#26A69A',
      accent: '#9C27B0',
      dark: '#000000',
      positive: '#00c853',
      negative: '#ee0000',
      info: '#0070f3',
      warning: '#f5a623',
    },
  },
});

app.use(router);

app.mount('#q-app');

// Extend window interface for TypeScript
declare global {
  interface Window {
    linera: any;
  }
}
