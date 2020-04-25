import Vue from 'vue'
import Router from 'vue-router'
import AuthPage from '../pages/Auth.vue'
import WalletPage from '../pages/Wallet.vue'
import BlockchainPage from '../pages/Blockchain.vue'
import BlockPage from '../pages/Block.vue'
import TransactionPage from '../pages/Transaction.vue'
import Votes from '../pages/Votes.vue'
import NewPoll from '../pages/NewPoll.vue'
import AuthAdmin from '../pages/AuthAdmin.vue'

Vue.use(Router)

export default new Router({
  routes: [
    {
      path: '/',
      name: 'home',
      component: AuthPage
    },
    {
      path: '/admin',
      name: 'admin',
      component: AuthAdmin
    },
    {
      path: '/newpoll',
      name: 'newpoll',
      component: NewPoll
    },
    {
      path: '/user',
      name: 'user',
      component: WalletPage
    },
    {
      path: '/votes',
      name: 'votes',
      component: Votes
    },
    {
      path: '/blockchain',
      name: 'blockchain',
      component: BlockchainPage
    },
    {
      path: '/block/:height',
      name: 'block',
      component: BlockPage,
      props: true
    },
    {
      path: '/transaction/:hash',
      name: 'transaction',
      component: TransactionPage,
      props: true
    }
  ]
})
