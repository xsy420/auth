pub const MIN_WIDTH: u16 = 110;
pub const MIN_HEIGHT: u16 = 31;
pub const TOTP_PERIOD: u64 = 30;
pub const TOTP_DIGITS: usize = 6;
pub const TOTP_STEP: u8 = 1;
pub const AUTH_DIR_NAME: &str = ".local/share/auth";
pub const ENTRIES_FILE: &str = "entries.toml";
pub const KEY_FILE: &str = "key";
pub const WARNING_TITLE: &str = " Warning ";
pub const AUTH_TITLE: &str = " Auth ";
pub const COPIED_MSG: &str = " Copied! ";
pub const ADD_ENTRY_TITLE: &str = " Add Entry ";
pub const EDIT_ENTRY_TITLE: &str = " Edit Entry ";
pub const IMPORT_TITLE: &str = " Import ";
pub const EXPORT_TITLE: &str = " Export ";
pub const BINDINGS_TITLE: &str = " Bindings ";
pub const NAME_LABEL: &str = "Name:";
pub const SECRET_LABEL: &str = "Secret:";
pub const PATH_LABEL: &str = "Path:";
pub const INVALID_LABEL: &str = "Invalid";
pub const INVALID_PATH_ERROR: &str = "Invalid path";
pub const TOML_EXT_ERROR: &str = "File must have .toml extension";
pub const SAVE_ERROR: &str = "Failed to save entries";
pub const PARSE_ERROR: &str = "Failed to parse entries";
pub const READ_ERROR: &str = "Failed to read file";
pub const WRITE_ERROR: &str = "Failed to write file";
pub const SERIALIZE_ERROR: &str = "Failed to serialize entries";
pub const INVALID_KEY_ERROR: &str = "Invalid key";
pub const ENCRYPTOR_ERROR: &str = "Failed to create encryptor";
pub const CLIPBOARD_ERROR: &str = "Failed to copy to clipboard";
pub const TOTP_ERROR: &str = "Failed to generate TOTP code";
pub const DECRYPT_ERROR: &str = "Failed to decrypt entries";
pub const UTF8_ERROR: &str = "Failed to decode entries as UTF-8";
pub const HOME_DIR_ERROR: &str = "Could not find home directory";
pub const CREATE_DIR_ERROR: &str = "Failed to create auth directory";
pub const CRYPTO_INIT_ERROR: &str = "Failed to initialize encryption";
pub const HOME_DIR_EXPAND_ERROR: &str = "Could not expand home directory";
pub const EMPTY_ENTRY_ERROR: &str = "Empty entries are not allowed";
pub const DIRECTORY_ERROR: &str = "Path points to a directory";
pub const FILE_NOT_EXIST_ERROR: &str = "File does not exist";
pub const NO_FILENAME_ERROR: &str = "No file name provided";
pub const EMPTY_EXPORT_ERROR: &str = "No entries to export";
pub const NOTIFICATION_TIMEOUT: u64 = 3;
pub const CLIPBOARD_SLEEP_DURATION: u64 = 100;
pub const EVENT_POLL_DURATION: u64 = 50;
pub const VERTICAL_PADDING_DIVISOR: u16 = 2;
pub const POPUP_WIDTH_PERCENT: u16 = 60;
pub const POPUP_HEIGHT_PERCENT: u16 = 20;
pub const TOTAL_PERCENTAGE: u16 = 100;
pub const SECRET_PADDING_BLOCK: usize = 8;
pub const SECRET_PAD_CHAR: char = '=';
pub const MIN_SECRET_LENGTH: usize = 16;
pub const FIRST_ENTRY_ROW: usize = 1;
pub const NAME_FIELD: usize = 0;
pub const SECRET_FIELD: usize = 1;
pub const HOME_PREFIX_LEN: usize = 2;
pub const LAST_ENTRY_INDEX: usize = 1;
pub const ENV_VAR_OFFSET: usize = 1;
pub const SINGLE_CHAR_PATH: usize = 1;
pub const PATH_SEPARATOR_OFFSET: usize = 1;
pub const LAST_ENTRY_OFFSET: usize = 1;
pub const NEXT_ENTRY_STEP: usize = 1;
pub const INVALID_REMAINING_TIME: u64 = 0;
pub const PADDING_BYTE: u8 = 0;
pub const REMAINDER_ZERO: usize = 0;
pub const DEFAULT_NAME_WIDTH: usize = 0;
pub const MIN_BLOCK_HEIGHT: u16 = 3;
pub const HELP_BLOCK_HEIGHT: u16 = 3;
pub const CODE_WIDTH: usize = 6;
pub const REMAINING_WIDTH: usize = 1;
pub const NAME_PADDING: usize = 2;
pub const CURSOR_CHAR: &str = "|";
pub const EMPTY_CURSOR: &str = "";

pub const ROOT_WARNING: &[&str] = &[
    "Running as root is not supported",
    "",
    "Press any key to exit",
];

pub const SIZE_WARNING: &[&str] = &[
    "Terminal size too small:",
    "Width = {} Height = {}",
    "",
    "Needed to display properly:",
    "Width = {} Height = {}",
];

pub const HELP_TEXT: &str =
    "a: add  E: edit  d: del  i: import  e: export  ↑/k: up  ↓/j: down  enter: copy  q: quit  tab: cycle fields";
