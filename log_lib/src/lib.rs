use chrono::Utc;
use serde_json::json;
use simply_colored::*;
use std::{
    path::PathBuf,
    sync::{
        Mutex,
        atomic::{AtomicI32, Ordering},
    },
};

use crate::level::LogLevel;

mod level;

static LOG_LEVEL: AtomicI32 = AtomicI32::new(LogLevel::Info as i32);
static PROJECT_NAME: Mutex<Option<String>> = Mutex::new(None);

#[unsafe(no_mangle)]
unsafe extern "C" fn setup_logging(level: i32, project_name: *const u8) {
    let project_name =
        unsafe { std::ffi::CStr::from_ptr(project_name as *const i8).to_string_lossy() };
    LOG_LEVEL.store(level, Ordering::Relaxed);
    *PROJECT_NAME.lock().unwrap() = Some(project_name.to_string());
}

#[unsafe(no_mangle)]
unsafe extern "C" fn backend_log_callback(
    level: i32,
    filename: *const u8,
    lineno: i32,
    message: *const u8,
) {
    if level < LOG_LEVEL.load(Ordering::Relaxed) {
        return;
    }

    let now = Utc::now();
    let timestamp = now.to_rfc3339_opts(chrono::SecondsFormat::Secs, true);

    let project_name = PROJECT_NAME
        .lock()
        .unwrap()
        .as_deref()
        .unwrap_or("")
        .to_string();
    let file_path = unsafe { std::ffi::CStr::from_ptr(filename as *const i8).to_string_lossy() };
    let filename = PathBuf::from(file_path.to_string())
        .file_name()
        .map(|f| f.to_string_lossy().to_string())
        .unwrap_or_default();
    let message = unsafe { std::ffi::CStr::from_ptr(message as *const i8).to_string_lossy() };
    let level = LogLevel::from(level);

    println!("{DIM_WHITE}{timestamp} {level:?} {filename}:{lineno} {message}");

    let v = json!({
        "timestamp": timestamp,
        "level": level.to_string(),
        "project_name": project_name,
        "file": file_path,
        "lineno": lineno,
        "message": message,
    });
    println!("{}", v.to_string());
}
