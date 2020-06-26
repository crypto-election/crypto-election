<template>
  <nav class="navbar navbar-expand-lg navbar-light bg-light">
    <div class="container">
      <router-link :to="{ name: isAuthorized ? 'newpoll' : 'home' }" class="navbar-brand">Crypto-election</router-link>
      <button class="navbar-toggler" type="button" data-toggle="collapse" data-target="#navbar" aria-controls="navbar" aria-expanded="false" aria-label="Toggle navigation">
        <span class="navbar-toggler-icon"/>
      </button>
      <div id="navbar" class="collapse navbar-collapse">
        <ul class="navbar-nav">
          <li class="nav-item">
            <router-link :to="{ name: 'blockchain' }" class="nav-link">Блокчейн</router-link>
          </li>
        </ul>
        <ul v-if="isAuthorized" class="navbar-nav ml-auto">
          <li class="nav-item">
            <a href="#/admin" class="nav-link" @click="logout">Выход</a>
          </li>
        </ul>
      </div>
    </div>
  </nav>
</template>

<script>
  module.exports = {
    name: 'navbaradmin',
    computed: {
      isAuthorized() {
        return this.$store.state.keyPair !== null
      }
    },
    methods: {
      logout() {
        this.$store.commit('logout')
        this.$router.push({ name: 'home' })
      }
    }
  }
</script>
