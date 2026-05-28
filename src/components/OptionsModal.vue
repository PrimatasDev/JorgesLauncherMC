<script setup lang="ts">
import CloseIcon from "../assets/ui_icons/CloseIcon.vue";

import { ref } from "vue";

defineProps<{
  isOpen: boolean;
}>();

const emit = defineEmits<{
  (e: "close"): void;
}>();

const memoryOptions = ["4GB", "5GB", "6GB", "8GB"];

const selectedMemory = ref("4GB");
</script>

<template>
  <div v-if="isOpen" class="modal-overlay" @click.self="emit('close')">
    <div class="modal-content">
      <div class="modal-header" data-tauri-drag-region>
        <h2>Opções do launcher</h2>
        <button class="close-btn" @click="emit('close')">
          <CloseIcon />
        </button>
      </div>
      <div class="modal-body">
        <!-- % Área de memória -->
        <div class="body-section">
          <span>Quantidade de Memória:</span>
          <div class="section-body">
            <button
              v-for="memory in memoryOptions"
              :key="memory"
              class="memory-btn"
              :class="{ selected: selectedMemory === memory }"
              @click="selectedMemory = memory"
            >
              {{ memory }}
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style lang="scss" scoped>
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  width: 100vw;
  height: 100vh;
  background-color: rgba(0, 0, 0, 0.7);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 100;
}

.modal-content {
  background-color: $header-background;
  border-radius: 12px;
  width: 450px;
  max-width: 90%;
  height: 250px;
  max-height: 90%;
  box-shadow: 0 10px 25px rgba(0, 0, 0, 0.5);
  display: flex;
  flex-direction: column;
  overflow: hidden;
  border: 1px solid color.adjust($header-background, $lightness: 10%);
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 5px 15px;
  background-color: color.adjust($header-background, $lightness: -2%);
  border-bottom: 1px solid color.adjust($header-background, $lightness: 5%);

  h2 {
    color: white;
    font-size: 1.2rem;
    font-weight: 700;
    pointer-events: none;
  }

  .close-btn {
    background: transparent;
    border: none;
    cursor: pointer;
    width: 30px;
    height: 30px;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 4px;

    svg {
      width: 16px;
      height: 16px;
      fill: color.adjust(white, $lightness: -30%);
    }

    &:hover {
      background-color: $close-btn-background;
      svg {
        fill: white;
      }
    }
  }
}

.modal-body {
  padding: 15px;
  display: flex;
  flex-direction: column;
  gap: 20px;

  > .body-section {
    display: flex;
    flex-direction: column;
    gap: 15px;
    align-items: center;
    justify-content: center;

    > span {
      color: rgb(129, 126, 126);
      font-size: 1.1rem;
    }

    .section-body {
      display: flex;
      flex-direction: row;
      gap: 10px;
      justify-content: center;
      align-items: center;

      > .memory-btn {
        background-color: transparent;
        border: 1px solid color.adjust($header-background, $lightness: 30%);
        color: white;
        padding: 12px;
        border-radius: 8px;
        cursor: pointer;
        font-family: "Lato", sans-serif;
        transition: all 0.2s;

        &:hover {
          background-color: color.adjust($main-background, $lightness: 5%);
          border-color: white;
        }
      }

      > .memory-btn.selected {
        background-color: color.adjust($main-background, $lightness: 5%);
        border-color: white;
      }
    }
  }

  .add-account-btn {
    background-color: transparent;
    border: 1px dashed color.adjust($header-background, $lightness: 30%);
    color: white;
    padding: 12px;
    border-radius: 8px;
    cursor: pointer;
    font-family: "Lato", sans-serif;
    transition: all 0.2s;

    &:hover {
      background-color: color.adjust($main-background, $lightness: 5%);
      border-color: white;
    }
  }
}
</style>
