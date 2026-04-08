#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use tauri::Emitter;

#[derive(Clone, serde::Serialize)]
struct ProgressPayload {
    current: u32,
    total: u32,
    file: String,
}

#[derive(serde::Serialize)]
struct ResultMeta {
    filename: String,
    created_at: String,
    result_type: String,
    title: Option<String>,
}

fn sort_audio_paths(paths: &mut Vec<String>) {
    paths.sort_by(|a, b| {
        let extract = |s: &str| -> Option<u64> {
            let name = std::path::Path::new(s)
                .file_stem()?
                .to_str()?
                .to_string();
            if let Some(n) = name.strip_prefix("audio_") {
                n.parse().ok()
            } else {
                None
            }
        };
        match (extract(a), extract(b)) {
            (Some(na), Some(nb)) => na.cmp(&nb),
            _ => a.cmp(b),
        }
    });
}

fn speechext_dir() -> Result<std::path::PathBuf, String> {
    let home = dirs::home_dir().ok_or("Cannot find home directory")?;
    let dir = home.join("Speechext");
    if !dir.exists() {
        std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    }
    Ok(dir)
}

#[tauri::command]
async fn transcribe_files(window: tauri::Window, files: Vec<String>, language: String) -> String {
    let mut sorted_files = files.clone();
    sort_audio_paths(&mut sorted_files);

    let script = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("transcribe.py");

    let mut cmd = Command::new("/Users/alligator/Documents/education/projects/2026/speechext/.venv/bin/python3");
    cmd.arg(script);
    cmd.arg("--lang");
    cmd.arg(&language);
    for f in &sorted_files {
        cmd.arg(f);
    }

    let mut child = match cmd
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
    {
        Ok(c) => c,
        Err(e) => return format!(r#"{{"error": "Failed to start python: {}"}}"#, e),
    };

    let stderr = child.stderr.take().unwrap();
    let window_clone = window.clone();
    std::thread::spawn(move || {
        let reader = BufReader::new(stderr);
        for line in reader.lines().flatten() {
            if let Ok(val) = serde_json::from_str::<serde_json::Value>(&line) {
                let current = val["progress"].as_u64().unwrap_or(0) as u32;
                let total = val["total"].as_u64().unwrap_or(0) as u32;
                let file = val["file"].as_str().unwrap_or("").to_string();
                let _ = window_clone.emit("transcribe-progress", ProgressPayload { current, total, file });
            }
        }
    });

    match child.wait_with_output() {
        Ok(out) => {
            let stdout = String::from_utf8_lossy(&out.stdout).trim().to_string();
            if stdout.is_empty() {
                let stderr = String::from_utf8_lossy(&out.stderr).trim().to_string();
                format!(r#"{{"error": "No output. stderr: {}"}}"#, stderr.replace('"', "'"))
            } else {
                stdout
            }
        }
        Err(e) => format!(r#"{{"error": "Process error: {}"}}"#, e),
    }
}

#[tauri::command]
fn save_result(payload: String) -> Result<String, String> {
    let dir = speechext_dir()?;

    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|e| e.to_string())?
        .as_secs();

    let secs = now;
    let s = secs % 60;
    let m = (secs / 60) % 60;
    let h = (secs / 3600) % 24;
    let days = secs / 86400;
    let (year, month, day) = epoch_days_to_date(days);
    let filename = format!("{:04}-{:02}-{:02}T{:02}-{:02}-{:02}Z.json", year, month, day, h, m, s);

    let path = dir.join(&filename);
    std::fs::write(&path, &payload).map_err(|e| e.to_string())?;

    Ok(filename)
}

#[tauri::command]
fn list_results() -> Result<Vec<ResultMeta>, String> {
    let dir = speechext_dir()?;

    let mut entries: Vec<ResultMeta> = std::fs::read_dir(&dir)
        .map_err(|e| e.to_string())?
        .flatten()
        .filter(|e| {
            e.path().extension().and_then(|x| x.to_str()) == Some("json")
        })
        .filter_map(|e| {
            let filename = e.file_name().to_string_lossy().to_string();
            let content = std::fs::read_to_string(e.path()).ok()?;
            let val: serde_json::Value = serde_json::from_str(&content).ok()?;
            let result_type = val["type"].as_str().unwrap_or("audio-text").to_string();
            let title = val["title"].as_str().map(|s| s.to_string());
            let created_at = filename.trim_end_matches(".json").to_string();
            Some(ResultMeta { filename, created_at, result_type, title })
        })
        .collect();

    entries.sort_by(|a, b| b.created_at.cmp(&a.created_at));

    Ok(entries)
}

#[tauri::command]
fn load_result(filename: String) -> Result<String, String> {
    let dir = speechext_dir()?;
    let path = dir.join(&filename);
    std::fs::read_to_string(path).map_err(|e| e.to_string())
}

fn epoch_days_to_date(days: u64) -> (u64, u64, u64) {
    let mut remaining = days;
    let mut year = 1970u64;
    loop {
        let days_in_year = if is_leap(year) { 366 } else { 365 };
        if remaining < days_in_year {
            break;
        }
        remaining -= days_in_year;
        year += 1;
    }
    let months = [31, if is_leap(year) { 29 } else { 28 }, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let mut month = 1u64;
    for &days_in_month in &months {
        if remaining < days_in_month {
            break;
        }
        remaining -= days_in_month;
        month += 1;
    }
    (year, month, remaining + 1)
}

fn is_leap(year: u64) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            transcribe_files,
            save_result,
            list_results,
            load_result,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
