pub mod core;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|_app| {
            //% Criar diretório padrão
            core::get_root_dir().map_err(|e| format!("Erro no diretório: {}", e))?;

            //% Criar configurações padrão caso não existam
            core::create_configs().map_err(|e| format!("Erro ao criar cfgs: {}", e))?;

            //% Carregar configurações existentes
            core::load_configs().map_err(|e| format!("Erro ao carregar cfgs: {}", e))?;

            //% Retornar OK ao final de tudo
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            core::tauri_commands::load_launcher_configs,
            core::tauri_commands::open_instance_dir,
            core::tauri_commands::save_launcher_configs,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
