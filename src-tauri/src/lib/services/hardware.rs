use crate::lib::db::DB;
use crate::lib::models::*;
use rusqlite::params;

pub fn get_relay_status() -> Vec<RelayStatus> {
    let conn = DB.lock();
    let mut stmt = match conn.prepare("SELECT id, name, state FROM relays ORDER BY id") {
        Ok(s) => s,
        Err(_) => {
            return vec![
                RelayStatus { id: 1, name: "灯控1".to_string(), on: false },
                RelayStatus { id: 2, name: "灯控2".to_string(), on: false },
                RelayStatus { id: 3, name: "电源1".to_string(), on: false },
                RelayStatus { id: 4, name: "电源2".to_string(), on: false },
            ];
        }
    };
    let result: Vec<RelayStatus> = match stmt.query_map([], |row| {
        Ok(RelayStatus { id: row.get(0)?, name: row.get(1)?, on: row.get::<_, i32>(2)? != 0 })
    }) {
        Ok(iter) => iter.filter_map(|r| r.ok()).collect(),
        Err(_) => vec![],
    };
    result
}

pub fn set_relay(id: i64, on: bool) -> Result<RelayStatus, String> {
    let conn = DB.lock();
    match conn.execute("UPDATE relays SET state = ?1 WHERE id = ?2", params![on as i32, id]) {
        Ok(_) => {
            let (name, state): (String, i32) = conn.query_row("SELECT name, state FROM relays WHERE id = ?1", params![id], |r| Ok((r.get(0)?, r.get(1)?)))
                .map_err(|_| "继电器不存在")?;
            Ok(RelayStatus { id, name, on: state != 0 })
        }
        Err(_) => Err("继电器不存在".to_string()),
    }
}

pub fn test_connection() -> ConnectionTestResult {
    let start = std::time::Instant::now();
    let conn = DB.lock();
    let result: Result<usize, rusqlite::Error> = conn.execute("SELECT 1", []);
    let elapsed = start.elapsed().as_millis() as u64;
    match result {
        Ok(_) => ConnectionTestResult {
            success: true,
            latency_ms: Some(elapsed),
            message: Some(format!("数据库连接正常 ({}ms)", elapsed)),
        },
        Err(e) => ConnectionTestResult {
            success: false,
            latency_ms: Some(elapsed),
            message: Some(format!("连接失败: {}", e)),
        },
    }
}

pub fn get_hardware_status() -> Vec<HardwareStatusItem> {
    let mut items = Vec::new();
    items.push(HardwareStatusItem {
        device_type: "printer".to_string(),
        device_name: "打印机".to_string(),
        online: true,
    });
    let conn = DB.lock();
    let count: i64 = conn.query_row("SELECT COUNT(*) FROM relays", [], |r| r.get(0)).unwrap_or(0);
    let online = count >= 0;
    items.push(HardwareStatusItem {
        device_type: "relay".to_string(),
        device_name: "控制板".to_string(),
        online,
    });
    items
}
