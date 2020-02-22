<template>
  <div>
    <div class="container">
      <div class="row justify-content-sm-center">
        <div class="col-md-6 col-md-offset-3">
          <h1 class="mt-5 mb-4">Авторизация</h1>
          <tabs>
            <!--Введение параметров в формы регистрации-->
            <tab :is-active="true" title="Register">
              <form @submit.prevent="register">
                <div class="form-group">
                  <label for="name" class="control-label">Логин:</label>
                  <input
                    id="name"
                    v-model.trim="name"
                    type="text"
                    class="form-control"
                    placeholder="Введите ваш логин"
                    maxlength="260"
                    required
                  >
                </div>
                <!--Email-->
                <div class="form-group">
                  <label for="email" class="control-label">Email:</label>
                  <input
                    id="email"
                    v-model.trim="email"
                    type="email"
                    class="form-control"
                    placeholder="Введите email"
                    maxlength="260"
                    required
                  >
                </div>
                <!--Phone-->
                <div class="form-group">
                  <label for="phone" class="control-label">Телефон:</label>
                  <input
                    id="phone"
                    v-model.trim="phone_number"
                    type="tel"
                    pattern="80[0-9]{9}"
                    class="form-control"
                    placeholder="80XXXXXXXXX"
                    maxlength="260"
                    required
                  >
                </div>
                <!--Residence-->
                <div class="form-group">
                  <label for="residence" class="control-label">Прописка:</label>
                  <!--ToDo: Добавить treeselect (https://vue-treeselect.js.org/)-->
                  <input
                    id="residence"
                    v-model.trim="residence"
                    type="text"
                    class="form-control"
                    placeholder="Введите вашу резиденцию (?)"
                    maxlength="260"
                  >
                </div>
                <!--Pass_code-->
                <div class="form-group">
                  <label for="pass_code" class="control-label">Паспорт:</label>
                  <input
                    id="pass_code"
                    v-model.trim="pass_code"
                    type="text"
                    class="form-control"
                    placeholder="Введите данные паспорта"
                    maxlength="260"
                    required
                  >
                </div>
                <button type="submit" class="btn btn-lg btn-block btn-primary">Регистрация</button>
              </form>
            </tab>
            <tab title="Log in">
              <form @submit.prevent="login">
                <div class="form-group">
                  <label class="control-label">Secret key:</label>
                  <input
                    v-model="secretKey"
                    type="text"
                    class="form-control"
                    placeholder="Enter secret key"
                    required
                  >
                </div>
                <button type="submit" class="btn btn-lg btn-block btn-primary">Log in</button>
              </form>
            </tab>
          </tabs>
        </div>
      </div>
    </div>

    <modal
      :visible="isModalVisible"
      title="Wallet has been created"
      action-btn="Log in"
      @close="closeModal"
      @submit="proceed"
    >
      <div
        class="alert alert-warning"
        role="alert"
      >Save the secret key in a safe place. You will need it to log in to the demo next time.</div>
      <div class="form-group">
        <label>Secret key:</label>
        <div>
          <code>{{ keyPair.secretKey }}</code>
        </div>
      </div>
    </modal>

    <spinner :visible="isSpinnerVisible" />
  </div>
</template>

<script>
import Tab from "../components/Tab.vue";
import Tabs from "../components/Tabs.vue";
import Modal from "../components/Modal.vue";
import Spinner from "../components/Spinner.vue";

module.exports = {
  components: {
    Tab,
    Tabs,
    Modal,
    Spinner
  },
  data() {
    return {
      name: "",
      email: "",
      phone_number: "",
      residence: "",
      pass_code: "",
      secretKey: "",
      keyPair: {},
      isModalVisible: false,
      isSpinnerVisible: false
    };
  },
  methods: {
    login() {
      if (!this.$validateHex(this.secretKey, 64)) {
        return this.$notify("error", "Invalid secret key is passed");
      }

      this.isSpinnerVisible = true;

      this.$store.commit("login", {
        publicKey: this.secretKey.substr(64),
        secretKey: this.secretKey
      });

      this.$nextTick(function() {
        this.$router.push({ name: "user" });
      });
    },

    async register() {
      if (!this.name) {
        return this.$notify("error", "The name is a required field");
      }
      if (!this.email) {
        return this.$notify("error", "The email is a required field");
      }
      if (!this.phone_number) {
        return this.$notify("error", "The phone number is a required field");
      }

      this.isSpinnerVisible = true;
      this.keyPair = this.$blockchain.generateKeyPair();

      try {
        await this.$blockchain.createParticipant(this.keyPair, {
          name: this.name,
          email: this.email,
          phone_number: this.phone_number,
          //residence: this.residence,
          pass_code: this.pass_code
        });

        this.name = "";
        this.email = "";
        this.phone_number = "";
        this.residence = "";
        this.pass_code = "";
        this.isSpinnerVisible = false;
        this.isModalVisible = true;
      } catch (error) {
        this.isSpinnerVisible = false;
        this.$notify("error", error.toString());
      }
    },

    closeModal() {
      this.isModalVisible = false;
    },

    proceed() {
      this.isModalVisible = false;

      this.$store.commit("login", this.keyPair);

      this.$nextTick(function() {
        this.$router.push({ name: "user" });
      });
    }
  }
};
</script>
