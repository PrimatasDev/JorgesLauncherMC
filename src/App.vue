<script setup lang="ts">
import { ref } from "vue";

import Titlebar from "./components/Titlebar.vue";
import Bottombar from "./components/Bottombar.vue";

import OptionsModal from "./components/OptionsModal.vue";
import AccountsModal from "./components/AccountsModal.vue";

import background from "./assets/img/background.png";

const isSettingsOpen = ref(false);
const isAccountsOpen = ref(false);

document.addEventListener(
  "contextmenu",
  (e) => {
    e.preventDefault();
  },
  false,
);
</script>

<template>
  <Titlebar />
  <div class="app-content">
    <div class="jorges-modpack">
      <div class="image-bg">
        <img :src="background" />
      </div>
    </div>
  </div>

  <OptionsModal :is-open="isSettingsOpen" @close="isSettingsOpen = false" />
  <AccountsModal :is-open="isAccountsOpen" @close="isAccountsOpen = false" />

  <Bottombar
    @open-settings="isSettingsOpen = true"
    @open-accounts="isAccountsOpen = true"
  />
</template>

<style lang="scss" scoped>
.app-content {
  background-color: $main-background;
  width: 100%;
  height: calc(100vh - 42px);
  overflow: hidden;
}

.jorges-modpack {
  width: 100%;
  height: 100%;

  > .image-bg {
    > img {
      width: 100%;
      height: 100%;
      filter: brightness(0.5);
      transform: translateY(-30px);
    }
  }
}
</style>
