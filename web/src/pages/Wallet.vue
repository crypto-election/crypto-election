<template>
  <div>
    <navbar/>

    <div class="container">
      <div class="row">

        <div class="col-md-7">
          <div v-for="{administrationName, elections} of electionGroups"
               :key="administrationName" class="card mt-5">
            <div class="card-header">{{ administrationName }}</div>
            <div class="card-body">
              <div class="list-group">
                <div v-for="election of elections" :key="election.addr">
                  <button :class="{ active: !election.showResults }"
                          :data-target="'#' + 'elect-card-' + election.addr"
                          :aria-controls="'elect-card-' + election.addr" type="button"
                          class="list-group-item list-group-item-action"
                          data-toggle="collapse" aria-expanded="false"
                  >{{ election.question }}</button>
                  <div :id="'elect-card-' + election.addr" class="collapse">
                    <div :visible="!election.loading" class="card card-body">
                      <div class="d-flex flex-row">
                        <strong class="mr-2">Начало:</strong>
                        <span>{{ election.dateStart }}</span>
                      </div>
                      <div class="d-flex flex-row">
                        <strong class="mr-2">Конец:</strong>
                        <span>{{ election.dateFinish }}</span>
                      </div>
                      <vue-poll v-bind="election" @addvote="selected => vote(election, selected)" />
                    </div>
                    <spinner :visible="election.loading" />
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>

        <div class="col-md-5">
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
              <li v-for="transaction in reverseTransactions" :key="transaction.hash" class="list-group-item">
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
      </div>
    </div>

    <spinner :visible="isSpinnerVisible" />
  </div>
</template>

<script>
  import moment from 'moment';

  import { mapState } from 'vuex'
  import Modal from '../components/Modal.vue'
  import Navbar from '../components/Navbar.vue'
  import Spinner from '../components/Spinner.vue'
  import VuePoll from 'vue-poll'

  function formatTitle(election) {
    const
      dateStart = new Date(election.start_date).toLocaleString("ru"),
      dateFinish = new Date(election.finish_date).toLocaleString("ru");
    return '<div class="d-flex w-100 justify-content-between">' +
              `<h5 class="mb-1">${election.name}</h5>` +
              `<small class="text-muted">(${dateStart} - ${dateFinish})</small>` +
            '</div>';
  }

  function mapElection(election) {
    const answers = election.options.map(
      ({ id, title, votes_count }) => ({ value: id, text: title, votes: votes_count, })
    );

    return {
      addr: election.addr,
      question: election.name,
      answers,
      showResults: election.is_voted_yet,
      loading: false,
      dateStart: moment(election.start_date).fromNow(),
      dateFinish: moment(election.finish_date).fromNow(),
      multiple: false,
    };
  }

  module.exports = {
    components: { Modal, Navbar, Spinner, VuePoll },
    data() {
      return {
        name: '', email: '', phone_number: '', residence: '',
        electionGroups: [],
        receiver: '',
        isSpinnerVisible: false,
        transactions: [],
      };
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
        try {
          if (this.keyPair === null) {
            this.$store.commit('logout')
            this.$router.push({ name: 'home' })
            return
          }

          this.isSpinnerVisible = true

          const { participant, transactions } =
                  await this.$blockchain.getParticipant(this.keyPair.publicKey);
          const { name, email, phone_number, residence } = participant;
          
          this.name = name;
          this.email = email;
          this.phone_number = phone_number;
          this.residence = residence;

          this.transactions = transactions;

          const [ electionGroups, date ] =
                  await this.$blockchain.getSuggestedElections(this.keyPair.publicKey);
          
          this.electionGroups = electionGroups.map(grp => ({
            administrationName: grp.organization_name,
            elections: grp.elections.map(mapElection)
          }));
          
        } catch (error) {
          this.$notify('error', error.toString())
        } finally {
          this.isSpinnerVisible = false
        }
      },

      async vote(election, selected) {
        try {
          await this.$blockchain.vote(this.keyPair, election.addr, selected.value);
        } finally {}
      }
    },
    mounted() {
      this.$nextTick(function() {
        this.loadUser()
      })
    }
  }
</script>
