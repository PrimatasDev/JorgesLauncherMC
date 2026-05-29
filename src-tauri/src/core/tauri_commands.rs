//# ───▶ Importações do JorgesLauncherMC
use super::{LaunhcerOptions, get_instance_dir, load_configs, save_configs};

//& ───▶ Abrir diretório ao APP ◀──────────────────────────────────────
//$ Abrir diretório "INSTANCE" do aplicativo no gerenciador de arquivos do sistema
#[tauri::command]
pub fn open_instance_dir() -> Result<(), String> {
    //% Criar pasta "INSTANCE" dentro da pasta raiz do projeto
    let instance_dir = get_instance_dir().map_err(|e| e.to_string())?;

    //% Tenta abrir o gerenciador de arquivos
    open::that(&instance_dir)
        .map_err(|e| format!("Falha ao abrir o gerenciador de arquivos: {}", e))?;

    Ok(())
}

//& ───▶ Salvar configurações ◀──────────────────────────────────────
//$ Salvar configurações do tipo "LaunhcerOptions" em um arquivo CFG_JSON na pasta raiz do app
#[tauri::command]
pub fn save_launcher_configs(options: LaunhcerOptions) -> Result<(), String> {
    save_configs(&options).map_err(|e| format!("Falha ao salvar o arquivo. \n ERRO: {}", e))?;
    Ok(())
}

//& ───▶ Carregar configurações ◀──────────────────────────────────────
//$ Carregar configurações do arquivo CFG_JSON na pasta raiz do app
#[tauri::command]
pub fn load_launcher_configs() -> Result<LaunhcerOptions, String> {
    let options =
        load_configs().map_err(|e| format!("Falha ao carregar o arquivo. \n ERRO: {}", e))?;
    Ok(options)
}
