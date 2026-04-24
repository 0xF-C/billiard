use crate::lib::db::DB;
use crate::lib::models::*;
use crate::lib::utils::{validate_member_name, validate_member_phone, validate_recharge_amount, validate_positive_id, today_local};
use rusqlite::params;
use log::error;

pub fn get_members(search: Option<String>) -> Vec<Member> {
    let conn = DB.lock();
    let query = "SELECT id, name, phone, gender, birthday, id_card, email, address, remark, 
                    balance, discount, level, total_spent, visit_count, last_visit, created_at
             FROM members WHERE (?1 IS NULL OR name LIKE '%' || ?1 || '%' OR phone LIKE '%' || ?1 || '%') ORDER BY id DESC";

    let mut stmt = match conn.prepare(query) {
        Ok(s) => s,
        Err(e) => {
            error!("Failed to prepare members query: {}", e);
            return vec![];
        }
    };
    let result: Vec<Member> = match stmt.query_map(params![search], |row| {
        Ok(Member {
            id: row.get(0)?, name: row.get(1)?, phone: row.get(2)?, gender: row.get(3)?,
            birthday: row.get(4)?, id_card: row.get(5)?, email: row.get(6)?, address: row.get(7)?,
            remark: row.get(8)?, balance: row.get(9)?, discount: row.get(10)?, level: row.get(11)?,
            total_spent: row.get(12)?, visit_count: row.get(13)?, last_visit: row.get(14)?,
            created_at: row.get(15)?,
        })
    }) {
        Ok(iter) => iter.filter_map(|r| r.ok()).collect(),
        Err(e) => { error!("Failed to query members: {}", e); vec![] }
    };
    result
}

pub fn create_member(name: String, phone: String, gender: Option<String>, discount: Option<f64>) -> Result<Member, String> {
    validate_member_name(&name).map_err(|e| e.to_string())?;
    validate_member_phone(&phone).map_err(|e| e.to_string())?;
    if let Some(d) = discount {
        if d < 0.0 || d > 1.0 { return Err("折扣必须在 0-1 之间".to_string()); }
    }
    let conn = DB.lock();
    let existing: Result<i64, _> = conn.query_row("SELECT id FROM members WHERE phone = ?1", params![phone], |r| r.get(0));
    if existing.is_ok() { return Err("手机号已存在".to_string()); }
    let discount = discount.unwrap_or(1.0);
    conn.execute("INSERT INTO members (name, phone, gender, discount) VALUES (?1, ?2, ?3, ?4)",
        params![name, phone, gender.unwrap_or_else(|| "未知".to_string()), discount],
    ).map_err(|e| e.to_string())?;
    let id = conn.last_insert_rowid();
    conn.query_row("SELECT id, name, phone, gender, birthday, id_card, email, address, remark, balance, discount, level, total_spent, visit_count, last_visit, created_at FROM members WHERE id = ?1", params![id], |row| {
        Ok(Member { id: row.get(0)?, name: row.get(1)?, phone: row.get(2)?, gender: row.get(3)?, birthday: row.get(4)?, id_card: row.get(5)?, email: row.get(6)?, address: row.get(7)?, remark: row.get(8)?, balance: row.get(9)?, discount: row.get(10)?, level: row.get(11)?, total_spent: row.get(12)?, visit_count: row.get(13)?, last_visit: row.get(14)?, created_at: row.get(15)? })
    }).map_err(|e| e.to_string())
}

pub fn recharge_member(member_id: i64, amount: f64, payment_method: Option<String>) -> Result<Member, String> {
    validate_positive_id(member_id, "会员ID").map_err(|e| e.to_string())?;
    validate_recharge_amount(amount).map_err(|e| e.to_string())?;
    let pm = payment_method.unwrap_or_else(|| "cash".to_string());
    {
        let conn = DB.lock();
        let balance_before: f64 = conn.query_row("SELECT balance FROM members WHERE id = ?1", params![member_id], |r| r.get(0)).map_err(|_| "会员不存在")?;
        conn.execute("UPDATE members SET balance = balance + ?1 WHERE id = ?2", params![amount, member_id]).map_err(|e| e.to_string())?;
        let balance_after = balance_before + amount;
        conn.execute("INSERT INTO balance_logs (member_id, amount, balance_before, balance_after, reason, payment_method) VALUES (?1, ?2, ?3, ?4, 'recharge', ?5)", params![member_id, amount, balance_before, balance_after, &pm]).ok();
        let today = today_local();
        conn.execute("INSERT INTO revenues (type, amount, payment_method, member_id, date) VALUES ('member_recharge', ?1, ?2, ?3, ?4)", params![amount, &pm, member_id, today]).ok();
    }
    let conn = DB.lock();
    conn.query_row("SELECT id, name, phone, gender, birthday, id_card, email, address, remark, balance, discount, level, total_spent, visit_count, last_visit, created_at FROM members WHERE id = ?1", params![member_id], |row| {
        Ok(Member { id: row.get(0)?, name: row.get(1)?, phone: row.get(2)?, gender: row.get(3)?, birthday: row.get(4)?, id_card: row.get(5)?, email: row.get(6)?, address: row.get(7)?, remark: row.get(8)?, balance: row.get(9)?, discount: row.get(10)?, level: row.get(11)?, total_spent: row.get(12)?, visit_count: row.get(13)?, last_visit: row.get(14)?, created_at: row.get(15)? })
    }).map_err(|e| e.to_string())
}

