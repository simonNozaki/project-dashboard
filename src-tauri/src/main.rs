// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use ferris_says::say;
use log::{debug, error, LevelFilter};
use package_dashboard::foundations::package_json::{to_project_meta, ProjectMeta};
use std::fs::File;
use std::io::{stdout, BufWriter, Read};
use std::process::{ExitStatus, Stdio};
use tauri::Window;
use tauri_plugin_log::LogTarget;
use tokio::io::AsyncBufReadExt;
use tokio::io::BufReader;
use tokio::process::Command;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

/// 指定されたディレクトリ直下のpackage.jsonを探して、プロジェクト名とスクリプトを読み込む
/// # Errors
/// 指定されたディレクトリ直下に `package.json` がないとき、Errorを返す
#[tauri::command]
fn set_npm_project(dir: &str) -> Result<ProjectMeta, String> {
    debug!("Directory: {}", dir);
    let package_json = format!("{}/package.json", dir);

    // ファイルを読み出してバッファに書き込み -> JSONをパースしてプロジェクト名を取り出す
    let file = File::open(package_json);
    let mut buffer = String::from("");
    if let Ok(mut f) = file {
        let _ = f.read_to_string(&mut buffer);
    } else {
        return Err(String::from("$error_file_not_found"));
    }

    let (name, scripts) = to_project_meta(buffer);
    Ok(ProjectMeta { name, scripts })
}

// the payload type must implement `Serialize` and `Clone`.
#[derive(Clone, serde::Serialize)]
struct Payload {
    text: String,
}

// the payload type must implement `Serialize` and `Clone`.
#[derive(Clone, serde::Serialize)]
struct SerializableStatus {
    code: Option<i32>,
}

impl From<ExitStatus> for SerializableStatus {
    fn from(value: ExitStatus) -> Self {
        SerializableStatus { code: value.code() }
    }
}

/// package.jsonから読み込んだnpmスクリプトを実行し、結果をEventとして逐次emitする
#[tauri::command]
async fn execute_async(
    current_dir: &str,
    script: &str,
    window: Window,
) -> Result<SerializableStatus, String> {
    let yarn = Command::new("yarn")
        .current_dir(current_dir)
        .arg(script)
        .stdout(Stdio::piped())
        .spawn();
    let mut child = match yarn {
        Ok(c) => c,
        Err(e) => {
            let message = format!("Error spawning a process: {}", e);
            error!("{}", &message);
            return Err(message);
        }
    };
    let stdout = child.stdout.take().expect("");
    let reader = BufReader::new(stdout);
    let lines = reader.lines();
    tokio::pin!(lines);

    let mut result = String::from("");
    while let Ok(next) = lines.next_line().await {
        match next {
            Some(line) => {
                debug!("{}", &line);
                result.push_str(&line);
                result.push('\n');
                window
                    .emit(
                        "on-print-stdout",
                        Payload {
                            text: result.to_owned(),
                        },
                    )
                    .unwrap();
            }
            None => break,
        }
    }

    match child.wait().await {
        Ok(status) => Ok(SerializableStatus::from(status)),
        Err(e) => {
            let message = format!("Error on stopping a process: {}", e);
            error!("{}", &message);
            Err(message)
        }
    }
}

fn main() {
    let text = "Starting tauri app ...";
    let writer = BufWriter::new(stdout());
    say(text, 24, writer).unwrap();

    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::default()
                .targets([LogTarget::LogDir, LogTarget::Stdout, LogTarget::Webview])
                .level(LevelFilter::Debug) // FIXME: 起動時にdebugログ出すか指定できるようにする
                .build(),
        )
        .invoke_handler(tauri::generate_handler![set_npm_project, execute_async])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
