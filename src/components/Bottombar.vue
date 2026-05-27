<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";

import PlayIcon from "../assets/ui_icons/PlayIcon.vue";
import CogIcon from "../assets/ui_icons/CogIcon.vue";
import UserIcon from "../assets/ui_icons/UserIcon.vue";
import FolderIcon from "../assets/ui_icons/FolderIcon.vue";

const emit = defineEmits<{
  (e: "open-settings"): void;
  (e: "open-accounts"): void;
}>();

async function openInstanceFolder() {
  try {
    await invoke("open_instance_folder");
    console.log("Pasta instance aberta com sucesso!");
  } catch (error) {
    console.log(`Ocorreu um erro: ${error}`);
  }
}
</script>

<template>
  <div class="bottom-separator"></div>

  <div class="app-titlebar">
    <div class="bottom-content">
      <nav class="account-actions">
        <button
          class="account-btn"
          @click="emit('open-accounts')"
          title="Gerenciar Contas"
        >
          <div class="account-icon-area">
            <UserIcon />
          </div>
          <div class="account-name-area">
            <span>User</span>
          </div>
        </button>
      </nav>

      <nav class="controls">
        <!-- $ ───▶ Jogar ◀────────────────────────────────────── -->
        <button class="generic-btn" title="Configurações">
          <PlayIcon />
        </button>

        <!-- $ ───▶ Open Launcher Folder ◀────────────────────────────────────── -->
        <button
          class="generic-btn"
          title="Configurações"
          @click="openInstanceFolder"
        >
          <FolderIcon />
        </button>

        <!-- $ ───▶ Configurações ◀────────────────────────────────────── -->
        <button
          class="generic-btn"
          @click="emit('open-settings')"
          title="Configurações"
        >
          <CogIcon />
        </button>
      </nav>
    </div>
  </div>
</template>

<style lang="scss" scoped>
.app-titlebar {
  width: 100%;
  height: 40px;
  flex-shrink: 0;
  background-color: $header-background;

  > .bottom-content {
    display: flex;
    flex-direction: row;
    align-items: end;
    justify-content: space-between;
    width: 100%;
    height: 100%;

    > .account-actions {
      display: flex;
      flex-direction: row;
      justify-content: end;
      align-items: center;
      pointer-events: none;
      height: 100%;

      > button {
        width: 150px;
        height: 40px;
        pointer-events: auto;
        background-color: transparent;
        border: none;
        cursor: pointer;
        display: flex;
        align-items: center;
        justify-content: start;
        overflow: hidden;

        > .account-icon-area {
          width: 40px;
          height: 40px;
          display: flex;
          align-items: center;
          justify-content: center;
          border-radius: 50%;
        }

        > .account-name-area {
          font-size: 1rem;
          color: white;
          display: flex;
          align-items: center;
          justify-content: start;
          width: auto;
          height: 40px;
        }
      }

      > .account-btn {
        > .account-icon-area {
          width: 40px;
          height: 40px;

          > svg {
            fill: color.adjust(white, $lightness: -20%);
            width: 20px;
            height: 20px;
          }
        }

        &:hover {
          background-color: $btn-hover-background;
        }

        &:active {
          background-color: color.adjust($btn-hover-background, $lightness: 5%);
        }

        &:hover,
        &:active {
          > .account-icon-area {
            > svg {
              fill: color.adjust(white, $lightness: -20%);
            }
          }
        }
      }
    }

    > .controls {
      display: flex;
      flex-direction: row;
      justify-content: end;
      align-items: center;
      pointer-events: none;
      height: 100%;

      > button {
        width: 40px;
        height: 40px;
        pointer-events: auto;
        background-color: transparent;
        border: none;
        cursor: pointer;
        display: flex;
        align-items: center;
        justify-content: center;
        overflow: hidden;
      }

      > .generic-btn {
        > svg {
          fill: color.adjust(white, $lightness: -20%);
          width: 20px;
          height: 20px;
        }

        &:hover {
          background-color: $btn-hover-background;
        }

        &:active {
          background-color: color.adjust($btn-hover-background, $lightness: 5%);
        }

        &:hover,
        &:active {
          > svg {
            fill: color.adjust(white, $lightness: -20%);
          }
        }
      }

      > .close-btn {
        > svg {
          fill: color.adjust(white, $lightness: -20%);
          width: 16px;
          height: 16px;
        }

        &:hover {
          background-color: $close-btn-background;
        }

        &:active {
          background-color: color.adjust(
            $close-btn-background,
            $lightness: -10%
          );
        }

        &:hover,
        &:active {
          > svg {
            fill: color.adjust(white, $lightness: -20%);
          }
        }
      }
    }
  }
}

.bottom-separator {
  outline: none;
  border: none;
  width: 100%;
  height: 2px;
  background-color: color.adjust($header-background, $lightness: -5%);
}
</style>