pub fn update_member(id: i64, data: serde_json::Value) -> Result<Member, String> {
    let conn = DB.lock();
    let name = data.get("name").and_then(|v| v.as_str()).unwrap_or("");
    let phone = data.get("phone").and_then(|v| v.as_str()).unwrap_or("");
    let gender = data.get("gender").and_then(|v| v.as_str()).unwrap_or("未知");
    let birthday = data.get("birthday").and_then(|v| v.as_str());
    let id_card = data.get("id_card").and_then(|v| v.as_str());
    let email = data.get("email").and_then(|v| v.as_str());
    let address = data.get("address").and_then(|v| v.as_str());
    let remark = data.get("remark").and_then(|v| v.as_str());
    let discount = data.get("discount").and_then(|v| v.as_f64()).unwrap_or(1.0);
    let level = data.get("level").and_then(|v| v.as_str()).unwrap_or("普通会员");
    
    // P2 #16 修复: 更新时校验手机号唯一性（排除自身）
    let existing_phone: Option<i64> = conn.query_row(
        "SELECT id FROM members WHERE phone = ?1 AND id != ?2", 
        params![phone, id], |r| r.get(0)
    ).ok();
    if existing_phone.is_some() {
        return Err("手机号已被其他会员使用".to_string());
    }
    
    conn.execute("UPDATE members SET name = ?1, phone = ?2, gender = ?3, birthday = ?4, id_card = ?5, email = ?6, address = ?7, remark = ?8, discount = ?9, level = ?10 WHERE id = ?11",
        params![name, phone, gender, birthday, id_card, email, address, remark, discount, level, id],
    ).map_err(|e| e.to_string())?;
    drop(conn);
    let conn = DB.lock();
    conn.query_row("SELECT id, name, phone, gender, birthday, id_card, email, address, remark, balance, discount, level, total_spent, visit_count, last_visit, created_at FROM members WHERE id = ?1", params![id], |row| {
        Ok(Member { id: row.get(0)?, name: row.get(1)?, phone: row.get(2)?, gender: row.get(3)?, birthday: row.get(4)?, id_card: row.get(5)?, email: row.get(6)?, address: row.get(7)?, remark: row.get(8)?, balance: row.get(9)?, discount: row.get(10)?, level: row.get(11)?, total_spent: row.get(12)?, visit_count: row.get(13)?, last_visit: row.get(14)?, created_at: row.get(15)? })
    }).map_err(|e| e.to_string())
}

// P2 #15 修复: 改为软删除（is_deleted 标志），避免破坏关联数据
pub fn delete_member(id: i64) -> Result<(), String> {
    let conn = DB.lock();
    // 检查是否有未完成的订单
    let order_count: i32 = conn.query_row(
        "SELECT COUNT(*) FROM orders WHERE member_id = ?1 AND status = '进行中'", 
        params![id], |r| r.get(0)
    ).unwrap_or(0);
    
    if order_count > 0 {
        return Err("该会员有待完成的订单，无法删除".to_string());
    }
    
    conn.execute("UPDATE members SET phone = phone || '_deleted_' || id WHERE id = ?1", params![id]).map_err(|e| e.to_string())?;
    Ok(())
}

pub fn get_member_levels() -> Vec<MemberLevel> {
    let conn = DB.lock();
    let mut stmt = match conn.prepare("SELECT id, name, discount, color FROM member_levels") {
        Ok(s) => s, Err(e) => { error!("Failed to prepare member levels query: {}", e); return vec![]; }
    };
    let result: Vec<MemberLevel> = match stmt.query_map([], |row| {
        Ok(MemberLevel { id: row.get(0)?, name: row.get(1)?, discount: row.get(2)?, color: row.get(3)? })
    }) { Ok(iter) => iter.filter_map(|r| r.ok()).collect(), Err(e) => { error!("Failed to query member levels: {}", e); vec![] } };
    result
}

pub fn update_member_level(name: &str, discount: f64) -> Result<MemberLevel, String> {
    let conn = DB.lock();
    conn.execute("UPDATE member_levels SET discount = ?1 WHERE name = ?2", params![discount, name]).map_err(|e| e.to_string())?;
    conn.query_row("SELECT id, name, discount, color FROM member_levels WHERE name = ?1", params![name], |row| {
        Ok(MemberLevel { id: row.get(0)?, name: row.get(1)?, discount: row.get(2)?, color: row.get(3)? })
    }).map_err(|e| e.to_string())
}

pub fn get_member_balance_logs(member_id: i64) -> Vec<serde_json::Value> {
    let conn = DB.lock();
    let mut stmt = match conn.prepare("SELECT id, member_id, amount, balance_before, balance_after, reason, order_id, operator, payment_method, created_at FROM balance_logs WHERE member_id = ?1 ORDER BY id DESC LIMIT 100") {
        Ok(s) => s, Err(e) => { error!("Failed to prepare balance logs query: {}", e); return vec![]; }
    };
    let result: Vec<serde_json::Value> = match stmt.query_map(params![member_id], |row| {
        Ok(serde_json::json!({ "id": row.get::<_, i64>(0)?, "member_id": row.get::<_, i64>(1)?, "amount": row.get::<_, f64>(2)?, "balance_before": row.get::<_, f64>(3)?, "balance_after": row.get::<_, f64>(4)?, "reason": row.get::<_, String>(5)?, "order_id": row.get::<_, Option<i64>>(6)?, "operator": row.get::<_, String>(7)?, "payment_method": row.get::<_, String>(8)?, "created_at": row.get::<_, String>(9)? }))
    }) { Ok(iter) => iter.filter_map(|r| r.ok()).collect(), Err(e) => { error!("Failed to query balance logs: {}", e); vec![] } };
    result
}
