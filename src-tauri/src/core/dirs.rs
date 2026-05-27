use directories::UserDirs;
use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;

//$ ───▶ Obter diretórios do APP ◀──────────────────────────────────────
pub fn get_app_dir<P: AsRef<Path>>(sub_path: P) -> io::Result<PathBuf> {
    //% Obter diretório do usuário
    let user_dir = UserDirs::new().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::NotFound,
            "Não foi possível encontrar o diretório do usuário",
        )
    })?;

    //% Obter diretório de documentos
    let document_dir = user_dir.document_dir().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::NotFound,
            "Não foi possível encontrar o diretório de documentos",
        )
    })?;

    //% Obter pasta do aplicativo
    let base_dir = document_dir.join("JorgesLauncherMC");

    //% Obter sub pasta dentro da pasta do aplicativo
    let final_dir = base_dir.join(sub_path);

    //% Se não existir, crie
    if let Some(parent) = final_dir.parent() {
        fs::create_dir_all(parent)?;
    }

    Ok(final_dir)
}