//# ───▶ Importações STD
use std::fs::File;
use std::io;

//# ───▶ Importações serde
use serde::{Deserialize, Serialize};

//# ───▶ Importações JorgesLauncherMC
use super::get_root_dir;

#[derive(Debug, Serialize, Deserialize)]
pub struct LaunhcerOptions {
    pub game_memory: u16,
    pub username: String,
}

const CFG_JSON: &str = "cfgs.json";

//& ───▶ Métodos públicos ◀──────────────────────────────────────

//$ Criar arquivo de configurações padrão caso não exista
/// # Criar arquivo de configurações
/// Criar arquivo de configurações caso não exista
pub fn create_configs() -> Result<(), io::Error> {
    //% Obter pasta do APP
    let app_dir = get_root_dir()?;

    //% Criar arquivo de configurações
    let config_file = app_dir.join(CFG_JSON);

    if config_file.exists() {
        return Ok(());
    }

    let file = File::create(&config_file)?;

    //% Definir configurações padrões de começo
    let opts = LaunhcerOptions {
        game_memory: 5120,
        username: "Steve".into(),
    };

    //% Escrever um JSON dentro do arquivo de configurações
    serde_json::to_writer_pretty(file, &opts)?;

    Ok(())
}

//$ Salvar configuraçoes do tipo "LaunhcerOptions" em um arquivo CFG_JSON na pasta raiz do app
/// # Salvar configurações
/// Função para salvar configurações do tipo "LaunhcerOptions" em um arquivo CFG_JSON na pasta raiz do app
pub fn save_configs(options: &LaunhcerOptions) -> io::Result<()> {
    //% Obter pasta do APP
    let app_dir = get_root_dir()?;

    //% Criar arquivo de configurações
    let config_file = app_dir.join(CFG_JSON);
    let file = File::create(&config_file)?;

    //% Escrever um JSON dentro do arquivo de configurações
    serde_json::to_writer_pretty(file, options)?;

    Ok(())
}

//$ Carregar configurações do arquivo CFG_JSON na pasta raiz do app
/// # Carregar configurações
/// Carrega as configurações do arquivo CFG_JSON na pasta raiz do app
pub fn load_configs() -> io::Result<LaunhcerOptions> {
    //% Obter pasta do APP
    let app_dir = get_root_dir()?;

    //% Criar arquivo de configurações
    let config_file = app_dir.join(CFG_JSON);
    let file = File::open(&config_file)?;

    //% Ler o JSON dentro do arquivo de configurações
    let options: LaunhcerOptions = serde_json::from_reader(file)?;

    Ok(options)
}
