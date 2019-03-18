import Vue from 'vue'
import VueRouter from 'vue-router'
import Home from './views/home.vue'
import Details from './views/details.vue'
import Error404 from './views/404.vue'

Vue.use(VueRouter)

const router = new VueRouter({
  mode: 'history',
  routes: [
    { path: '/', component: Home },
    { path: '/:category/:name/:version', name: 'Details', component: Details },
    { path: '/404', abstract: true, name: 'Error404', component: Error404 },
    { path: '*', component: Error404 }
  ]
})

export default router
