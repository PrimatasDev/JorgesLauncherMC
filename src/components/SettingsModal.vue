<script setup lang="ts">
import CloseIcon from "../assets/ui_icons/CloseIcon.vue";

defineProps<{
  isOpen: boolean;
}>();

const emit = defineEmits<{
  (e: "close"): void;
}>();
</script>

<template>
  <div v-if="isOpen" class="modal-overlay" @click.self="emit('close')">
    <div class="modal-content">
      <div class="modal-header">
        <h2>Configurações do Launcher</h2>
        <button class="close-btn" @click="emit('close')">
          <CloseIcon />
        </button>
      </div>
      <div class="modal-body">
        <!-- Placeholder configuration options -->
        <div class="settings-group">
          <label>Memória RAM Máxima (MB)</label>
          <input type="number" value="4096" />
        </div>
        <div class="settings-group">
          <label>Caminho do Java (Opcional)</label>
          <input type="text" placeholder="Caminho para o javaw.exe" />
        </div>
      </div>
      <div class="modal-footer">
        <button class="save-btn" @click="emit('close')">Salvar</button>
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
  width: 500px;
  max-width: 90%;
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
  padding: 15px 20px;
  background-color: color.adjust($header-background, $lightness: -2%);
  border-bottom: 1px solid color.adjust($header-background, $lightness: 5%);

  h2 {
    color: white;
    font-size: 1.2rem;
    font-weight: 700;
    font-family: "Lato", sans-serif;
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
  padding: 20px;
  display: flex;
  flex-direction: column;
  gap: 15px;

  .settings-group {
    display: flex;
    flex-direction: column;
    gap: 8px;

    label {
      color: #ccc;
      font-size: 0.9rem;
      font-family: "Lato", sans-serif;
    }

    input {
      background-color: $main-background;
      border: 1px solid color.adjust($header-background, $lightness: 15%);
      border-radius: 6px;
      padding: 10px 12px;
      color: white;
      font-size: 1rem;
      font-family: "Lato", sans-serif;
      outline: none;

      &:focus {
        border-color: #64b5f6;
      }
    }
  }
}

.modal-footer {
  padding: 15px 20px;
  display: flex;
  justify-content: flex-end;
  background-color: color.adjust($header-background, $lightness: -2%);
  border-top: 1px solid color.adjust($header-background, $lightness: 5%);

  .save-btn {
    background-color: #4caf50;
    color: white;
    border: none;
    padding: 10px 20px;
    border-radius: 6px;
    font-weight: bold;
    font-size: 1rem;
    cursor: pointer;
    font-family: "Lato", sans-serif;
    transition: background-color 0.2s;

    &:hover {
      background-color: #43a047;
    }
  }
}
</style>
