<template>
  <div>
    <navbaradmin/>

    <div class="container">
      <div class="row">
        <div class="col-md-6">
          <div class="card mt-5">
            <div class="card-header">Информация о пользователе</div>
            <ul class="list-group list-group-flush">
              <li class="list-group-item">
                <div class="row">
                  <div class="col-sm-3"><strong>Логин:</strong></div>
                  <div class="col-sm-9">{{ login }}</div>
                </div>
              </li>
              <li class="list-group-item">
                <div class="row">
                  <div class="col-sm-3"><strong>Публичный ключ:</strong></div>
                  <div class="col-sm-9"><code>{{ keyPair.publicKey }}</code></div>
                </div>
              </li>
            </ul>
          </div>
          <div class="card mt-5">
            <div class="card-header">Транзакции</div>
            <ul class="list-group list-group-flush">
              <li class="list-group-item font-weight-bold">
                <div class="row">
                  <div class="col-sm-12">Описание</div>
                </div>
              </li>
              <!-- eslint-disable-next-line vue/require-v-for-key -->
              <li v-for="transaction in reverseTransactions" class="list-group-item">
                <div class="row">
                  <div class="col-sm-12">
                    <router-link :to="{ name: 'transaction', params: { hash: transaction.hash } }">
                      <span v-if="transaction.name">Создание кабинета избирателя</span>
                      <span v-else-if="transaction.to && transaction.to === keyPair.publicKey">
                        <strong v-numeral="transaction.amount"/> funds received
                      </span>
                      <span v-else-if="transaction.to">
                        <strong v-numeral="transaction.amount"/> funds sent
                      </span>
                      <span v-else>
                        <strong v-numeral="transaction.amount"/> funds added
                      </span>
                    </router-link>
                  </div>
                </div>
              </li>
            </ul>
          </div>
        </div>
        <div class="col-md-6">
          <div class="card mt-5">
            <div class="card-header">Регистрация голосования</div>
            <form @submit.prevent="newpoll">
              <div class="form-group">
                <label for="name" class="control-label">Название:</label>
                <input
                  id="name"
                  v-model.trim="name"
                  type="text"
                  class="form-control"
                  maxlength="260"
                  required
                >
              </div>
              <div class="form-group">
                <label for="start_date" class="control-label">Дата начала голосования:</label>
                <input
                  id="start_date"
                  v-model.trim="start_date"
                  type="date"
                  class="form-control"
                  maxlength="260"
                >
              </div>
              <div class="form-group">
                <label for="finish_date" class="control-label">Дата конца голосования:</label>
                <input
                  id="finish_date"
                  v-model.trim="finish_date"
                  type="date"
                  class="form-control"
                  maxlength="260"
                >
              </div>
              <div class="form-group">
                <label for="options" class="control-label">Вопросы:</label>
                <input
                  id="options"
                  v-model.trim="options"
                  type="text"
                  class="form-control"
                  placeholder="Вводите вопросы через запятую"
                  maxlength="500"
                  required
                >
              </div>
              <button type="submit" class="btn btn-lg btn-block btn-primary">Создать голосование</button>
            </form>
          </div>
        </div> 
      </div>
    </div>




    <spinner :visible="isSpinnerVisible"/>
  </div>
</template>

<script>
  import { mapState } from 'vuex'
  import NavbarAdmin from '../components/NavbarAdmin.vue'
  import Spinner from '../components/Spinner.vue'

  module.exports = {
    components: {
      NavbarAdmin,
      Spinner
    },
    data() {
      return {
        isSpinnerVisible: false,
        login: '',
        name: "",
        start_date: "",
        finish_date: "",
        options: "",
        //keyPair: {},
        transactions: []
      }
    },
    computed: Object.assign({},
      mapState({ keyPair: state => state.keyPair })
    ),
    methods: {
      async loadUser() {
        if (this.keyPair === null) {
          this.$store.commit('logout')
          this.$router.push({ name: 'home' })
          return
        }

        this.isSpinnerVisible = true

        try {
          const { administration, transactions } =
                  await this.$blockchain.getAdministration(this.keyPair.publicKey)
          this.login = administration.name
          this.transactions = transactions
          this.isSpinnerVisible = false
        } catch (error) {
          this.isSpinnerVisible = false
          this.$notify('error', error.toString())
        }
      },
      async newpoll() {
      if (!this.name) {
        return this.$notify("error", "The name is a required field");
      }
      if (!this.options) {
        return this.$notify("error", "The options is a required field");
      }

      this.isSpinnerVisible = true;

      try {
        await this.$blockchain.createNewPoll(this.keyPair, {
          name: this.name,
        //  start_date: this.start_date,
        //  finish_date: this.finish_date,
        //  options: this.options
        });

        this.name = "";
        //this.start_date = "";
        //this.finish_date = "";
        //this.options = "";

        this.isSpinnerVisible = false;
      } catch (error) {
        this.isSpinnerVisible = false;
        this.$notify("error", error.toString());
      }
    }
    }
  }
</script>
