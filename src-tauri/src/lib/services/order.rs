use crate::lib::db::DB;
use crate::lib::models::*;
use crate::lib::services::settings::load_settings;
use crate::lib::utils::{calc_bill_minutes, validate_open_table_request, validate_close_table_request, today_local};
use crate::lib::services::settings::get_member_day_discount;
use crate::lib::services::printer::print_receipt;
use chrono::{DateTime, Utc};
use rusqlite::params;
use rusqlite::Connection;
use log::{info, warn, error};

pub fn get_order_by_id(order_id: i64) -> Result<Order, String> {
    let conn = DB.lock();
    let order = get_order_by_id_locked(&conn, order_id)?;
    drop(conn);
    Ok(order)
}

fn get_order_by_id_locked(conn: &Connection, order_id: i64) -> Result<Order, String> {
    conn.query_row(
        "SELECT o.id, o.table_id, t.name, o.member_id, COALESCE(m.name, o.customer_name, '散客'), 
                o.customer_name, o.customer_phone, o.start_time, o.end_time, o.duration_minutes,
                o.total_amount, o.discount_amount, o.deposit, o.change_amount, o.final_amount, o.status,
                o.package_id, o.package_name, o.package_price, o.package_hours, o.last_deduction_time,
                o.cancel_time, o.cancel_reason, o.deposit_refunded, o.refund_method, o.payment_method
         FROM orders o 
         LEFT JOIN tables t ON o.table_id = t.id 
         LEFT JOIN members m ON o.member_id = m.id 
         WHERE o.id = ?1",
        params![order_id],
        |row| {
            Ok(Order {
                id: row.get(0)?,
                table_id: row.get(1)?,
                table_name: row.get(2)?,
                member_id: row.get(3)?,
                member_name: row.get(4)?,
                customer_name: row.get(5)?,
                customer_phone: row.get(6)?,
                start_time: row.get(7)?,
                end_time: row.get(8)?,
                duration_minutes: row.get(9)?,
                total_amount: row.get(10)?,
                discount_amount: row.get(11)?,
                deposit: row.get(12)?,
                change_amount: row.get(13)?,
                final_amount: row.get(14)?,
                status: row.get(15)?,
                package_id: row.get(16)?,
                package_name: row.get(17)?,
                package_price: row.get(18)?,
                package_hours: row.get(19)?,
                last_deduction_time: row.get(20)?,
                cancel_time: row.get(21)?,
                cancel_reason: row.get(22)?,
                deposit_refunded: row.get(23)?,
                refund_method: row.get(24)?,
                payment_method: row.get(25)?,
            })
        },
    ).map_err(|e| e.to_string())
}

pub fn open_table(req: OpenTableRequest) -> Result<Order, String> {
    validate_open_table_request(&req).map_err(|e| e.to_string())?;
    
    let order_id: i64;

    let (package_name, package_price, package_hours) = if let Some(pid) = req.package_id {
        let settings = load_settings();
        if let Some(pkg) = settings.packages.iter().find(|p| p.id == pid as i32) {
            (Some(pkg.name.clone()), Some(pkg.price), Some(pkg.hours))
        } else {
            (None, None, None)
        }
    } else {
        (None, None, None)
    };

    {
        let mut conn = DB.lock();
        let tx = conn.transaction().map_err(|e| format!("Transaction failed: {}", e))?;

        let status: String = tx
            .query_row(
                "SELECT status FROM tables WHERE id = ?1",
                params![req.table_id],
                |r| r.get(0),
            )
            .map_err(|_| "球桌不存在")?;

        if status != "空闲" {
            tx.rollback().ok();
            return Err(format!("球桌当前状态: {}，无法开台", status));
        }

        let now = chrono::Utc::now().to_rfc3339();

        if let Err(e) = tx.execute(
            "INSERT INTO orders (table_id, member_id, customer_name, customer_phone, deposit, package_id, package_name, package_price, package_hours, status, start_time) 
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, '进行中', ?10)",
            params![
                req.table_id,
                req.member_id,
                req.customer_name,
                req.customer_phone,
                req.deposit.unwrap_or(0.0),
                req.package_id,
                package_name,
                package_price,
                package_hours,
                now
            ],
        ) {
            tx.rollback().ok();
            return Err(e.to_string());
        }

        order_id = tx.last_insert_rowid();

        if let Err(e) = tx.execute(
            "UPDATE tables SET status = '使用中' WHERE id = ?1",
            params![req.table_id],
        ) {
            tx.rollback().ok();
            return Err(e.to_string());
        }

        if let Some(member_id) = req.member_id {
            tx.execute(
                "UPDATE members SET visit_count = visit_count + 1, last_visit = ?1 WHERE id = ?2",
                params![now, member_id],
            ).ok();
        }

        tx.commit().map_err(|e| format!("Commit failed: {}", e))?;
    }

    get_order_by_id(order_id)
}

