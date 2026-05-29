//# ───▶ Importações do JorgesLauncherMC
use super::{LaunhcerOptions, get_instance_dir, load_configs, save_configs};

#[tauri::command]
//$ Abrir diretório "INSTANCE" do aplicativo no gerenciador de arquivos do sistema
pub fn open_instance_dir() -> Result<(), String> {
    //% Criar pasta "INSTANCE" dentro da pasta raiz do projeto
    let instance_dir = get_instance_dir().map_err(|e| e.to_string())?;

    //% Tenta abrir o gerenciador de arquivos
    open::that(&instance_dir)
        .map_err(|e| format!("Falha ao abrir o gerenciador de arquivos: {}", e))?;

    Ok(())
}

#[tauri::command]
//$ Salvar configurações do tipo "LaunhcerOptions" em um arquivo CFG_JSON na pasta raiz do app
pub fn save_launcher_configs(options: LaunhcerOptions) -> Result<(), String> {
    save_configs(&options).map_err(|e| format!("Falha ao salvar o arquivo. \n ERRO: {}", e))?;
    Ok(())
}

#[tauri::command]
//$ Carregar configurações do arquivo CFG_JSON na pasta raiz do app
pub fn load_launcher_configs() -> Result<LaunhcerOptions, String> {
    let options =
        load_configs().map_err(|e| format!("Falha ao carregar o arquivo. \n ERRO: {}", e))?;
    Ok(options)
}
