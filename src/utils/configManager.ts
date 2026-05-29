import { invoke } from "@tauri-apps/api/core";

export interface LauncherOptions {
  game_memory: number;
  username: string;
}

/**
 * Carrega as configurações atuais do arquivo JSON através do Rust.
 * Retorna um objeto com as opções ou valores padrão em caso de erro.
 */
export async function loadLauncherConfigs(): Promise<LauncherOptions> {
  try {
    const configs = await invoke<LauncherOptions>("load_launcher_configs");
    return configs;
  } catch (error) {
    console.error("Erro ao carregar configurações do Rust:", error);
    return {
      game_memory: 5120,
      username: "Steve",
    };
  }
}

/**
 * Salva as configurações completas no arquivo JSON através do Rust.
 * Aceita atualizações parciais combinando com os valores antigos para não perder dados.
 */
export async function saveLauncherConfigs(
  options: Partial<LauncherOptions>,
): Promise<boolean> {
  try {
    // 1. Primeiro, busca o estado atual para garantir que não vamos apagar o outro campo
    const currentConfigs = await loadLauncherConfigs();

    // 2. Mescla o que já existia com a nova alteração (ex: mantém o username se só alterou a memória)
    const mergedOptions: LauncherOptions = {
      ...currentConfigs,
      ...options,
    };

    // 3. Envia o objeto completo estruturado para o Rust
    await invoke("save_launcher_configs", {
      options: mergedOptions,
    });

    return true;
  } catch (error) {
    console.error("Erro ao salvar configurações no Rust:", error);
    return false;
  }
}
