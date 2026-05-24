<script setup lang="ts">
import Titlebar from "./components/Titlebar.vue";
import Footerbar from "./components/Footerbar.vue";

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
  <div class="app-content"></div>
  <Footerbar />
</template>

<style lang="scss" scoped>
.app-content {
  background-color: $main-background;
  width: 100%;
  height: 100%;
  overflow: hidden;
}
</style>
