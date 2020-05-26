<template>
  <div>
    <navbarAdmin />

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
              <div class="form-group px-2">
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
              <div class="form-group px-2">
                
                <label for="start-date" class="control-label">Дата начала голосования:</label>
                
                <datetime
                  id="start-date"
                  v-model="startDate"
                  :min-datetime="now"
                  input-class="form-control"
                  type="datetime"
                />
              </div>
              <div class="form-group px-2">
                
                <label for="finish-date" class="control-label">Дата конца голосования:</label>
                
                <datetime
                  id="finish-date"
                  v-model="finishDate"
                  :min-datetime="startDate"
                  input-class="form-control"
                  type="datetime"
                />
              </div>
              <div class="form-group px-2">
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
              <button type="submit" class="btn btn-lg btn-block btn-primary px-2">Создать голосование</button>
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

  import { Datetime } from 'vue-datetime';
  import 'vue-datetime/dist/vue-datetime.css'

  module.exports = {
    components: {
      NavbarAdmin,
      Spinner,
      datetime: Datetime
    },

    data() {
      return {
        isSpinnerVisible: false,
        login: '',
        name: "",
        now: new Date().toString(),
        startDate: null,
        finishDate: null,
        options: "",
        transactions: []
      }
    },

    computed: Object.assign({
      reverseTransactions() {
        return this.transactions.slice().reverse()
      }
    }, mapState({
      keyPair: state => state.keyPair
    })),
    
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
        
        let options = this.options.split(",").map(s => s.trim()).filter(s => s);
        if (!options) {
          return this.$notify("error", "The options is a required field");
        }

        if (!this.startDate) {
          return this.$notify("error", "The start date is a required field");
        }

        if (!this.finishDate) {
          return this.$notify("error", "The finish date is a required field");
        }

        this.isSpinnerVisible = true;

        try {
          await this.$blockchain.createNewPoll(this.keyPair, {
            name: this.name,
            startDate: Date.parse(this.startDate),
            finishDate: Date.parse(this.finishDate),
            options
          });

          // this.name = "";
          // this.startDate = null;
          // this.finishDate = null;
          // this.options = "";

          this.$notify("success", "Голосование создано");
          this.isSpinnerVisible = false;
        } catch (error) {
          this.isSpinnerVisible = false;
          this.$notify("error", error.toString());
        }
      }
    },
    
    mounted() {
      this.$nextTick(function() {
        this.loadUser()
      })
    }
  }
</script>
