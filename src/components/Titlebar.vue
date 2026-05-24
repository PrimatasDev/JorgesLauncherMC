<script setup lang="ts">
import { getCurrentWindow } from "@tauri-apps/api/window";
import CloseWindowIcon from "../assets/Icons/CloseWindowIcon.vue";
import MinimizeWindowIcon from "../assets/Icons/MinimizeWindowIcon.vue";

const appWindow = getCurrentWindow();
const minimizeWindow = () => appWindow.minimize();
const closeWindow = () => appWindow.close();
</script>

<template>
  <header class="app-titlebar">
    <div class="header-content" data-tauri-drag-region>
      <nav class="account-actions">
        <button class="account-btn">
          <div class="account-icon-area">
            <CloseWindowIcon />
          </div>
          <div class="account-name-area">
            <span>User</span>
          </div>
        </button>
      </nav>

      <nav class="control-window">
        <button class="minimize-btn" @click="minimizeWindow">
          <MinimizeWindowIcon />
        </button>

        <button class="close-btn" @click="closeWindow">
          <CloseWindowIcon />
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
          font-size: 1.1rem;
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
            width: 16px;
            height: 18px;
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

      > .minimize-btn {
        > svg {
          fill: color.adjust(white, $lightness: -20%);
          width: 16px;
          height: 18px;
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
          width: 14px;
          height: 14px;
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