pub fn get_active_orders() -> Vec<Order> {
    let conn = DB.lock();
    let mut stmt = match conn.prepare(
        "SELECT o.id, o.table_id, t.name, o.member_id, COALESCE(m.name, o.customer_name, '散客'), 
                o.customer_name, o.customer_phone, o.start_time, o.end_time, o.duration_minutes,
                o.total_amount, o.discount_amount, o.deposit, o.change_amount, o.final_amount, o.status,
                o.package_id, o.package_name, o.package_price, o.package_hours, o.last_deduction_time,
                o.cancel_time, o.cancel_reason, o.deposit_refunded, o.refund_method, o.payment_method
         FROM orders o 
         LEFT JOIN tables t ON o.table_id = t.id 
         LEFT JOIN members m ON o.member_id = m.id 
         WHERE o.status = '进行中' 
         ORDER BY o.start_time"
    ) {
        Ok(s) => s,
        Err(e) => {
            error!("Failed to prepare active orders query: {}", e);
            return vec![];
        }
    };

    let result: Vec<Order> = match stmt.query_map([], |row| {
        Ok(Order {
            id: row.get(0)?,
            table_id: row.get(1)?,
            table_name: row.get(2)?,
            member_id: row.get(3)?,
            member_name: row.get(4)?,
            customer_name: row.get(5)?,
            customer_phone: row.get(6)?,
            start_time: row.get(7)?,
            end_time: row.get(8)?,
            duration_minutes: row.get(9)?,
            total_amount: row.get(10)?,
            discount_amount: row.get(11)?,
            deposit: row.get(12)?,
            change_amount: row.get(13)?,
            final_amount: row.get(14)?,
            status: row.get(15)?,
            package_id: row.get(16)?,
            package_name: row.get(17)?,
            package_price: row.get(18)?,
            package_hours: row.get(19)?,
            last_deduction_time: row.get(20)?,
            cancel_time: row.get(21)?,
            cancel_reason: row.get(22)?,
            deposit_refunded: row.get(23)?,
            refund_method: row.get(24)?,
            payment_method: row.get(25)?,
        })
    }) {
        Ok(iter) => iter.filter_map(|r| r.ok()).collect(),
        Err(e) => {
            error!("Failed to query active orders: {}", e);
            vec![]
        }
    };
    result
}

pub fn close_order(order_id: i64, req: CloseTableRequest) -> Result<Order, String> {
    if order_id <= 0 { return Err("订单ID无效".to_string()); }
    validate_close_table_request(&req).map_err(|e| e.to_string())?;
    
    info!("close_order called: order_id={}, payment_method={:?}", order_id, req.payment_method);
    
    let member_day_discount = get_member_day_discount();
    
    let mut conn = DB.lock();
    let tx = conn.transaction().map_err(|e| format!("Transaction failed: {}", e))?;
    
    let result = close_order_with_conn(&tx, order_id, req, member_day_discount);
    
    if result.is_ok() {
        if let Err(e) = tx.commit() {
            return Err(format!("Commit failed: {}", e));
        }
    } else {
        tx.rollback().ok();
    }
    
    // 结账成功后自动打印小票
    if let Ok(ref order) = result {
        auto_print_receipt(order);
    }
    
    result
}

