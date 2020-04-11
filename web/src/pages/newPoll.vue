<template>
  <div>
    <navbar/>

    <div class="container">
      <div class="row">
        <div class="col-sm-12">
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
  import Navbar from '../components/Navbar.vue'
  import Spinner from '../components/Spinner.vue'

  module.exports = {
    components: {
      Navbar,
      Spinner
    },
    data() {
      return {
        isSpinnerVisible: false,
        name: "",
        start_date: "",
        finish_date: "",
        options: [],
        keyPair: {}
      }
    },
    methods: {
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
