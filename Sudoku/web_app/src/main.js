import Vue from 'vue'
import 'es6-promise/auto'
import Vuex from 'vuex'
import App from './App.vue'
import router from './router'

Vue.config.productionTip = false
Vue.use(Vuex)

const store = new Vuex.Store({
  state: {
    input: []
  },
  mutations: {
    change (input, payload) {
      input[payload.id] = payload.value
    }
  }
})

new Vue({
  router,
  store,
  render: h => h(App)
}).$mount('#app')