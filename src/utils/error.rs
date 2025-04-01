use age::DecryptError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("Failed to save entries")]
    SaveError,

    #[error("Failed to parse entries")]
    ParseError,

    #[error("Failed to read file")]
    ReadError,

    #[error("Failed to write file")]
    WriteError,

    #[error("Failed to serialize entries")]
    SerializeError,

    #[error("Invalid key: {0}")]
    InvalidKey(String),

    #[error("Failed to create encryptor")]
    EncryptorError,

    #[error("Failed to copy to clipboard")]
    ClipboardError,

    #[error("Failed to generate TOTP code")]
    TotpError,

    #[error("Failed to decrypt entries")]
    DecryptError,

    #[error("Failed to decode entries as UTF-8")]
    Utf8Error,

    #[error("Could not find home directory")]
    HomeDirError,

    #[error("Failed to create auth directory")]
    CreateDirError,

    #[error("Failed to initialize encryption")]
    CryptoInitError,

    #[error("Empty entries are not allowed")]
    EmptyEntryError,

    #[error("Path points to a directory")]
    DirectoryError,

    #[error("File does not exist")]
    FileNotExistError,

    #[error("No file name provided")]
    NoFilenameError,

    #[error("No entries to export")]
    EmptyExportError,

    #[error("File must have .toml extension")]
    TomlExtError,

    #[error("Failed to spawn clipboard process")]
    ClipboardSpawnError,

    #[error("Failed to write to clipboard process")]
    ClipboardWriteError,

    #[error("Failed to wait for clipboard process")]
    ClipboardWaitError,

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("TOML serialization error: {0}")]
    TomlError(#[from] toml::ser::Error),

    #[error("TOML deserialization error: {0}")]
    TomlDeError(#[from] toml::de::Error),

    #[error("UTF-8 conversion error: {0}")]
    FromUtf8Error(#[from] std::string::FromUtf8Error),

    #[error("Decryption error: {0}")]
    AgeDecryptError(#[from] DecryptError),
}

pub type AuthResult<T> = Result<T, AuthError>;
