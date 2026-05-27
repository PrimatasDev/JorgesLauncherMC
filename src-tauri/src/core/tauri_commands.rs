use super::get_app_dir;

//$ ───▶ Abrir diretório ao APP ◀──────────────────────────────────────
#[tauri::command]
pub fn open_instance_folder() -> Result<(), String> {
    let instance_folder = get_app_dir("instance").map_err(|e| e.to_string())?;

    //% Se a pasta não existir, cria
    if !instance_folder.exists() {
        std::fs::create_dir_all(&instance_folder).map_err(|e| e.to_string())?;
    }

    //% Tenta abrir o gerenciador de arquivos
    open::that(&instance_folder).map_err(|e| {
        format!("Falha ao abrir o gerenciador de arquivos: {}", e)
    })?;

    Ok(())
}