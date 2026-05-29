import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

export interface LauncherOptions {
  game_memory: number;
  username: string;
}

// ESTADO GLOBAL REATIVO
export const currentConfigs = ref<LauncherOptions>({
  game_memory: 5120,
  username: "Steve",
});

/**
 * Carrega as configurações atuais do arquivo JSON através do Rust e atualiza o estado reativo.
 */
export async function loadLauncherConfigs(): Promise<LauncherOptions> {
  try {
    const configs = await invoke<LauncherOptions>("load_launcher_configs");
    currentConfigs.value = configs;
    return configs;
  } catch (error) {
    console.error("Erro ao carregar configurações do Rust:", error);
    return currentConfigs.value;
  }
}

/**
 * Salva as configurações completas no arquivo JSON através do Rust e atualiza o estado reativo.
 */
export async function saveLauncherConfigs(
  options: Partial<LauncherOptions>,
): Promise<boolean> {
  try {
    // Mescla o valor antigo com a nova alteração
    const mergedOptions: LauncherOptions = {
      ...currentConfigs.value,
      ...options,
    };

    // Envia para o Rust salvar no arquivo físico
    await invoke("save_launcher_configs", {
      options: mergedOptions,
    });

    // Atualiza o estado global na memória instantaneamente
    currentConfigs.value = mergedOptions;

    return true;
  } catch (error) {
    console.error("Erro ao salvar configurações no Rust:", error);
    return false;
  }
}
