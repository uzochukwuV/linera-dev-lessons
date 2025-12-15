import { configure } from 'quasar/wrappers';

export default configure((ctx) => {
  return {
    boot: [],

    css: [
      'app.css'
    ],

    extras: [
      'roboto-font',
      'material-icons',
    ],

    build: {
      target: {
        browser: ['es2022', 'firefox115', 'chrome115', 'safari14'],
        node: 'node20'
      },

      vueRouterMode: 'hash',
    },

    devServer: {
      open: true,
      port: 3000
    },

    framework: {
      config: {},
      plugins: ['Notify', 'Dialog', 'Loading']
    },

    animations: [],

    ssr: {
      pwa: false,
      prodPort: 3000,
      middlewares: [
        'render'
      ]
    },

    pwa: {
      workboxMode: 'generateSW',
      injectPwaMetaTags: true,
      swFilename: 'sw.js',
      manifestFilename: 'manifest.json',
    },
  };
});