fn close_order_with_conn(conn: &Connection, order_id: i64, req: CloseTableRequest, member_day_discount: i32) -> Result<Order, String> {

    let (table_id, member_id, start_time, deposit, package_id, package_price, package_hours): (i64, Option<i64>, String, f64, Option<i64>, Option<f64>, Option<f64>) = conn
        .query_row(
            "SELECT table_id, member_id, start_time, deposit, package_id, package_price, package_hours FROM orders WHERE id = ?1 AND status = '进行中'",
            params![order_id],
            |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?, r.get(4)?, r.get(5)?, r.get(6)?)),
        )
        .map_err(|_| "订单不存在或已结账")?;

    // Fix #13: Use table rate, fallback to area rate, then default 30.0
    let hourly_rate: f64 = conn
        .query_row(
            "SELECT COALESCE(NULLIF(t.rate_per_hour, 0), a.rate_per_hour, 30.0)
             FROM tables t LEFT JOIN areas a ON t.area_id = a.id
             WHERE t.id = ?1",
            params![table_id],
            |r| r.get(0),
        )
        .unwrap_or(30.0);

    let start = DateTime::parse_from_rfc3339(&start_time).map_err(|e| format!("时间解析失败: {}", e))?;
    let now = chrono::Utc::now();
    let duration = (now - start.with_timezone(&Utc)).num_minutes().max(1);
    let bill_min = calc_bill_minutes(duration);

    let mut total = 0.0;
    if let (Some(_), Some(price), Some(hours)) = (package_id, package_price, package_hours) {
        total = price;
        let pkg_duration = (hours * 60.0) as i64;
        if duration > pkg_duration {
            let extra_mins = duration - pkg_duration;
            let extra_bill = calc_bill_minutes(extra_mins);
            let extra_fee = (extra_bill as f64 / 60.0) * hourly_rate;
            total += extra_fee;
        }
    }
    if total == 0.0 {
        total = (bill_min as f64 / 60.0) * hourly_rate;
    }

    let mut discount = 0.0;
    let payment_method = req.payment_method.unwrap_or_else(|| "cash".to_string());

    if let Some(mid) = member_id {
        let member_discount: f64 = conn
            .query_row(
                "SELECT discount FROM members WHERE id = ?1",
                params![mid],
                |r| r.get(0),
            )
            .unwrap_or(1.0);

        let final_factor = member_discount * (1.0 - (member_day_discount as f64 / 100.0));
        discount = total * (1.0 - final_factor);
    }

    let final_amount = (total - discount).max(0.0);
    let change_amount = if deposit > final_amount { deposit - final_amount } else { 0.0 };
    let net_final = (final_amount - deposit).max(0.0);

    let mut balance_paid = 0.0;
    if let Some(mid) = member_id {
        let member_balance: f64 = conn
            .query_row("SELECT balance FROM members WHERE id = ?1", params![mid], |r| r.get(0))
            .unwrap_or(0.0);
        balance_paid = member_balance.max(0.0).min(net_final);
    }

    if balance_paid > 0.0 {
        if let Some(mid) = member_id {
            conn.execute(
                "UPDATE members SET balance = balance - ?1, total_spent = total_spent + ?1 WHERE id = ?2",
                params![balance_paid, mid],
            ).map_err(|e| format!("扣减余额失败: {}", e))?;
            conn.execute(
                "INSERT INTO balance_logs (member_id, order_id, amount, balance_before, balance_after, reason, payment_method) 
                 SELECT ?1, ?2, -?3, balance, balance - ?3, 'order', ?4 FROM members WHERE id = ?1",
                params![mid, order_id, balance_paid, &payment_method],
            ).ok();
        }
    }

    let now_str = now.to_rfc3339();
    let rows_affected = conn.execute(
        "UPDATE orders SET end_time = ?1, duration_minutes = ?2, total_amount = ?3, discount_amount = ?4, 
         change_amount = ?5, final_amount = ?6, payment_method = ?7, status = '已结账' WHERE id = ?8 AND status = '进行中'",
        params![
            now_str,
            duration,
            total,
            discount,
            change_amount,
            net_final,
            payment_method,
            order_id
        ],
    ).map_err(|e| e.to_string())?;

    if rows_affected == 0 {
        return Err("订单已被其他操作处理，请刷新后重试".to_string());
    }

    conn.execute(
        "UPDATE tables SET status = '空闲' WHERE id = ?1",
        params![table_id],
    ).map_err(|e| e.to_string())?;

    if net_final > 0.0 {
        let today = today_local();
        if let Err(e) = conn.execute(
            "INSERT INTO revenues (type, amount, payment_method, table_id, order_id, member_id, date) VALUES ('order', ?1, ?2, ?3, ?4, ?5, ?6)",
            params![net_final, payment_method, table_id, order_id, member_id, today],
        ) {
            warn!("Failed to record revenue: {}", e);
        }
    }

    get_order_by_id_locked(conn, order_id)
}

