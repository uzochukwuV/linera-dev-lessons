import { route } from 'quasar/wrappers';
import {
  createRouter,
  createWebHashHistory,
} from 'vue-router';

import MainLayout from 'layouts/MainLayout.vue';
import IndexPage from 'pages/IndexPage.vue';

const routes = [
  {
    path: '/',
    component: MainLayout,
    children: [{ path: '', component: IndexPage }],
  },
];

export default route(function () {
  const Router = createRouter({
    scrollBehavior: () => ({ left: 0, top: 0 }),
    routes,
    history: createWebHashHistory(),
  });

  return Router;
});
