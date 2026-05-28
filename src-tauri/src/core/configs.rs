//# ───▶ Importações STD
use std::fs::File;
use std::io;

//# ───▶ Importações serde
use serde::{Deserialize, Serialize};

//# ───▶ Importações JorgesLauncherMC
use super::get_app_root_dir;


#[derive(Debug, Serialize, Deserialize)]
pub struct LaunhcerOptions {
    pub game_memory: u16,
}

//$ ───▶ Salvar configurações ◀──────────────────────────────────────
pub fn save_configs(options: &LaunhcerOptions) -> io::Result<()> {
    //% Obter pasta do APP
    let app_dir = get_app_root_dir()?;

    //% Criar arquivo de configurações
    let config_file = app_dir.join("cfg.json");
    let file = File::create(&config_file)?;

    //% Escrever um JSON dentro do arquivo de configurações
    serde_json::to_writer_pretty(file, options)?;

    Ok(())
}

//$ ───▶ Carregar configurações ◀──────────────────────────────────────
pub fn load_configs() -> io::Result<LaunhcerOptions> {
    //% Obter pasta do APP
    let app_dir = get_app_root_dir()?;

    //% Criar arquivo de configurações
    let config_file = app_dir.join("cfg.json");
    let file = File::open(&config_file)?;

    //% Ler o JSON dentro do arquivo de configurações
    let options: LaunhcerOptions = serde_json::from_reader(file)?;

    Ok(options)
}