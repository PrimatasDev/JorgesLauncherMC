pub mod core;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|_app| {
            //% Criar diretório padrão do app
            if let Err(e) = core::get_app_root_dir() {
                eprintln!("Erro ao inicializar diretório padrão. \n ERRO: {}", e);
            }
            
            //% Carregar configurações
            if let Err(e) = core::load_configs() {
                eprintln!("Erro ao carregar configurações padrão do launcher. \n ERRO: {}", e);
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            core::tauri_commands::open_instance_folder,
            core::tauri_commands::save_launcher_configs,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
