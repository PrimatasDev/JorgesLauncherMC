<script setup lang="ts">
defineProps({
  width: { type: [String, Number], default: 24 },
  height: { type: [String, Number], default: 24 },
});
</script>

<template>
  <svg
    xmlns="http://www.w3.org/2000/svg"
    xml:space="preserve"
    viewBox="0 0 256 30"
  >
    <path
      d="M409.68 192.622v36.068c0 9.953-4.512 18.034-10.07 18.034H132.044c-5.558 0-10.07-8.081-10.07-18.034v-36.068c0-9.953 4.512-18.034 10.07-18.034H399.61c5.558 0 10.07 8.081 10.07 18.034"
      transform="matrix(.8898 0 0 .41588 -108.532 -72.607)"
    />
  </svg>
</template>