pub fn get_orders(status: Option<String>) -> Vec<Order> {
    let conn = DB.lock();
    let query = "SELECT o.id, o.table_id, t.name, o.member_id, COALESCE(m.name, o.customer_name, '散客'), 
                    o.customer_name, o.customer_phone, o.start_time, o.end_time, o.duration_minutes,
                    o.total_amount, o.discount_amount, o.deposit, o.change_amount, o.final_amount, o.status,
                    o.package_id, o.package_name, o.package_price, o.package_hours, o.last_deduction_time,
                    o.cancel_time, o.cancel_reason, o.deposit_refunded, o.refund_method, o.payment_method
             FROM orders o 
             LEFT JOIN tables t ON o.table_id = t.id 
             LEFT JOIN members m ON o.member_id = m.id 
             WHERE (?1 IS NULL OR o.status = ?1) ORDER BY o.id DESC";

    let mut stmt = match conn.prepare(query) {
        Ok(s) => s,
        Err(e) => {
            error!("Failed to prepare orders query: {}", e);
            return vec![];
        }
    };
    let result: Vec<Order> = match stmt.query_map(params![status], |row| {
        Ok(Order {
            id: row.get(0)?,
            table_id: row.get(1)?,
            table_name: row.get(2)?,
            member_id: row.get(3)?,
            member_name: row.get(4)?,
            customer_name: row.get(5)?,
            customer_phone: row.get(6)?,
            start_time: row.get(7)?,
            end_time: row.get(8)?,
            duration_minutes: row.get(9)?,
            total_amount: row.get(10)?,
            discount_amount: row.get(11)?,
            deposit: row.get(12)?,
            change_amount: row.get(13)?,
            final_amount: row.get(14)?,
            status: row.get(15)?,
            package_id: row.get(16)?,
            package_name: row.get(17)?,
            package_price: row.get(18)?,
            package_hours: row.get(19)?,
            last_deduction_time: row.get(20)?,
            cancel_time: row.get(21)?,
            cancel_reason: row.get(22)?,
            deposit_refunded: row.get(23)?,
            refund_method: row.get(24)?,
            payment_method: row.get(25)?,
        })
    }) {
        Ok(iter) => iter.filter_map(|r| r.ok()).collect(),
        Err(e) => {
            error!("Failed to query orders: {}", e);
            vec![]
        }
    };
    result
}

