<template>
  <div>
    <navbar/>

    <div class="container">
      <div class="row">
        <div class="col-md-6">
          <div class="card mt-5">
            <div class="card-header">Информация о пользователе</div>
            <ul class="list-group list-group-flush">
              <li class="list-group-item">
                <div class="row">
                  <div class="col-sm-3"><strong>Логин:</strong></div>
                  <div class="col-sm-9">{{ name }}</div>
                </div>
              </li>
              <li class="list-group-item">
                <div class="row">
                  <div class="col-sm-3"><strong>Email:</strong></div>
                  <div class="col-sm-9">{{ email }}</div>
                </div>
              </li>
              <li class="list-group-item">
                <div class="row">
                  <div class="col-sm-3"><strong>Телефон:</strong></div>
                  <div class="col-sm-9">{{ phone_number }}</div>
                </div>
              </li>
              <li class="list-group-item">
                <div class="row">
                  <div class="col-sm-3"><strong>Район:</strong></div>
                  <div class="col-sm-9">{{ residence }}</div>
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
            <div class="card-header">Выбор голосования</div>
            <div class="card-body">
              <form @submit.prevent="getElection">
                <div class="form-group">
                  <label class="d-block">Выберите голосование из списка:</label>
                  <select v-model="selectedform">
                    <option v-for="optionform in optionsform" v-bind:value="optionform.value">
                      {{ optionform.text }}
                    </option>
                  </select>
                  <span>Выбрано: {{ selectedform }}</span>
                </div>
                <button type="submit" class="btn btn-primary">Выбор голосования</button>
              </form>
            </div>
          </div>

          <div class="card mt-5">
            <div class="card-header">Голосование</div>
            <div class="card-body">
              <div>
                <vue-poll v-bind="options" @addvote="addVote"/>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <spinner :visible="isSpinnerVisible"/>
  </div>
</template>

<script>
  import { mapState } from 'vuex'
  import Modal from '../components/Modal.vue'
  import Navbar from '../components/Navbar.vue'
  import Spinner from '../components/Spinner.vue'
  import VuePoll from 'vue-poll'

  module.exports = {
    components: {
      Modal,
      Navbar,
      Spinner,
      VuePoll
    },
    data() {
      return {
        name: '',
        email: '',
        phone_number: '',
        residence: '',
        options: {
                    question: 'What\'s your favourite <strong>JS</strong> framework?',
                    answers: [
                        { value: 1, text: 'Vue', votes: 53 },
                        { value: 2, text: 'React', votes: 35 },
                        { value: 3, text: 'Angular', votes: 30 },
                        { value: 4, text: 'Other', votes: 10 } 
                    ]
                },
        receiver: '',
        isSpinnerVisible: false,
        transactions: [],
        selectedform: '',
        optionsform: {}
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
          const { participant, transactions } =
                  await this.$blockchain.getParticipant(this.keyPair.publicKey)
          this.name = participant.name
          this.email = participant.email
          this.phone_number = participant.phone_number
          this.residence = participant.residence
          this.transactions = transactions
          this.isSpinnerVisible = false
        } catch (error) {
          this.isSpinnerVisible = false
          this.$notify('error', error.toString())
        }
      },
      async getElection(){
        this.isSpinnerVisible = true

        const seed = this.$blockchain.generateSeed()
        // help me
        try {
          await this.$blockchain.getElection(this.keyPair, seed)
          const data = await this.$blockchain.getParticipant(this.keyPair.publicKey)
          this.optionsform = data.election.name
          this.transactions = data.transactions
          this.isSpinnerVisible = false
          this.$notify('success', 'Get election transaction has been written into the blockchain')
        } catch (error) {
          this.isSpinnerVisible = false
          this.$notify('error', error.toString())
        }
      },
      async addFunds() {
        this.isSpinnerVisible = true

        const seed = this.$blockchain.generateSeed()

        try {
          await this.$blockchain.addFunds(this.keyPair, this.amountToAdd, seed)
          const data = await this.$blockchain.getParticipant(this.keyPair.publicKey)
          this.balance = data.wallet.balance
          this.transactions = data.transactions
          this.isSpinnerVisible = false
          this.$notify('success', 'Add funds transaction has been written into the blockchain')
        } catch (error) {
          this.isSpinnerVisible = false
          this.$notify('error', error.toString())
        }
      },

      async transfer() {
        if (!this.$validateHex(this.receiver)) {
          return this.$notify('error', 'Invalid public key is passed')
        }

        if (this.receiver === this.keyPair.publicKey) {
          return this.$notify('error', 'Can not transfer funds to yourself')
        }

        this.isSpinnerVisible = true

        const seed = this.$blockchain.generateSeed()

        try {
          await this.$blockchain.transfer(this.keyPair, this.receiver, this.amountToTransfer, seed)
          const data = await this.$blockchain.getParticipant(this.keyPair.publicKey)
          this.balance = data.wallet.balance
          this.transactions = data.transactions
          this.isSpinnerVisible = false
          this.$notify('success', 'Transfer transaction has been written into the blockchain')
        } catch (error) {
          this.isSpinnerVisible = false
          this.$notify('error', error.toString())
        }
      },

      addVote(obj){
          console.log('You voted ' + obj.value + '!');
      }
    },
    mounted() {
      this.$nextTick(function() {
        this.loadUser()
      })
    }
  }
</script>
