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

fn sort_audio_paths(paths: &mut Vec<String>) {
    paths.sort_by(|a, b| {
        let extract = |s: &str| -> Option<u64> {
            let name = std::path::Path::new(s)
                .file_stem()?
                .to_str()?
                .to_string();
            // если имя файла вида "audio_42" — берём число
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

#[tauri::command]
async fn transcribe_files(files: Vec<String>, window: tauri::Window) -> String {
    let mut sorted_files = files.clone();
    sort_audio_paths(&mut sorted_files);

    let script = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("transcribe.py");

    let mut cmd = Command::new("/Users/alligator/Documents/education/projects/2026/speechext/.venv/bin/python3");
    cmd.arg(script);
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

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![transcribe_files])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