pub fn cancel_order(order_id: i64, reason: String) -> Result<Order, String> {
    if order_id <= 0 { return Err("订单ID无效".to_string()); }

    let member_day_discount = get_member_day_discount();
    let conn = DB.lock();

    let (table_id, member_id, start_time, deposit, package_id, package_price, package_hours): (i64, Option<i64>, String, f64, Option<i64>, Option<f64>, Option<f64>) = conn
        .query_row(
            "SELECT table_id, member_id, start_time, deposit, package_id, package_price, package_hours FROM orders WHERE id = ?1 AND status = '进行中'",
            params![order_id],
            |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?, r.get(4)?, r.get(5)?, r.get(6)?)),
        )
        .map_err(|_| "订单不存在或状态不允许取消")?;

    // Fix #13: Use table rate, fallback to area rate, then default 30.0
    let hourly_rate: f64 = conn
        .query_row(
            "SELECT COALESCE(NULLIF(t.rate_per_hour, 0), a.rate_per_hour, 30.0)
             FROM tables t LEFT JOIN areas a ON t.area_id = a.id
             WHERE t.id = ?1",
            params![table_id],
            |r| r.get(0),
        )
        .unwrap_or(30.0);

    let start = DateTime::parse_from_rfc3339(&start_time).map_err(|e| format!("时间解析失败: {}", e))?;
    let now = chrono::Utc::now();
    let duration = (now - start.with_timezone(&Utc)).num_minutes().max(1);
    let bill_min = calc_bill_minutes(duration);

    let mut total = 0.0;
    if let (Some(_), Some(price), Some(hours)) = (package_id, package_price, package_hours) {
        total = price;
        let pkg_duration = (hours * 60.0) as i64;
        if duration > pkg_duration {
            let extra_mins = duration - pkg_duration;
            let extra_bill = calc_bill_minutes(extra_mins);
            let extra_fee = (extra_bill as f64 / 60.0) * hourly_rate;
            total += extra_fee;
        }
    }
    if total == 0.0 {
        total = (bill_min as f64 / 60.0) * hourly_rate;
    }

    let mut discount = 0.0;
    if let Some(mid) = member_id {
        let member_discount: f64 = conn
            .query_row("SELECT discount FROM members WHERE id = ?1", params![mid], |r| r.get(0))
            .unwrap_or(1.0);
        let final_factor = member_discount * (1.0 - (member_day_discount as f64 / 100.0));
        discount = total * (1.0 - final_factor);
    }

    let final_amount = (total - discount).max(0.0);
    let refund_amount = if deposit > final_amount { (deposit - final_amount).max(0.0) } else { 0.0 };
    let now_str = now.to_rfc3339();

    // Fix #3: Atomic status update - only succeeds if status is still '进行中'
    let rows_affected = conn.execute(
        "UPDATE orders SET end_time = ?1, duration_minutes = ?2, total_amount = ?3, discount_amount = ?4,
         final_amount = ?5, cancel_time = ?6, cancel_reason = ?7, deposit_refunded = ?8,
         refund_method = ?9, status = '已取消' WHERE id = ?10 AND status = '进行中'",
        params![
            now_str, duration, total, discount, final_amount,
            now_str, reason, refund_amount,
            if refund_amount > 0.0 { Some("cash") } else { None },
            order_id
        ],
    ).map_err(|e| e.to_string())?;

    if rows_affected == 0 {
        return Err("订单已被其他操作处理，请刷新后重试".to_string());
    }

    conn.execute(
        "UPDATE tables SET status = '空闲' WHERE id = ?1",
        params![table_id],
    ).map_err(|e| e.to_string())?;

    // Fix #1: Actually refund deposit to member balance
    if refund_amount > 0.0 {
        if let Some(mid) = member_id {
            conn.execute(
                "UPDATE members SET balance = balance + ?1 WHERE id = ?2",
                params![refund_amount, mid],
            ).ok();
            conn.execute(
                "INSERT INTO balance_logs (member_id, amount, balance_before, balance_after, reason)
                 SELECT ?1, ?2, balance, balance + ?2, 'cancel_refund' FROM members WHERE id = ?1",
                params![mid, refund_amount],
            ).ok();
        }
    }

    let order = get_order_by_id_locked(&conn, order_id)?;
    drop(conn);
    Ok(order)
}

