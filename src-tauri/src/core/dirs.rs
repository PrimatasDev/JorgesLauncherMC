//# ───▶ Importando Directories
use directories::UserDirs;

//# ───▶ Importando STD
use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;

//$ ───▶ Obter pasta raiz do launcher ◀──────────────────────────────────────
/// Obter pasta raiz do APP
pub fn get_app_root_dir() -> io::Result<PathBuf> {
    //% Obter o diretório "USER" do computador
    let user_dir = UserDirs::new().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::NotFound,
            "Não foi possível encontrar o diretório do usuário",
        )
    })?;

    //% Obter diretório "DOCUMENTS" do computador
    let document_dir = user_dir.document_dir().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::NotFound,
            "Não foi possível encontrar o diretório de documentos",
        )
    })?;

    //% Obter e criar pasta root do APP
    let root_dir = document_dir.join("JorgesLMC");
    fs::create_dir_all(&root_dir)?;

    Ok(root_dir)
}

//$ ───▶ Criar diretórios para o APP ◀──────────────────────────────────────
/// Criar ou obter um diretório dentro da pasta raiz do jogo
pub fn get_or_create_dir<P: AsRef<Path>>(sub_path: P) -> io::Result<PathBuf> {
    //% Obter pasta do APP
    let app_dir = get_app_root_dir()?;

    //% Obter sub pasta dentro da pasta do aplicativo
    let sub_dir = app_dir.join(sub_path);

    //% Se não existir, crie
    fs::create_dir_all(&sub_dir)?;

    Ok(sub_dir)
}
