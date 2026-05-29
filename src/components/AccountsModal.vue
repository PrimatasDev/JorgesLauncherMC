<script setup lang="ts">
import { ref, onMounted, watch } from "vue";
import CloseIcon from "../assets/ui_icons/CloseIcon.vue";
import { validateUsername, sanitizeUsernameInput } from "../utils/validators";
// Importa o gerenciador unificado
import {
  loadLauncherConfigs,
  saveLauncherConfigs,
} from "../utils/configManager";

const props = defineProps<{ isOpen: boolean }>();
const emit = defineEmits<{ (e: "close"): void }>();

const username = ref("");

//% Busca apenas o que interessa para este modal
async function loadSavedUsername() {
  const configs = await loadLauncherConfigs();
  username.value = configs.username;
}

function handleInput(event: Event) {
  const target = event.target as HTMLInputElement;
  username.value = sanitizeUsernameInput(target.value);
}

//% Salva o nome de usuário isoladamente de forma segura
async function saveUsername() {
  if (!validateUsername(username.value)) {
    alert(
      "O nome de usuário deve ter entre 3 e 16 caracteres e conter apenas letras, números e '_'!",
    );
    return;
  }

  // O configManager vai cuidar de manter a memória atual intacta lá dentro
  const success = await saveLauncherConfigs({ username: username.value });

  if (success) {
    console.log(`Usuário definido com sucesso: ${username.value}`);
    emit("close");
  }
}

watch(
  () => props.isOpen,
  (isOpenNow) => {
    if (isOpenNow) loadSavedUsername();
  },
);
onMounted(() => {
  if (props.isOpen) loadSavedUsername();
});
</script>

<template>
  <div v-if="isOpen" class="modal-overlay" @click.self="emit('close')">
    <div class="modal-content">
      <div class="modal-header" data-tauri-drag-region>
        <h2>Entrar como</h2>
        <button class="close-btn" @click="emit('close')">
          <CloseIcon />
        </button>
      </div>
      <div class="modal-body">
        <!-- % Área de conta -->
        <div class="body-section">
          <!-- ! ADICIONAR CONTA DESCARTADO (POR ENQUANTO) -->
          <!-- @ <Button class="add-account-btn">Adicionar Conta</Button> -->
          <!-- <br /> -->
          <div class="section-body">
            <input
              class="username-input"
              type="text"
              placeholder="Nome de usuário"
              maxlength="16"
            />
            <Button class="define-name-btn">Definir</Button>
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

    .section-body {
      width: 100%;
      display: flex;
      flex-direction: column;
      gap: 10px;
      justify-content: center;
      align-items: center;

      > .username-input {
        width: 100%;
        height: 50px;
        background-color: transparent;
        border: 1px solid color.adjust($header-background, $lightness: 30%);
        color: white;
        padding: 12px;
        border-radius: 8px;
        cursor: text;
        font-family: "Lato", sans-serif;
        transition: all 0.2s;
        font-size: 1rem;

        &:hover {
          background-color: color.adjust($main-background, $lightness: 5%);
          border-color: white;
        }

        &:focus {
          background-color: color.adjust($main-background, $lightness: 10%);
        }
      }

      > .define-name-btn {
        width: 100%;
        height: 50px;
        background-color: transparent;
        border: 1px solid color.adjust($header-background, $lightness: 30%);
        color: white;
        padding: 12px;
        border-radius: 8px;
        cursor: pointer;
        font-family: "Lato", sans-serif;
        transition: all 0.2s;
        font-size: 1rem;

        &:hover {
          background-color: color.adjust($main-background, $lightness: 5%);
          border-color: white;
        }

        &:active {
          background-color: color.adjust($main-background, $lightness: 10%);
        }
      }
    }
  }
}
</style>
