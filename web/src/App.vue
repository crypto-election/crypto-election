<template>
  <div>
    <router-view/>

    <footer class="pb-4 hr">
      <hr class="mt-5 mb-5">
      <div class="container" />
    </footer>
  </div>
</template>

<style lang="scss">
  @import '~bootstrap/scss/bootstrap.scss';
  @import '~noty/lib/noty.css';
  @import '~noty/lib/themes/bootstrap-v4.css';

  input:invalid {
    box-shadow: none;
  }

  code {
    word-break: break-all;
    word-wrap: break-word;
  }
</style>
