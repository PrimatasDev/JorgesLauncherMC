pub mod core;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|_app| {
            if let Err(e) = core::get_app_dir("JorgesLauncherMC") {
                eprintln!("Failed to initialize directories: {}", e);
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            core::tauri_commands::open_instance_folder,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
