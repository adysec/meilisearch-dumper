use crate::args::IndexConfig;
use chrono::{DateTime, Utc};
use serde_json::json;
use std::fs::{self, File};
use std::io::{BufWriter, Write, BufReader, BufRead};
use std::path::Path;
use uuid::Uuid;
use std::process::Command;
use std::time::Instant;

const DUMP_DIR: &str = "meilisearch_dump";

pub fn generate_dump(configs: &[IndexConfig]) -> Result<(), String> {
    // 1. 清理并创建主目录
    let dump_path = Path::new(DUMP_DIR);
    if dump_path.exists() {
        fs::remove_dir_all(dump_path).map_err(|e| format!("清理dump目录失败: {}", e))?;
    }
    fs::create_dir(dump_path).map_err(|e| format!("创建dump目录失败: {}", e))?;
    fs::create_dir(dump_path.join("batches")).map_err(|e| e.to_string())?;
    fs::create_dir_all(dump_path.join("tasks/update_files")).map_err(|e| e.to_string())?;
    fs::create_dir(dump_path.join("indexes")).map_err(|e| e.to_string())?;

    // 2. metadata.json
    let now: DateTime<Utc> = Utc::now();
    let metadata = json!({
        "dumpVersion": "V6",
        "dbVersion": "1.15.2",
        "dumpDate": now.to_rfc3339(),
    });
    write_json(dump_path.join("metadata.json"), &metadata)?;

    // 3. instance_uid.uuid
    let instance_uid = Uuid::new_v4().to_string();
    fs::write(dump_path.join("instance_uid.uuid"), &instance_uid).map_err(|e| e.to_string())?;

    // 4. keys.jsonl
    let keys = vec![
        json!({
            "description": "Use it to search from the frontend",
            "name": "Default Search API Key",
            "uid": Uuid::new_v4().to_string(),
            "actions": ["search"],
            "indexes": ["*"],
            "expires_at": null,
            "created_at": now.to_rfc3339(),
            "updated_at": now.to_rfc3339()
        }),
        json!({
            "description": "Use it for anything that is not a search operation. Caution! Do not expose it on a public frontend",
            "name": "Default Admin API Key",
            "uid": Uuid::new_v4().to_string(),
            "actions": ["*"],
            "indexes": ["*"],
            "expires_at": null,
            "created_at": now.to_rfc3339(),
            "updated_at": now.to_rfc3339()
        }),
        json!({
            "description": "Use it to chat and search from the frontend",
            "name": "Default Chat API Key",
            "uid": Uuid::new_v4().to_string(),
            "actions": ["chatCompletions", "search"],
            "indexes": ["*"],
            "expires_at": null,
            "created_at": now.to_rfc3339(),
            "updated_at": now.to_rfc3339()
        })
    ];
    let mut keys_file = BufWriter::new(File::create(dump_path.join("keys.jsonl")).map_err(|e| e.to_string())?);
    for k in keys {
        writeln!(keys_file, "{}", k).map_err(|e| e.to_string())?;
    }

    // 5. network.json
    write_json(dump_path.join("network.json"), &json!({"self": null, "remotes": {}}))?;

    // 6. experimental-features.json
    write_json(dump_path.join("experimental-features.json"), &json!({
        "metrics": false,
        "logsRoute": false,
        "editDocumentsByFunction": false,
        "containsFilter": false,
        "network": false,
        "getTaskDocumentsRoute": false,
        "compositeEmbedders": false,
        "chatCompletions": false
    }))?;

    // 7. 每个索引目录和文件
    for c in configs {
        let idx_dir = dump_path.join("indexes").join(&c.index);
        fs::create_dir_all(&idx_dir).map_err(|e| e.to_string())?;
        // metadata.json
        let idx_meta = json!({
            "uid": c.index,
            "primaryKey": "id",
            "createdAt": now.to_rfc3339(),
            "updatedAt": now.to_rfc3339()
        });
        write_json(idx_dir.join("metadata.json"), &idx_meta)?;
        // settings.json
        let settings = json!({
            "displayedAttributes": c.displayed,
            "searchableAttributes": c.searchable,
            "filterableAttributes": c.filterable,
            "sortableAttributes": [],
            "rankingRules": ["words", "typo", "proximity", "attribute", "sort", "exactness"],
            "stopWords": [],
            "nonSeparatorTokens": [],
            "separatorTokens": [],
            "dictionary": [],
            "synonyms": {},
            "distinctAttribute": null,
            "proximityPrecision": "byWord",
            "typoTolerance": {
                "enabled": true,
                "minWordSizeForTypos": {"oneTypo": 5, "twoTypos": 9},
                "disableOnWords": [],
                "disableOnAttributes": [],
                "disableOnNumbers": false
            },
            "faceting": {
                "maxValuesPerFacet": 100,
                "sortFacetValuesBy": {"*": "alpha"}
            },
            "pagination": {
                "maxTotalHits": 1000
            },
            "embedders": {},
            "searchCutoffMs": null,
            "localizedAttributes": null,
            "facetSearch": true,
            "prefixSearch": "indexingTime",
            "chat": {
                "description": "",
                "documentTemplate": "{% for field in fields %}{% if field.is_searchable and field.value != nil %}{{ field.name }}: {{ field.value }}\\n{% endif %}{% endfor %}",
                "documentTemplateMaxBytes": 400,
                "searchParameters": {}
            }
        });
        write_json(idx_dir.join("settings.json"), &settings)?;
        // documents.jsonl
        let file = File::open(&c.file).map_err(|e| e.to_string())?;
        let reader = BufReader::new(file);
        let mut doc_file = BufWriter::new(File::create(idx_dir.join("documents.jsonl")).map_err(|e| e.to_string())?);
        let mut lines = reader.lines();
        // 跳过第一行 [
        lines.next();
        let start = Instant::now();
        let mut line_count = 0u64;
        for line in lines {
            let line = line.map_err(|e| e.to_string())?;
            let trimmed = line.trim_end();
            // 只处理以{开头且以},结尾的行
            if trimmed.starts_with('{') && trimmed.ends_with("},") {
                let new_line = &trimmed[..trimmed.len()-1]; // 去掉末尾逗号
                writeln!(doc_file, "{}", new_line).map_err(|e| e.to_string())?;
                line_count += 1;
            }
            // 其它行全部忽略
        }
        doc_file.flush().map_err(|e| e.to_string())?;
        let elapsed = start.elapsed();
        let idx_file_path = idx_dir.join("documents.jsonl");
        let dump_size = std::fs::metadata(&idx_file_path).map(|m| m.len()).unwrap_or(0);
        println!(
            "索引 {}: 处理行数: {}，dump文件大小: {:.2} MB，用时: {:.2?}，平均速度: {:.2} 行/秒",
            c.index,
            line_count,
            dump_size as f64 / 1024.0 / 1024.0,
            elapsed,
            if elapsed.as_secs_f64() > 0.0 { line_count as f64 / elapsed.as_secs_f64() } else { 0.0 }
        );
    }

    // 8. batches/queue.jsonl
    let mut index_uids = serde_json::Map::new();
    for c in configs {
        index_uids.insert(c.index.clone(), json!(1));
    }
    let batches_queue = json!({
        "uid": 0,
        "details": {"receivedDocuments": 2, "indexedDocuments": 2},
        "stats": {
            "totalNbTasks": 1,
            "status": {"succeeded": 1},
            "types": {"documentAdditionOrUpdate": 1},
            "indexUids": index_uids,
            "progressTrace": serde_json::Map::new(),
            "writeChannelCongestion": serde_json::Map::new(),
            "internalDatabaseSizes": serde_json::Map::new()
        },
        "startedAt": now.to_rfc3339(),
        "finishedAt": now.to_rfc3339(),
        "enqueuedAt": {"earliest": now.to_rfc3339(), "oldest": now.to_rfc3339()},
        "stopReason": "batched all enqueued tasks"
    });
    let mut batch_file = BufWriter::new(File::create(dump_path.join("batches/queue.jsonl")).map_err(|e| e.to_string())?);
    writeln!(batch_file, "{}", batches_queue).map_err(|e| e.to_string())?;

    // 9. tasks/queue.jsonl
    let mut tasks_file = BufWriter::new(File::create(dump_path.join("tasks/queue.jsonl")).map_err(|e| e.to_string())?);
    for (i, c) in configs.iter().enumerate() {
        let task = json!({
            "uid": i,
            "batchUid": null,
            "indexUid": c.index,
            "status": "succeeded",
            "type": {
                "documentImport": {
                    "primary_key": null,
                    "method": "ReplaceDocuments",
                    "documents_count": 2,
                    "allow_index_creation": true
                }
            },
            "details": {
                "DocumentAdditionOrUpdate": {
                    "received_documents": 2,
                    "indexed_documents": 2
                }
            },
            "enqueuedAt": now.to_rfc3339(),
            "startedAt": now.to_rfc3339(),
            "finishedAt": now.to_rfc3339()
        });
        writeln!(tasks_file, "{}", task).map_err(|e| e.to_string())?;
    }

    // 10. 打包为tar.gz（集成shell命令）
    let tar_start = Instant::now();
    let status = Command::new("tar")
        .arg("-czf")
        .arg("meilisearch_dump.dump")
        .arg("-C")
        .arg(DUMP_DIR)
        .arg(".")
        .status()
        .map_err(|e| format!("调用tar命令失败: {}", e))?;
    if !status.success() {
        return Err(format!("tar命令执行失败，退出码: {:?}", status.code()));
    }
    let tar_elapsed = tar_start.elapsed();
    println!("打包用时: {:.2?}", tar_elapsed);
    Ok(())
}

fn write_json<P: AsRef<Path>>(path: P, value: &serde_json::Value) -> Result<(), String> {
    let file = File::create(path).map_err(|e| e.to_string())?;
    serde_json::to_writer(file, value).map_err(|e| e.to_string())?;
    Ok(())
} 
