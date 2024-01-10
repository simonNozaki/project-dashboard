use serde_json::Value;
use std::collections::HashMap;

const DEFAULT_PROJECT_NAME: &str = "無題のプロジェクト";

/// npmプロジェクトのメタデータ構造定義。
// the payload type must implement `Serialize` and `Clone`.
#[derive(Clone, serde::Serialize)]
pub struct ProjectMeta {
    pub name: String,
    pub scripts: HashMap<String, String>,
}

/// package.jsonのバッファ領域を読み込んでプロジェクト名とnpmスクリプトのタプルを抽出する
pub fn to_project_meta(file_buffer: String) -> (String, HashMap<String, String>) {
    let maybe_json: Result<Value, serde_json::Error> = serde_json::from_str(&file_buffer);
    let default_result: (String, HashMap<String, String>) =
        (DEFAULT_PROJECT_NAME.to_string(), HashMap::new());

    let json_values = match maybe_json {
        Ok(v) => v,
        Err(_) => return default_result,
    };

    let name = if let Value::String(s) = &json_values["name"] {
        String::from(s)
    } else {
        DEFAULT_PROJECT_NAME.to_string()
    };
    let mut script_map: HashMap<String, String> = HashMap::new();
    let scripts = if let Value::Object(map) = &json_values["scripts"] {
        for (k, v) in map {
            if let Value::String(s) = v {
                script_map.insert(k.to_string(), s.to_owned());
            }
        }
        script_map
    } else {
        script_map
    };

    (name, scripts)
}
