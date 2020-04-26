import Vue from 'vue'
import Vuex from 'vuex'

const KEY_KEYPAIR = 'cryptoelection-keypair'
const keyPair = JSON.parse(localStorage.getItem(KEY_KEYPAIR))
const KEY_USER_KIND = 'cryptoelection-userkind'
const userKind = JSON.parse(localStorage.getItem(KEY_USER_KIND))

Vue.use(Vuex)

export default new Vuex.Store({
  state: {
    keyPair: keyPair,
    userKind: userKind
  },
  mutations: {
    login: (state, keyPair) => {
      localStorage.setItem(KEY_KEYPAIR, JSON.stringify(keyPair))
      state.keyPair = keyPair
      localStorage.setItem(KEY_USER_KIND, JSON.stringify(userKind))
      state.userKind = userKind
    },
    logout: state => {
      localStorage.removeItem(KEY_KEYPAIR)
      state.keyPair = null
      localStorage.removeItem(KEY_USER_KIND)
      state.userKind = null
    }
  }
})
