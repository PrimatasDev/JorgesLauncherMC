<script setup lang="ts">
import CloseIcon from "../assets/ui_icons/CloseIcon.vue";
import UserIcon from "../assets/ui_icons/UserIcon.vue";

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
        <h2>Contas Minecraft</h2>
        <button class="close-btn" @click="emit('close')">
          <CloseIcon />
        </button>
      </div>
      <div class="modal-body">
        <div class="accounts-list">
          <!-- Placeholder for accounts -->
          <div class="account-item active">
            <div class="account-avatar">
              <UserIcon />
            </div>
            <div class="account-details">
              <span class="account-name">Steve (Padrão)</span>
              <span class="account-type">Offline</span>
            </div>
          </div>
        </div>

        <button class="add-account-btn">Adicionar Nova Conta</button>
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
  gap: 20px;

  .accounts-list {
    display: flex;
    flex-direction: column;
    gap: 10px;

    .account-item {
      display: flex;
      align-items: center;
      gap: 15px;
      background-color: $main-background;
      padding: 10px 15px;
      border-radius: 8px;
      border: 1px solid color.adjust($header-background, $lightness: 15%);
      cursor: pointer;
      transition: background-color 0.2s;

      &:hover {
        background-color: color.adjust($main-background, $lightness: 5%);
      }

      &.active {
        border-color: #64b5f6;
      }

      .account-avatar {
        width: 40px;
        height: 40px;
        border-radius: 50%;
        background-color: color.adjust($main-background, $lightness: 10%);
        display: flex;
        align-items: center;
        justify-content: center;

        svg {
          width: 24px;
          height: 24px;
          fill: #ccc;
        }
      }

      .account-details {
        display: flex;
        flex-direction: column;

        .account-name {
          color: white;
          font-weight: bold;
          font-family: "Lato", sans-serif;
        }

        .account-type {
          color: #aaa;
          font-size: 0.85rem;
          font-family: "Lato", sans-serif;
        }
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
