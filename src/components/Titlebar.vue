<script setup lang="ts">
import { getCurrentWindow } from "@tauri-apps/api/window";
import CloseIcon from "../assets/ui_icons/CloseIcon.vue";
import MinimizeIcon from "../assets/ui_icons/MinimizeIcon.vue";

const appWindow = getCurrentWindow();
const minimizeWindow = () => appWindow.minimize();
const closeWindow = () => appWindow.close();
</script>

<template>
  <header class="app-titlebar">
    <div class="header-content" data-tauri-drag-region>
      <div class="title-area">
        <span>Jorges Launcher MC</span>
      </div>

      <nav class="control-window">
        <!-- $ ───▶ Minimizar Janela ◀────────────────────────────────────── -->
        <button class="generic-btn" @click="minimizeWindow" title="Minimizar">
          <MinimizeIcon />
        </button>

        <!-- $ ───▶ Fechar Janela ◀────────────────────────────────────── -->
        <button class="close-btn" @click="closeWindow" title="Fechar">
          <CloseIcon />
        </button>
      </nav>
    </div>
  </header>
  <div class="titlebar-separator"></div>
</template>

<style lang="scss" scoped>
.app-titlebar {
  width: 100%;
  height: 40px;
  flex-shrink: 0;
  background-color: $header-background;

  > .header-content {
    display: flex;
    flex-direction: row;
    align-items: end;
    justify-content: space-between;
    width: 100%;
    height: 100%;

    .title-area {
      width: 100%;
      height: 100%;
      display: flex;
      justify-content: start;
      align-items: center;
      color: rgb(87, 87, 87);
      font-family: "Segoe UI", Tahoma, Geneva, Verdana, sans-serif;
      padding: 0 0 0 10px;
      font-weight: 500;
      pointer-events: none;
    }

    > .control-window {
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
          width: 20px;
          height: 20px;
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

.titlebar-separator {
  outline: none;
  border: none;
  width: 100%;
  height: 2px;
  background-color: color.adjust($header-background, $lightness: -5%);
}
</style>
