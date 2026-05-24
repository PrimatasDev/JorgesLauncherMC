<script setup lang="ts">
import CloseWindowIcon from "../assets/Icons/CloseWindowIcon.vue";
</script>

<template>
  <div class="footer-separator"></div>
  <footer class="app-footerbar">
    <div class="footer-content" data-tauri-drag-region>
      <nav class="actions-btns">
        <!-- Jogar -->
        <button class="action-btn">
          <div class="btn-content">
            <CloseWindowIcon />
            <span>Jogar</span>
          </div>
        </button>

        <!-- Local -->
        <button class="action-btn">
          <div class="btn-content">
            <CloseWindowIcon />
            <span>Local</span>
          </div>
        </button>

        <!-- Opções -->
        <button class="action-btn">
          <div class="btn-content">
            <CloseWindowIcon />
            <span>Opções</span>
          </div>
        </button>
      </nav>
    </div>
  </footer>
</template>

<style lang="scss" scoped>
.app-footerbar {
  width: 100%;
  height: 40px;
  flex-shrink: 0;
  background-color: $header-background;

  > .footer-content {
    display: flex;
    flex-direction: row;
    align-items: end;
    justify-content: center;
    width: 100%;
    height: 100%;

    > .actions-btns {
      display: flex;
      flex-direction: row;
      justify-content: end;
      align-items: center;
      pointer-events: none;
      height: 100%;

      > button {
        width: 120px;
        height: 40px;
        pointer-events: auto;
        background-color: transparent;
        border: none;
        cursor: pointer;
        display: flex;
        align-items: center;
        justify-content: center;
        overflow: hidden;

        > .btn-content {
          display: flex;
          flex-direction: row;
          align-items: center;
          justify-content: center;
          gap: 10px;
          width: 100%;
          height: 100%;
          color: white;
          font-size: 1.2rem;

          > svg {
            fill: color.adjust(white, $lightness: -20%);
            width: 18px;
            height: 18px;
          }
        }
      }

      > .action-btn {
        > .btn-icon-area {
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
          > .btn-icon-area {
            > svg {
              fill: color.adjust(white, $lightness: -20%);
            }
          }
        }
      }
    }
  }
}

.footer-separator {
  outline: none;
  border: none;
  width: 100%;
  height: 2px;
  background-color: color.adjust($header-background, $lightness: -5%);
}
</style>