pub fn check_expired_packages() -> Vec<ExpiredOrderInfo> {
    let conn = DB.lock();
    let mut stmt = match conn.prepare(
        "SELECT o.id, o.table_id, COALESCE(t.name, ''), o.package_name, o.start_time, o.package_hours
         FROM orders o LEFT JOIN tables t ON o.table_id = t.id
         WHERE o.status = '进行中' AND o.package_id IS NOT NULL AND o.package_hours IS NOT NULL"
    ) {
        Ok(s) => s,
        Err(_) => return vec![],
    };
    let result: Vec<ExpiredOrderInfo> = match stmt.query_map([], |row| {
        let start_time: String = row.get(4)?;
        let package_hours: Option<f64> = row.get(5)?;
        let start = DateTime::parse_from_rfc3339(&start_time).ok();
        let duration = if let Some(s) = start {
            (chrono::Utc::now() - s.with_timezone(&Utc)).num_minutes()
        } else { 0 };
        let remaining = if let Some(ph) = package_hours {
            ((ph * 60.0) as i64 - duration).max(0)
        } else { 0 };
        Ok(ExpiredOrderInfo {
            order_id: row.get(0)?,
            table_id: row.get(1)?,
            table_name: row.get(2)?,
            package_name: row.get(3)?,
            start_time,
            duration_minutes: duration,
            package_hours,
            remaining_minutes: remaining,
        })
    }) {
        Ok(iter) => iter.filter_map(|r| r.ok()).collect(),
        Err(_) => vec![],
    };
    result.into_iter().filter(|info| {
        let pkg_hours = info.package_hours.unwrap_or(0.0);
        info.duration_minutes as f64 > pkg_hours * 60.0
    }).collect()
}

pub fn auto_close_expired(order_ids: Vec<i64>) -> Vec<Result<Order, String>> {
    let member_day_discount = get_member_day_discount();
    let mut conn = DB.lock();
    let tx = match conn.transaction() {
        Ok(t) => t,
        Err(e) => return vec![Err(format!("Transaction failed: {}", e))],
    };
    
    let results: Vec<Result<Order, String>> = order_ids.iter().map(|id| {
        let req = CloseTableRequest { payment_method: Some("auto".to_string()), discount_amount: None };
        close_order_with_conn(&tx, *id, req, member_day_discount)
    }).collect();
    
    if results.iter().all(|r| r.is_ok()) {
        if let Err(e) = tx.commit() {
            vec![Err(format!("Commit failed: {}", e))]
        } else {
            results
        }
    } else {
        tx.rollback().ok();
        results
    }
}

pub fn realtime_check(minutes: i32) -> Vec<RealtimeCheckResult> {
    let conn = DB.lock();
    let mut stmt = match conn.prepare(
        "SELECT o.id, COALESCE(t.name, ''), o.start_time, o.total_amount
         FROM orders o LEFT JOIN tables t ON o.table_id = t.id
         WHERE o.status = '进行中'"
    ) {
        Ok(s) => s,
        Err(_) => return vec![],
    };
    let result: Vec<RealtimeCheckResult> = match stmt.query_map([], |row| {
        let start_time: String = row.get(2)?;
        let start = DateTime::parse_from_rfc3339(&start_time).ok();
        let duration = if let Some(s) = start { (chrono::Utc::now() - s.with_timezone(&Utc)).num_minutes() } else { 0 };
        let status = if duration > minutes as i64 { "超时".to_string() } else { "正常".to_string() };
        Ok(RealtimeCheckResult {
            order_id: row.get(0)?,
            table_name: row.get(1)?,
            duration_minutes: duration,
            current_amount: row.get(3)?,
            status,
        })
    }) {
        Ok(iter) => iter.filter_map(|r| r.ok()).collect(),
        Err(_) => vec![],
    };
    result
}

