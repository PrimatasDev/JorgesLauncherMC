<script setup lang="ts">
import TrashIcon from "../assets/ui_icons/TrashIcon.vue";
import MenuDotsIcon from "../assets/ui_icons/MenuDotsIcon.vue";

defineProps<{
  name: string;
  version: string;
  gameVersion: string;
  loader: string;
  loaderVersion: string;
  iconUrl?: string;
}>();

const emit = defineEmits<{
  (e: 'delete'): void;
  (e: 'options'): void;
}>();
</script>

<template>
  <div class="modpack-card">
    <div class="card-left">
      <div class="modpack-icon">
        <img v-if="iconUrl" :src="iconUrl" alt="Modpack Icon" />
        <div v-else class="icon-placeholder"></div>
      </div>
      <div class="modpack-info">
        <span class="modpack-name">{{ name }}</span>
        <span class="modpack-version">{{ version }}</span>
      </div>
    </div>

    <div class="card-right">
      <div class="game-info">
        <span class="game-version">Minecraft: {{ gameVersion }}</span>
        <span class="loader-version">{{ loader }}: {{ loaderVersion }}</span>
      </div>
      
      <div class="card-actions">
        <button class="action-btn delete-btn" @click="emit('delete')" title="Excluir">
          <TrashIcon />
        </button>
        <button class="action-btn menu-btn" @click="emit('options')" title="Opções">
          <MenuDotsIcon />
        </button>
      </div>
    </div>
  </div>
</template>

<style lang="scss" scoped>
.modpack-card {
  display: flex;
  flex-direction: row;
  align-items: center;
  justify-content: space-between;
  width: 100%;
  height: 70px;
  background-color: $header-background;
  border-radius: 8px;
  padding: 10px 15px;
  transition: background-color 0.2s ease, transform 0.2s ease;
  box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);

  &:hover {
    background-color: color.adjust($header-background, $lightness: 5%);
    transform: translateY(-2px);
  }

  .card-left {
    display: flex;
    align-items: center;
    gap: 15px;
    height: 100%;

    .modpack-icon {
      width: 50px;
      height: 50px;
      border-radius: 8px;
      overflow: hidden;
      background-color: color.adjust($header-background, $lightness: 10%);
      flex-shrink: 0;

      img {
        width: 100%;
        height: 100%;
        object-fit: cover;
      }

      .icon-placeholder {
        width: 100%;
        height: 100%;
        /* A placeholder pattern or color can be applied here */
        background: linear-gradient(135deg, #4caf50 0%, #2e7d32 100%);
      }
    }

    .modpack-info {
      display: flex;
      flex-direction: column;
      justify-content: center;

      .modpack-name {
        color: white;
        font-size: 1.1rem;
        font-weight: 700;
        font-family: 'Lato', sans-serif;
      }

      .modpack-version {
        color: #64b5f6; /* Light blueish color for version */
        font-size: 0.85rem;
        font-weight: 400;
        font-family: 'Lato', sans-serif;
        margin-top: 2px;
      }
    }
  }

  .card-right {
    display: flex;
    align-items: center;
    gap: 20px;
    height: 100%;

    .game-info {
      display: flex;
      flex-direction: column;
      justify-content: center;
      text-align: left;
      margin-right: 20px;

      .game-version {
        color: white;
        font-size: 0.9rem;
        font-weight: 700;
        font-family: 'Lato', sans-serif;
      }

      .loader-version {
        color: #64b5f6;
        font-size: 0.85rem;
        font-weight: 400;
        font-family: 'Lato', sans-serif;
        margin-top: 2px;
      }
    }

    .card-actions {
      display: flex;
      align-items: center;
      gap: 10px;

      .action-btn {
        width: 32px;
        height: 32px;
        border: none;
        background-color: transparent;
        border-radius: 4px;
        cursor: pointer;
        display: flex;
        align-items: center;
        justify-content: center;
        transition: background-color 0.2s ease;

        svg {
          width: 20px;
          height: 20px;
          fill: color.adjust(white, $lightness: -30%);
          transition: fill 0.2s ease;
        }

        &:hover {
          background-color: $btn-hover-background;
          svg {
            fill: white;
          }
        }

        &.delete-btn:hover {
          background-color: rgba(200, 50, 50, 0.2);
          svg {
            fill: #ef5350;
          }
        }
      }
    }
  }
}
</style>
