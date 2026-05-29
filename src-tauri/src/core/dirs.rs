//# ───▶ Importando Directories
use directories::UserDirs;

//# ───▶ Importando STD
use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;

//& ───▶ Métodos privados ◀──────────────────────────────────────
//$ Obter o diretório da raiz do aplicativo
/// Obter diretório raiz do aplicativo
fn get_app_root_dir() -> io::Result<PathBuf> {
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

//$ Obtém ou cria um diretório dentro da raiz do aplicativo
/// Obter ou criar diretórios na raiz do aplicativo
fn get_or_create_dir<P: AsRef<Path>>(sub_path: P) -> io::Result<PathBuf> {
    //% Obter diretório "JorgesLMC" do computador
    let app_dir = get_app_root_dir()?;

    //% Adicionar subdiretório dentro do diretório do app
    let sub_dir = app_dir.join(sub_path);

    //% Criar ou obter o diretório
    fs::create_dir_all(&sub_dir)?;

    //% Retornar o diretório
    Ok(sub_dir)
}

//& ───▶ Métodos públicos ◀──────────────────────────────────────
//$ Pega o diretório padrão do aplicativo "JorgesLMC"
/// Obter diretório padrão do aplicativo "JorgesLMC"
///
/// Retorna o caminho completo (`PathBuf`) do diretório.
///
/// # Erros
///
/// Retorna um `io::Error` se não for possível determinar a pasta usuários ou documentos do sistema
/// ou se falhar ao criar o diretório no sistema de arquivos.
pub fn get_root_dir() -> io::Result<PathBuf> {
    get_app_root_dir()
}

//$ Pega o diretório "INSTANCE" do aplicativo
/// Obter diretório "instance" na raiz do aplicativo
///
/// Retorna o caminho completo (`PathBuf`) do diretório.
///
/// # Erros
///
/// Retorna um `io::Error` se falhar ao criar o diretório no sistema de arquivos.
pub fn get_instance_dir() -> io::Result<PathBuf> {
    get_or_create_dir("instance")
}

//$ Pega o diretório "BIN" do aplicativo
/// Obter diretório "bin" na raiz do aplicativo
///
/// Retorna o caminho completo (`PathBuf`) do diretório.
///
/// # Erros
///
/// Retorna um `io::Error` se falhar ao criar o diretório no sistema de arquivos.
pub fn get_bin_dir() -> io::Result<PathBuf> {
    get_or_create_dir("bin")
}
