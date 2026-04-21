use crate::lib::db::DB;
use crate::lib::models::*;
use rusqlite::params;

pub fn get_bookings(status: Option<String>) -> Vec<Booking> {
    let conn = DB.lock();
    let query = "SELECT b.id, b.customer_name, b.customer_phone, b.table_id, COALESCE(t.name, ''), b.booking_time, COALESCE(b.duration_hours,2), b.status, b.remark, b.created_at FROM bookings b LEFT JOIN tables t ON b.table_id = t.id WHERE (?1 IS NULL OR b.status = ?1) ORDER BY b.booking_time";
    let mut stmt = match conn.prepare(query) { Ok(s) => s, Err(_) => return vec![] };
    let result: Vec<Booking> = match stmt.query_map(params![status], |row| {
        Ok(Booking { id: row.get(0)?, customer_name: row.get(1)?, customer_phone: row.get(2)?, table_id: row.get(3)?, table_name: row.get(4)?, booking_time: row.get(5)?, duration_hours: row.get(6)?, status: row.get(7)?, remark: row.get(8)?, created_at: row.get(9)? })
    }) { Ok(iter) => iter.filter_map(|r| r.ok()).collect(), Err(_) => vec![] };
    result
}

pub fn create_booking(req: CreateBookingRequest) -> Result<Booking, String> {
    let conn = DB.lock();
    conn.execute("INSERT INTO bookings (customer_name, customer_phone, table_id, booking_time, duration_hours, status, remark) VALUES (?1, ?2, ?3, ?4, ?5, 'pending', ?6)", params![req.customer_name, req.customer_phone, req.table_id, req.booking_time, req.duration_hours.unwrap_or(2.0), req.remark]).map_err(|e| e.to_string())?;
    let id = conn.last_insert_rowid();
    let table_name: Option<String> = if let Some(tid) = req.table_id { conn.query_row("SELECT name FROM tables WHERE id = ?1", params![tid], |r| r.get(0)).ok() } else { None };
    Ok(Booking { id, customer_name: req.customer_name, customer_phone: req.customer_phone, table_id: req.table_id, table_name, booking_time: req.booking_time, duration_hours: req.duration_hours.unwrap_or(2.0), status: "pending".to_string(), remark: req.remark, created_at: Some(chrono::Utc::now().to_rfc3339()) })
}

pub fn update_booking(id: i64, req: UpdateBookingRequest) -> Result<Booking, String> {
    let conn = DB.lock();
    if let Some(ref name) = req.customer_name { conn.execute("UPDATE bookings SET customer_name = ?1 WHERE id = ?2", params![name, id]).ok(); }
    if let Some(ref phone) = req.customer_phone { conn.execute("UPDATE bookings SET customer_phone = ?1 WHERE id = ?2", params![phone, id]).ok(); }
    if let Some(tid) = req.table_id { conn.execute("UPDATE bookings SET table_id = ?1 WHERE id = ?2", params![tid, id]).ok(); }
    if let Some(ref time) = req.booking_time { conn.execute("UPDATE bookings SET booking_time = ?1 WHERE id = ?2", params![time, id]).ok(); }
    if let Some(h) = req.duration_hours { conn.execute("UPDATE bookings SET duration_hours = ?1 WHERE id = ?2", params![h, id]).ok(); }
    if let Some(ref s) = req.status { conn.execute("UPDATE bookings SET status = ?1 WHERE id = ?2", params![s, id]).ok(); }
    if let Some(ref r) = req.remark { conn.execute("UPDATE bookings SET remark = ?1 WHERE id = ?2", params![r, id]).ok(); }
    drop(conn);
    get_bookings(None).into_iter().find(|b| b.id == id).ok_or("预订不存在".to_string())
}

pub fn delete_booking(id: i64) -> Result<(), String> {
    let conn = DB.lock();
    conn.execute("DELETE FROM bookings WHERE id = ?1", params![id]).map_err(|e| e.to_string())?;
    Ok(())
}
