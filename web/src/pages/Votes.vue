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
                  <div class="col-sm-3"><strong>Номер телефона:</strong></div>
                  <div class="col-sm-9">{{ phone_number }}</div>
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
          <!--Транзакции-->
          <div class="card mt-5">
            <div class="card-header">Транзакция</div>
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
                      <span v-if="transaction.name">Избирательный аккаунт создан</span>
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
            <div class="card-header">Список голосований</div>
            <!-- As a link -->
            <nav class="nav flex-column">
              <button
                v-for="(navOption, index) in options"
                :key="navOption.question"
                class="btn btn-primary"
                type="submit"
                @click="choiseFunc(index)"
              >{{ navOption.question }}
              </button>
            </nav>
          </div>
        
          <div class="card mt-5">
            <div class="card-header">Голосование</div>
            <div class="col-12">
              <vue-poll
                v-for="(option) of options"
                :key="option.id"
                v-bind="option"
                @addvote="addVote"
              />
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
        phone_number: 0,
        // receiver: '',
        // amountToTransfer: '',
        isSpinnerVisible: false,
        transactions: [],
        choice: "",
        visible: false,
        options: {
          1: {
            question: "Какой для вас JS framework является лучшим?",
            answers: [
              { value: 1, text: "Vue", votes: 53 },
              { value: 2, text: "React", votes: 35 },
              { value: 3, text: "Angular", votes: 30 },
              { value: 4, text: "Other", votes: 10 }
            ]
          },
          2: {
            question: "В каком вузе вы учитесь?",
            answers: [
              { value: 1, text: "ДОННУ", votes: 20 },
              { value: 2, text: "ДОННТУ", votes: 17 },
              { value: 3, text: "ДОНАУИГС", votes: 100 }
            ]
          }
        }
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
      addVote(obj){
        console.log('You voted ' + obj.value + '!');
      },
      choiseFunc: function(a) {
        this.choice = a;
      },
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
          this.transactions = transactions
          this.isSpinnerVisible = false
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
      }
    },
    mounted() {
      this.$nextTick(function() {
        this.loadUser()
      })
    }
  }
</script>
