use std::io::BufRead;

use serde_json::{Map, Value, json};

// based on https://gist.github.com/crabdancing/64c41f87c64e168f5a084c75ecfdeb4e

fn main() {
    let mut line = String::new();
    let mut handle = std::io::stdin().lock();
    loop {
        line.clear();

        if handle.read_line(&mut line).is_err() {
            break;
        }

        if line.is_empty() {
            break;
        }

        let line = line.trim();

        if !line.starts_with("@nix ") {
            println!("{line}");
            continue;
        }

        let trim = line.strip_prefix("@nix ").unwrap();

        let data: Value = match serde_json::from_str(trim) {
            Ok(e) => e,
            Err(e) => {
                eprintln!("Failed to parse serde: {e}");
                println!("{line}");
                continue;
            }
        };

        let mut nix_type = -1;

        if let Some(_nix_type) = data.get("type")
            && let Some(_nix_type) = _nix_type.as_i64()
            && _nix_type == 110
        {
            nix_type = 110;
        }

        if nix_type != 110 {
            println!("{line}");
            continue;
        }

        let empty_map = Map::new();

        let payload = data
            .get("payload")
            .expect("to get the payload")
            .as_object()
            .unwrap_or(&empty_map);

        let path = {
            if let Some(path) = data.get("path") {
                path
            } else {
                println!("{line}");
                continue;
            }
        };

        let drv_path: String = {
            if path.is_object() {
                path.get("drvPath")
                    .map(|e| e.as_str().unwrap().to_string())
                    .unwrap_or("unkown-drv".to_string())
            } else {
                path.as_str().unwrap().to_string()
            }
        };

        let name = drv_path.split("/").last().unwrap().replace(".drv", "");
        let path_hash = format!("{:x}", md5::compute(&name));
        let fake_id = isize::from_str_radix(&path_hash, 16).expect("To parse fake_id");

        let mut cache_hit = false;

        if let Some(success) = payload.get("success")
            && let Some(success) = success.as_bool()
            && success
        {
            cache_hit = true
        }

        if !cache_hit
            && let Some(status) = payload.get("status")
            && let Some(status) = status.as_str()
            && status == "AlreadyValid"
        {
            cache_hit = true
        }

        if cache_hit {
            let start_json = json!({
                "action": "start",
                "id": fake_id,
                "level": 0,
                "type": 104,
                "text": format!("cached: {name}"),
                "parent": 0
            });

            println!("@nix {}", start_json);

            let stop_json = json!({
                "action": "stop",
                "id": fake_id,
                "type": 105
            });

            println!("@nix {stop_json}");
        } else {
            let msg = json!({
                "action": "msg",
                "level": 0,
                "msg": format!("cache miss: {name}"),
            });

            println!("@nix {msg}");
        }
    }
}
