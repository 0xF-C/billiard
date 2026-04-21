use crate::lib::db::DB;
use crate::lib::db::get_db_path;
use crate::lib::models::*;
use std::path::PathBuf;
use std::io::Read;
use log::info;

pub fn backup_database() -> BackupResult {
    let db_path = get_db_path();
    let backup_dir = db_path.parent().map(|p| p.join("backups")).unwrap_or_else(|| PathBuf::from("backups"));
    std::fs::create_dir_all(&backup_dir).ok();
    let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
    let backup_path = backup_dir.join(format!("billiard_backup_{}.db", timestamp));
    
    match std::fs::copy(&db_path, &backup_path) {
        Ok(_) => {
            let size = std::fs::metadata(&backup_path).map(|m| m.len()).unwrap_or(0);
            BackupResult {
                success: true,
                path: backup_path.to_string_lossy().to_string(),
                size: size as i64,
                message: Some(format!("备份成功，大小: {} 字节", size)),
            }
        }
        Err(e) => BackupResult {
            success: false,
            path: String::new(),
            size: 0,
            message: Some(format!("备份失败: {}", e)),
        },
    }
}

pub fn restore_database(path: String) -> Result<(), String> {
    let source = std::path::Path::new(&path);
    
    if !source.exists() {
        return Err(format!("备份文件不存在: {}", path));
    }
    let metadata = std::fs::metadata(source).map_err(|e| format!("无法读取文件: {}", e))?;
    if metadata.len() == 0 {
        return Err("备份文件大小为0，文件已损坏".to_string());
    }
    
    match source.extension().and_then(|e| e.to_str()) {
        Some("db") | Some("sqlite") | Some("sqlite3") => {}
        _ => return Err("备份文件格式不正确，仅支持 .db / .sqlite / .sqlite3 文件".to_string()),
    }
    
    let mut header = [0u8; 16];
    let mut file = std::fs::File::open(source).map_err(|e| format!("无法打开备份文件: {}", e))?;
    file.read_exact(&mut header).map_err(|e| format!("无法读取文件头: {}", e))?;
    if &header[..15] != b"SQLite format 3" {
        return Err("文件不是有效的 SQLite 数据库".to_string());
    }
    
    let db_path = get_db_path();
    
    let backup_timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
    let pre_restore_backup = db_path.parent()
        .map(|p| p.join("backups"))
        .unwrap_or_else(|| PathBuf::from("backups"));
    std::fs::create_dir_all(&pre_restore_backup).ok();
    let pre_backup_path = pre_restore_backup.join(format!("pre_restore_{}.db", backup_timestamp));
    let _ = std::fs::copy(&db_path, &pre_backup_path);
    
    std::fs::copy(&path, &db_path).map_err(|e| format!("恢复失败: {}", e))?;
    
    // Verify database integrity after restore
    let conn = DB.lock();
    let integrity: String = conn.query_row("PRAGMA integrity_check", [], |r| r.get(0))
        .unwrap_or_else(|_| "unknown".to_string());
    if integrity != "ok" {
        // Restore failed, roll back to pre-restore backup
        if pre_backup_path.exists() {
            std::fs::copy(&pre_backup_path, &db_path).ok();
        }
        return Err(format!("恢复的数据库完整性检查失败: {}", integrity));
    }
    
    info!("数据库恢复成功，完整性检查通过，原数据库已备份至: {}", pre_backup_path.display());
    Ok(())
}
