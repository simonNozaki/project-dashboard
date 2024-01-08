// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use ferris_says::say;
use std::io::{stdout, BufWriter, Read, BufRead};
use std::fs::File;
use std::process::Stdio;
use tauri::Window;
use package_dashboard::foundations::package_json::{ProjectMeta, to_project_meta};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

/// 指定されたディレクトリ直下のpackage.jsonを探して、プロジェクト名とスクリプトを読み込む
/// # Errors
/// 指定されたディレクトリ直下に `package.json` がないとき、Errorを返す
#[tauri::command]
fn set_npm_project(dir: &str) -> Result<ProjectMeta, String> {
    // FIXME: 動作安定したら消す(デバッグコード)
    println!("Directory: {}", dir);
    let package_json = format!("{}/package.json", dir);

    // ファイルを読み出してバッファに書き込み -> JSONをパースしてプロジェクト名を取り出す
    let file = File::open(&package_json);
    let mut buffer = String::from("");
    if let Ok(mut f) = file {
        let _ = f.read_to_string(&mut buffer);
    } else {
        return Err(String::from("$error_file_not_found"));
    }

    let (name, scripts) = to_project_meta(buffer);
    for (k, v) in &scripts {
        // デバッグコードだが標準出力にはでて困るものでもないので残す
        println!("{}, {}", k, v);
    }
    Ok(ProjectMeta { name, scripts })
}

// the payload type must implement `Serialize` and `Clone`.
#[derive(Clone, serde::Serialize)]
struct Payload {
    text: String
}

// the payload type must implement `Serialize` and `Clone`.
#[derive(Clone, serde::Serialize)]
enum ExecutionStatus {
    Failed
}

/// package.jsonから読み込んだnpmスクリプトを実行し、結果をEventとして逐次emitする
#[tauri::command]
fn execute_script(current_dir: &str, script: &str, window: Window) -> Result<(), ExecutionStatus> {
    println!("Window: {}", window.label());
    let yarn = std::process::Command::new("yarn")
        .current_dir(current_dir)
        .arg(script)
        .stdout(Stdio::piped())
        .spawn();
    let stdout = if let Ok(v) = yarn {
        v.stdout
    } else {
        println!("[error] {}", yarn.unwrap_err());
        return Err(ExecutionStatus::Failed);
    };
    let child_stdout = match stdout {
        Some(v) => v,
        None => {
            println!("Error on unwrapping an error.");
            return Err(ExecutionStatus::Failed);
        }
    };
    let reader = std::io::BufReader::new(child_stdout);
    let mut result = String::from("");
    for line in reader.lines() {
        if let Ok(l) = line {
            println!("[debug] {}", &l);
            result.push_str(&l);
            result.push_str("\n");
            window.emit("on-print-stdout", Payload { text: result.to_owned() }).unwrap();
        }
    }

    Ok(())
}

fn main() {
    let text = "Starting tauri app ...";
    let writer = BufWriter::new(stdout());
    say(text, 24, writer).unwrap();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            set_npm_project,
            execute_script
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
