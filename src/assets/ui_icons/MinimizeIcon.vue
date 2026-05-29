<script setup lang="ts">
defineProps({
  width: { type: [String, Number], default: 24 },
  height: { type: [String, Number], default: 24 },
});
</script>

<template>
  <svg
    width="24"
    height="24"
    fill="currentColor"
    viewBox="0 0 24 24"
    transform=""
    id="injected-svg"
    xmlns="http://www.w3.org/2000/svg"
  >
    <path d="M3 11h18v2H3z"></path>
  </svg>
</template>
