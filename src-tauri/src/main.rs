#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::process::Command;

#[tauri::command]
fn transcribe_file(path: String) -> String {
  let script = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
    .join("transcribe.py");

  let output = Command::new("/Users/alligator/Documents/education/projects/2026/speechext/.venv/bin/python3")
    .arg(script)
    .arg(&path)
    .output();

  match output {
    Ok(out) => {
      let stdout = String::from_utf8_lossy(&out.stdout).trim().to_string();
      let stderr = String::from_utf8_lossy(&out.stderr).trim().to_string();
      eprintln!("STDOUT: {:?}", stdout);
      eprintln!("STDERR: {:?}", stderr);
      if out.status.success() {
        stdout
      } else {
        format!(r#"{{"error": "{}"}}"#, stderr.replace('"', "'"))
      }
    }
    Err(e) => {
      format!(r#"{{"error": "Failed to run python3: {}"}}"#, e)
    }
  }
}

fn main() {
  tauri::Builder::default()
    .plugin(tauri_plugin_dialog::init())
    .invoke_handler(tauri::generate_handler![transcribe_file])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