pub fn auto_close_exhausted() -> Vec<AutoCloseResult> {
    let settings = load_settings();
    if !settings.auto_close.enabled {
        return vec![];
    }

    let conn = DB.lock();
    let mut stmt = match conn.prepare(
        "SELECT o.id, o.table_id, COALESCE(t.name, ''), o.member_id, o.deposit, o.package_id, o.package_price, o.package_hours, o.start_time
         FROM orders o LEFT JOIN tables t ON o.table_id = t.id
         WHERE o.status = '进行中'"
    ) {
        Ok(s) => s,
        Err(_) => return vec![],
    };

    let orders: Vec<(i64, i64, String, Option<i64>, f64, Option<i64>, Option<f64>, Option<f64>, String)> = match stmt.query_map([], |row| {
        Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?, row.get(4)?, row.get(5)?, row.get(6)?, row.get(7)?, row.get(8)?))
    }) {
        Ok(iter) => iter.filter_map(|r| r.ok()).collect(),
        Err(_) => return vec![],
    };
    drop(stmt);

    let mut results = vec![];
    for (order_id, table_id, table_name, member_id, deposit, _pkg_id, _pkg_price, _pkg_hours, start_time) in orders {
        let start = match DateTime::parse_from_rfc3339(&start_time) {
            Ok(s) => s,
            Err(_) => continue,
        };
        let duration = (chrono::Utc::now() - start.with_timezone(&Utc)).num_minutes();

        let hourly_rate: f64 = conn
            .query_row(
                "SELECT COALESCE(NULLIF(t.rate_per_hour, 0), a.rate_per_hour, 30.0) FROM tables t LEFT JOIN areas a ON t.area_id = a.id WHERE t.id = ?1",
                params![table_id],
                |r| r.get(0),
            )
            .unwrap_or(30.0);

        let bill_min = calc_bill_minutes(duration.max(1));
        let total = (bill_min as f64 / 60.0) * hourly_rate;

        let mut available = deposit;
        if let Some(mid) = member_id {
            let balance: f64 = conn
                .query_row("SELECT balance FROM members WHERE id = ?1", params![mid], |r| r.get(0))
                .unwrap_or(0.0);
            available += balance.max(0.0);
        }

        if available < total {
            let req = CloseTableRequest { payment_method: Some("auto".to_string()), discount_amount: None };
            match close_order_with_conn(&conn, order_id, req, 0) {
                Ok(_order) => {
                    info!("[AutoClose] Order {} closed: balance/deposit exhausted", order_id);
                    results.push(AutoCloseResult {
                        order_id,
                        table_name,
                        reason: "余额/押金不足".to_string(),
                        total_amount: total,
                        available_amount: available,
                    });
                }
                Err(e) => {
                    warn!("[AutoClose] Failed to close order {}: {}", order_id, e);
                }
            }
        }
    }
    results
}

/// 结账成功后自动打印小票
fn auto_print_receipt(order: &Order) {
    // 从设置中读取是否启用自动打印
    let settings = load_settings();
    let auto_print = settings.auto_print_on_close.unwrap_or(true);
    if !auto_print {
        info!("auto_print_on_close disabled, skipping print");
        return;
    }
    
    let shop_name = settings.shop_name.clone().unwrap_or_else(|| "台球厅".to_string());
    let payment_method_label = match order.payment_method.as_deref() {
        Some("cash") => "现金",
        Some("wechat") => "微信",
        Some("alipay") => "支付宝",
        Some("member") => "会员余额",
        Some("auto") => "自动结账",
        Some(other) => other,
        None => "其他",
    };
    
    let print_req = PrintReceiptRequest {
        printer_id: None, // 使用默认打印机
        shop_name,
        order_no: Some(format!("{}", order.id)),
        table_name: order.table_name.clone(),
        member_name: if order.member_name == "散客" { None } else { Some(order.member_name.clone()) },
        start_time: order.start_time.clone().map(|t| t.replace('T', " ").chars().take(16).collect()),
        end_time: order.end_time.clone().map(|t| t.replace('T', " ").chars().take(16).collect()),
        duration_minutes: order.duration_minutes,
        items: None,
        total_amount: order.total_amount,
        discount_amount: if order.discount_amount > 0.0 { Some(order.discount_amount) } else { None },
        deposit: if order.deposit > 0.0 { Some(order.deposit) } else { None },
        final_amount: order.final_amount,
        payment_method: Some(payment_method_label.to_string()),
        receipt_type: "normal".to_string(),
    };
    
    match print_receipt(print_req) {
        result if result.success => info!("Auto-print receipt for order {} succeeded", order.id),
        result => warn!("Auto-print receipt for order {} failed: {}", order.id, result.message),
    }
}

#[derive(Debug, serde::Serialize)]
pub struct AutoCloseResult {
    pub order_id: i64,
    pub table_name: String,
    pub reason: String,
    pub total_amount: f64,
    pub available_amount: f64,
}
