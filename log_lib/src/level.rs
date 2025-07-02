use simply_colored::*;

pub enum LogLevel {
    Debug = 0,
    Info = 1,
    Warn = 2,
    Error = 3,
    Unknown,
}

impl LogLevel {
    const LOG_LEVEL_DEBUG: i32 = 0;
    const LOG_LEVEL_INFO: i32 = 1;
    const LOG_LEVEL_WARN: i32 = 2;
    const LOG_LEVEL_ERROR: i32 = 3;
}

impl From<i32> for LogLevel {
    fn from(level: i32) -> Self {
        match level {
            Self::LOG_LEVEL_DEBUG => LogLevel::Debug,
            Self::LOG_LEVEL_INFO => LogLevel::Info,
            Self::LOG_LEVEL_WARN => LogLevel::Warn,
            Self::LOG_LEVEL_ERROR => LogLevel::Error,
            _ => LogLevel::Unknown,
        }
    }
}

impl std::fmt::Debug for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogLevel::Debug => write!(f, "{BOLD}{GREEN}{REVERSE}[debug]{RESET}"),
            LogLevel::Info => write!(f, "{BOLD}{BLUE}{REVERSE}[info ]{RESET}"),
            LogLevel::Warn => write!(f, "{BOLD}{YELLOW}{REVERSE}[warn ]{RESET}"),
            LogLevel::Error => write!(f, "{BOLD}{RED}{REVERSE}[error]{RESET}"),
            LogLevel::Unknown => write!(f, "{BOLD}{RED}{REVERSE}[unknown]{RESET}"),
        }
    }
}

impl std::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                LogLevel::Debug => "debug",
                LogLevel::Info => "info",
                LogLevel::Warn => "warn",
                LogLevel::Error => "error",
                LogLevel::Unknown => "unknown",
            }
        )
    }
}
