
//# ───▶ Importações do JorgesLauncherMC
use super::{get_or_create_dir, load_configs, save_configs, LaunhcerOptions};


//$ ───▶ Abrir diretório ao APP ◀──────────────────────────────────────
#[tauri::command]
pub fn open_instance_folder() -> Result<(), String> {
    let instance_folder = get_or_create_dir("instance").map_err(|e| e.to_string())?;

    //% Tenta abrir o gerenciador de arquivos
    open::that(&instance_folder).map_err(|e| {
        format!("Falha ao abrir o gerenciador de arquivos: {}", e)
    })?;

    Ok(())
}

//$ ───▶ Salvar configurações ◀──────────────────────────────────────
#[tauri::command]
pub fn save_launcher_configs(options: LaunhcerOptions) -> Result<(), String> {
    save_configs(&options).map_err(|e| {
        format!("Falha ao salvar o arquivo. \n ERRO: {}", e)
    })?;
    
    Ok(())
}

//$ ───▶ Carregar configurações ◀──────────────────────────────────────
#[tauri::command]
pub fn load_launcher_configs() -> Result<LaunhcerOptions, String> {
    let options = load_configs().map_err(|e| {
        format!("Falha ao carregar o arquivo. \n ERRO: {}", e)
    })?;
    
    Ok(options)
}