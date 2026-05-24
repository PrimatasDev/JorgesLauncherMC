use serde::Deserialize;
use std::collections::HashMap;

//% Estrutura para representar o índice do Modrinth
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModrinthIndex {
    pub game: String,
    pub format_version: String,
    pub version_id: String,
    pub name: String,
    pub files: Vec<ModrinthFile>,
    pub dependencies: HashMap<String, String>,
}

//% Estrutura para representar um arquivo do Modrinth
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModrinthFile {
    pub path: String,
    pub hashes: HashMap<String, String>,
    pub downloads: Vec<String>,
    pub file_size: u64,
}

//% Ler arquivos JSON do Modrinth e retornar um ModrinthIndex
pub fn read_modrinth_index(json_content: &str) -> serde_json::Result<ModrinthIndex> {
    let index: ModrinthIndex = serde_json::from_str(json_content)?;
    Ok(index)
}
