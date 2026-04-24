use crate::lib::db::DB;
use crate::lib::models::*;
use crate::lib::services::settings::load_settings;
use crate::lib::utils::{calc_bill_minutes_with_params, validate_open_table_request, validate_close_table_request, today_local, BillingParams, calc_extra_minutes, calc_extra_minutes_with_params, round_to_two};
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
                o.total_amount, o.discount_amount, o.deposit, o.deposit_payment_method, o.change_amount, o.final_amount, o.status,
                o.package_id, o.package_name, o.package_price, o.package_hours, o.last_deduction_time,
                o.cancel_time, o.cancel_reason, o.deposit_refunded, o.refund_method, o.payment_method,
                COALESCE(o.deposit, 0) - COALESCE(o.final_amount, 0)
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
                deposit_payment_method: row.get(13)?,
                change_amount: row.get(14)?,
                final_amount: row.get(15)?,
                status: row.get(16)?,
                package_id: row.get(17)?,
                package_name: row.get(18)?,
                package_price: row.get(19)?,
                package_hours: row.get(20)?,
                last_deduction_time: row.get(21)?,
                cancel_time: row.get(22)?,
                cancel_reason: row.get(23)?,
                deposit_refunded: row.get(24)?,
                refund_method: row.get(25)?,
                payment_method: row.get(26)?,
                deposit_change: row.get(27)?,
            })
        },
    ).map_err(|e| e.to_string())
}

fn get_order_by_id_tx(tx: &rusqlite::Transaction, order_id: i64) -> Result<Order, String> {
    tx.query_row(
        "SELECT o.id, o.table_id, t.name, o.member_id, COALESCE(m.name, o.customer_name, '散客'), 
                o.customer_name, o.customer_phone, o.start_time, o.end_time, o.duration_minutes,
                o.total_amount, o.discount_amount, o.deposit, o.deposit_payment_method, o.change_amount, o.final_amount, o.status,
                o.package_id, o.package_name, o.package_price, o.package_hours, o.last_deduction_time,
                o.cancel_time, o.cancel_reason, o.deposit_refunded, o.refund_method, o.payment_method,
                COALESCE(o.deposit, 0) - COALESCE(o.final_amount, 0)
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
                deposit_payment_method: row.get(13)?,
                change_amount: row.get(14)?,
                final_amount: row.get(15)?,
                status: row.get(16)?,
                package_id: row.get(17)?,
                package_name: row.get(18)?,
                package_price: row.get(19)?,
                package_hours: row.get(20)?,
                last_deduction_time: row.get(21)?,
                cancel_time: row.get(22)?,
                cancel_reason: row.get(23)?,
                deposit_refunded: row.get(24)?,
                refund_method: row.get(25)?,
                payment_method: row.get(26)?,
                deposit_change: row.get(27)?,
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

        if let Some(member_id) = req.member_id {
            let balance: f64 = tx
                .query_row("SELECT balance FROM members WHERE id = ?1", params![member_id], |r| r.get(0))
                .unwrap_or(0.0);
            // P1 #3 修复: 余额 < 0 才报错，余额为 0 时可开台（需交押金）
            if balance < 0.0 {
                tx.rollback().ok();
                return Err("会员余额不足，无法开台".to_string());
            }
            // 套餐模式: 检查余额是否足够支付套餐价格
            if let Some(price) = package_price {
                if balance < price {
                    tx.rollback().ok();
                    return Err(format!("套餐价格 ¥{:.2}，会员余额不足", price));
                }
            } else {
                // 非套餐模式: 检查余额 + 押金是否足够支付最低消费预估
                let min_hours: i32 = tx
                    .query_row("SELECT min_hours FROM tables WHERE id = ?1", params![req.table_id], |r| r.get(0))
                    .unwrap_or(0);
                if min_hours > 0 {
                    let hourly_rate: f64 = tx
                        .query_row(
                            "SELECT COALESCE(NULLIF(t.rate_per_hour, 0), a.rate_per_hour, 30.0)
                             FROM tables t LEFT JOIN areas a ON t.area_id = a.id WHERE t.id = ?1",
                            params![req.table_id],
                            |r| r.get(0),
                        )
                        .unwrap_or(30.0);
                    let min_amount = (min_hours as f64) * hourly_rate;
                    let available = balance + req.deposit.unwrap_or(0.0);
                    if available < min_amount {
                        tx.rollback().ok();
                        return Err(format!("预估最低消费 ¥{:.2}，余额+押金不足", min_amount));
                    }
                }
            }
        } else {
            let min_hours: i32 = tx
                .query_row("SELECT min_hours FROM tables WHERE id = ?1", params![req.table_id], |r| r.get(0))
                .unwrap_or(0);
            if min_hours > 0 {
                let hourly_rate: f64 = tx
                    .query_row(
                        "SELECT COALESCE(NULLIF(t.rate_per_hour, 0), a.rate_per_hour, 30.0)
                         FROM tables t LEFT JOIN areas a ON t.area_id = a.id WHERE t.id = ?1",
                        params![req.table_id],
                        |r| r.get(0),
                    )
                    .unwrap_or(30.0);
                let min_amount = (min_hours as f64) * hourly_rate;
                let deposit = req.deposit.unwrap_or(0.0);
                if deposit < min_amount {
                    tx.rollback().ok();
                    return Err(format!("最低消费 ¥{:.2}，押金不足", min_amount));
                }
            }
        }

        let now = chrono::Utc::now().to_rfc3339();

        if let Err(e) = tx.execute(
            "INSERT INTO orders (table_id, member_id, customer_name, customer_phone, deposit, deposit_payment_method, package_id, package_name, package_price, package_hours, status, start_time) 
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, '进行中', ?11)",
            params![
                req.table_id,
                req.member_id,
                req.customer_name,
                req.customer_phone,
                req.deposit.unwrap_or(0.0),
                req.deposit_payment_method,
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
                o.total_amount, o.discount_amount, o.deposit, o.deposit_payment_method, o.change_amount, o.final_amount, o.status,
                o.package_id, o.package_name, o.package_price, o.package_hours, o.last_deduction_time,
                o.cancel_time, o.cancel_reason, o.deposit_refunded, o.refund_method, o.payment_method,
                COALESCE(o.deposit_change, 0)
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
            deposit_payment_method: row.get(13)?,
            change_amount: row.get(14)?,
            final_amount: row.get(15)?,
            status: row.get(16)?,
            package_id: row.get(17)?,
            package_name: row.get(18)?,
            package_price: row.get(19)?,
            package_hours: row.get(20)?,
            last_deduction_time: row.get(21)?,
            cancel_time: row.get(22)?,
            cancel_reason: row.get(23)?,
            deposit_refunded: row.get(24)?,
            refund_method: row.get(25)?,
            payment_method: row.get(26)?,
            deposit_change: row.get(27)?,
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
    
    let settings = load_settings();
    let member_day_discount = get_member_day_discount();
    let billing_params = BillingParams::new(
        settings.billing_rules.free_minutes,
        settings.billing_rules.billing_interval,
        settings.billing_rules.apply_rounding,
    );
    
    let mut conn = DB.lock();
    // P2 #18 修复: 直接创建事务并调用 close_order_with_tx
    let tx = conn.transaction().map_err(|e| format!("Transaction failed: {}", e))?;
    
    let result = close_order_with_tx(&tx, order_id, req, member_day_discount, &billing_params);
    
    if result.is_ok() {
        if let Err(e) = tx.commit() {
            return Err(format!("Commit failed: {}", e));
        }
    } else {
        tx.rollback().ok();
    }
    
    // 结账成功后自动打印小票（非阻塞）
    if let Ok(ref order) = result {
        let order_clone = order.clone();
        std::thread::spawn(move || {
            if let Err(e) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| auto_print_receipt(&order_clone))) {
                error!("Auto-print panic: {:?}", e);
            }
        });
    }
    
    result
}

fn close_order_with_tx(tx: &rusqlite::Transaction, order_id: i64, req: CloseTableRequest, member_day_discount: i32, billing_params: &BillingParams) -> Result<Order, String> {

    let (table_id, member_id, start_time, deposit, package_id, package_price, package_hours): (i64, Option<i64>, String, f64, Option<i64>, Option<f64>, Option<f64>) = tx
        .query_row(
            "SELECT table_id, member_id, start_time, deposit, package_id, package_price, package_hours FROM orders WHERE id = ?1 AND status = '进行中'",
            params![order_id],
            |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?, r.get(4)?, r.get(5)?, r.get(6)?)),
        )
        .map_err(|_| "订单不存在或已结账")?;

    let hourly_rate: f64 = tx
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
    let bill_min = calc_bill_minutes_with_params(duration, &billing_params);

    let mut total = 0.0;
    if let (Some(_), Some(price), Some(hours)) = (package_id, package_price, package_hours) {
        total = price;
        let pkg_duration = (hours * 60.0) as i64;
        if duration > pkg_duration {
            let extra_mins = duration - pkg_duration;
            let extra_params = BillingParams::new(0, billing_params.billing_interval, billing_params.apply_rounding);
            let extra_fee = calc_extra_minutes_with_params(extra_mins, hourly_rate, &extra_params);
            total += extra_fee;
        }
    }
    if total == 0.0 {
        total = (bill_min as f64 / 60.0) * hourly_rate;
    }

    let mut discount = 0.0;
    let payment_method = req.payment_method.unwrap_or_else(|| "cash".to_string());

    if let Some(mid) = member_id {
        let member_discount: f64 = tx
            .query_row(
                "SELECT discount FROM members WHERE id = ?1",
                params![mid],
                |r| r.get(0),
            )
            .unwrap_or(1.0);

        // P1 #2 + P1 #12 修复: 取更优策略 + 折扣限制
        let effective_member_discount = member_discount.clamp(0.05, 1.0);
        let member_day_factor = 1.0 - (member_day_discount as f64 / 100.0);
        let final_factor = effective_member_discount.min(member_day_factor).max(0.05).min(1.0);
        discount = total * (1.0 - final_factor);
    }

    let final_amount = (total - discount).max(0.0);
    let change_amount = if deposit > final_amount { deposit - final_amount } else { 0.0 };
    // P0 #1 修复: final_amount 存应收总额 (total - discount)，paid_amount 存实收净额
    let paid_amount = (final_amount - deposit).max(0.0);
    let deposit_change = if deposit > final_amount { deposit - final_amount } else { 0.0 };

    let mut balance_paid = 0.0;
    if let Some(mid) = member_id {
        let member_balance: f64 = tx
            .query_row("SELECT balance FROM members WHERE id = ?1", params![mid], |r| r.get(0))
            .unwrap_or(0.0);
        balance_paid = member_balance.max(0.0).min(paid_amount);
    }

    if balance_paid > 0.0 {
        if let Some(mid) = member_id {
            // P0 #4 修复: 先读取 balance_before，再 UPDATE，再计算 balance_after
            let balance_before: f64 = tx
                .query_row("SELECT balance FROM members WHERE id = ?1", params![mid], |r| r.get(0))
                .unwrap_or(0.0);
            tx.execute(
                "UPDATE members SET balance = balance - ?1 WHERE id = ?2",
                params![balance_paid, mid],
            ).map_err(|e| format!("扣减余额失败: {}", e))?;
            let balance_after = balance_before - balance_paid;
            tx.execute(
                "INSERT INTO balance_logs (member_id, order_id, amount, balance_before, balance_after, reason, payment_method)
                 VALUES (?1, ?2, ?3, ?4, ?5, 'order', ?6)",
                params![mid, order_id, -balance_paid, balance_before, balance_after, &payment_method],
            ).ok();
            // P1 #10 修复: total_spent 累加实际消费总额 (final_amount)，而非仅余额扣减部分
            tx.execute(
                "UPDATE members SET total_spent = total_spent + ?1 WHERE id = ?2",
                params![final_amount, mid],
            ).ok();
        }
    }

    let now_str = now.to_rfc3339();
    // P0 #1 修复: final_amount 存应收总额 (total - discount)
    let rows_affected = tx.execute(
        "UPDATE orders SET end_time = ?1, duration_minutes = ?2, total_amount = ?3, discount_amount = ?4,
         change_amount = ?5, final_amount = ?6, payment_method = ?7, deposit_change = ?8, status = '已结账' WHERE id = ?9 AND status = '进行中'",
        params![
            now_str,
            duration,
            total,
            discount,
            change_amount,
            final_amount,
            payment_method,
            deposit_change,
            order_id
        ],
    ).map_err(|e| e.to_string())?;

    if rows_affected == 0 {
        return Err("订单已被其他操作处理，请刷新后重试".to_string());
    }

    tx.execute(
        "UPDATE tables SET status = '空闲' WHERE id = ?1",
        params![table_id],
    ).map_err(|e| e.to_string())?;

    // P0 #1 + P1 #11 修复: revenues 记录应收总额 (final_amount)，净额由 paid_amount 表示
    if paid_amount > 0.0 {
        let today = today_local();
        if let Err(e) = tx.execute(
            "INSERT INTO revenues (type, amount, payment_method, table_id, order_id, member_id, date) VALUES ('order', ?1, ?2, ?3, ?4, ?5, ?6)",
            params![paid_amount, payment_method, table_id, order_id, member_id, today],
        ) {
            warn!("Failed to record revenue: {}", e);
        }
    }

    get_order_by_id_tx(tx, order_id)
}

pub fn get_orders(status: Option<String>) -> Vec<Order> {
    let conn = DB.lock();
    let query = "SELECT o.id, o.table_id, t.name, o.member_id, COALESCE(m.name, o.customer_name, '散客'), 
                    o.customer_name, o.customer_phone, o.start_time, o.end_time, o.duration_minutes,
                    o.total_amount, o.discount_amount, o.deposit, o.deposit_payment_method, o.change_amount, o.final_amount, o.status,
                    o.package_id, o.package_name, o.package_price, o.package_hours, o.last_deduction_time,
                    o.cancel_time, o.cancel_reason, o.deposit_refunded, o.refund_method, o.payment_method,
                    COALESCE(o.deposit, 0) - COALESCE(o.final_amount, 0)
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
            deposit_payment_method: row.get(13)?,
            change_amount: row.get(14)?,
            final_amount: row.get(15)?,
            status: row.get(16)?,
            package_id: row.get(17)?,
            package_name: row.get(18)?,
            package_price: row.get(19)?,
            package_hours: row.get(20)?,
            last_deduction_time: row.get(21)?,
            cancel_time: row.get(22)?,
            cancel_reason: row.get(23)?,
            deposit_refunded: row.get(24)?,
            refund_method: row.get(25)?,
            payment_method: row.get(26)?,
            deposit_change: row.get(27)?,
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

    let settings = load_settings();
    let member_day_discount = get_member_day_discount();
    let billing_params = BillingParams::new(
        settings.billing_rules.free_minutes,
        settings.billing_rules.billing_interval,
        settings.billing_rules.apply_rounding,
    );
    let mut conn = DB.lock();
    let tx = conn.transaction().map_err(|e| format!("事务创建失败: {}", e))?;
    
    let (table_id, member_id, start_time, deposit, package_id, package_price, package_hours): (i64, Option<i64>, String, f64, Option<i64>, Option<f64>, Option<f64>) = tx
        .query_row(
            "SELECT table_id, member_id, start_time, deposit, package_id, package_price, package_hours FROM orders WHERE id = ?1 AND status = '进行中'",
            params![order_id],
            |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?, r.get(4)?, r.get(5)?, r.get(6)?)),
        )
        .map_err(|_| "订单不存在或状态不允许取消")?;

    let hourly_rate: f64 = tx
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
    let bill_min = calc_bill_minutes_with_params(duration, &billing_params);

    let mut total = 0.0;
    if let (Some(_), Some(price), Some(hours)) = (package_id, package_price, package_hours) {
        total = price;
        let pkg_duration = (hours * 60.0) as i64;
        if duration > pkg_duration {
            let extra_mins = duration - pkg_duration;
            let extra_params = BillingParams::new(0, billing_params.billing_interval, billing_params.apply_rounding);
            let extra_fee = calc_extra_minutes_with_params(extra_mins, hourly_rate, &extra_params);
            total += extra_fee;
        }
    }
    if total == 0.0 {
        total = (bill_min as f64 / 60.0) * hourly_rate;
    }

    let mut discount = 0.0;
    if let Some(mid) = member_id {
        let member_discount: f64 = tx
            .query_row("SELECT discount FROM members WHERE id = ?1", params![mid], |r| r.get(0))
            .unwrap_or(1.0);
        // P1 #2 + P1 #12 修复: 取更优策略 + 折扣限制
        let effective_member_discount = member_discount.clamp(0.05, 1.0);
        let member_day_factor = 1.0 - (member_day_discount as f64 / 100.0);
        let final_factor = effective_member_discount.min(member_day_factor).max(0.05).min(1.0);
        discount = total * (1.0 - final_factor);
    }

    let final_amount = (total - discount).max(0.0);
    let refund_amount = if deposit > final_amount { (deposit - final_amount).max(0.0) } else { 0.0 };
    let deposit_change = refund_amount;
    let now_str = now.to_rfc3339();

    let rows_affected = tx.execute(
        "UPDATE orders SET end_time = ?1, duration_minutes = ?2, total_amount = ?3, discount_amount = ?4,
         final_amount = ?5, cancel_time = ?6, cancel_reason = ?7, deposit_refunded = ?8, deposit_change = ?9,
         refund_method = ?10, status = '已取消' WHERE id = ?11 AND status = '进行中'",
        params![
            now_str, duration, total, discount, final_amount,
            now_str, reason, refund_amount, deposit_change,
            if refund_amount > 0.0 { Some("cash") } else { None },
            order_id
        ],
    ).map_err(|e| e.to_string())?;

    if rows_affected == 0 {
        tx.rollback().ok();
        return Err("订单已被其他操作处理，请刷新后重试".to_string());
    }

    tx.execute(
        "UPDATE tables SET status = '空闲' WHERE id = ?1",
        params![table_id],
    ).map_err(|e| e.to_string())?;

    if refund_amount > 0.0 {
        if let Some(mid) = member_id {
            // P0 #4 修复: 先读取 balance_before，再 UPDATE，再计算 balance_after
            let balance_before: f64 = tx
                .query_row("SELECT balance FROM members WHERE id = ?1", params![mid], |r| r.get(0))
                .unwrap_or(0.0);
            tx.execute(
                "UPDATE members SET balance = balance + ?1 WHERE id = ?2",
                params![refund_amount, mid],
            ).map_err(|e| e.to_string())?;
            let balance_after = balance_before + refund_amount;
            // P1 #6 修复: 使用显式参数而非 SELECT 子查询，确保记录正确的 balance_before 和 balance_after
            tx.execute(
                "INSERT INTO balance_logs (member_id, amount, balance_before, balance_after, reason)
                 VALUES (?1, ?2, ?3, ?4, 'cancel_refund')",
                params![mid, refund_amount, balance_before, balance_after],
            ).ok();
        }
    }

    tx.commit().map_err(|e| e.to_string())?;
    drop(conn);
    Ok(get_order_by_id(order_id)?)
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
    let settings = load_settings();
    let member_day_discount = get_member_day_discount();
    let billing_params = BillingParams::new(
        settings.billing_rules.free_minutes,
        settings.billing_rules.billing_interval,
        settings.billing_rules.apply_rounding,
    );
    let mut conn = DB.lock();
    let tx = match conn.transaction() {
        Ok(t) => t,
        Err(e) => return vec![Err(format!("Transaction failed: {}", e))],
    };
    
    let results: Vec<Result<Order, String>> = order_ids.iter().map(|id| {
        let req = CloseTableRequest { payment_method: Some("auto".to_string()), discount_amount: None };
        close_order_with_tx(&tx, *id, req, member_day_discount, &billing_params)
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
    let settings = load_settings();
    let member_day_discount = get_member_day_discount();
    let _billing_params = BillingParams::new(
        settings.billing_rules.free_minutes,
        settings.billing_rules.billing_interval,
        settings.billing_rules.apply_rounding,
    );
    let conn = DB.lock();
    let mut stmt = match conn.prepare(
        "SELECT o.id, COALESCE(t.name, ''), o.start_time
         FROM orders o LEFT JOIN tables t ON o.table_id = t.id
         WHERE o.status = '进行中'"
    ) {
        Ok(s) => s,
        Err(_) => return vec![],
    };
    let result: Vec<RealtimeCheckResult> = match stmt.query_map([], |row| {
        let order_id: i64 = row.get(0)?;
        let start_time: String = row.get(2)?;
        
        let start = DateTime::parse_from_rfc3339(&start_time).ok();
        let duration = if let Some(s) = start { (chrono::Utc::now() - s.with_timezone(&Utc)).num_minutes() } else { 0 };
        
        let table_id: i64 = conn.query_row(
            "SELECT table_id FROM orders WHERE id = ?1", params![order_id], |r| r.get(0)
        ).unwrap_or(0);
        
        let hourly_rate: f64 = conn.query_row(
            "SELECT COALESCE(NULLIF(t.rate_per_hour, 0), a.rate_per_hour, 30.0) FROM tables t LEFT JOIN areas a ON t.area_id = a.id WHERE t.id = ?1",
            params![table_id], |r| r.get(0)
        ).unwrap_or(30.0);
        
        let (current_amount, _, _, _) = calc_order_billing(&conn, order_id, duration.max(1), hourly_rate, member_day_discount)
            .unwrap_or((0.0, 0.0, 0.0, 0));
        
        let status = if duration > minutes as i64 { "超时".to_string() } else { "正常".to_string() };
        
        Ok(RealtimeCheckResult {
            order_id,
            table_name: row.get(1)?,
            duration_minutes: duration,
            current_amount,
            status,
        })
    }) {
        Ok(iter) => iter.filter_map(|r| r.ok()).collect(),
        Err(_) => vec![],
    };
    result
}

/// 统一计费：返回 (总金额, 折扣, 实付净额, bill_minutes)
fn calc_order_billing(conn: &Connection, order_id: i64, duration: i64, hourly_rate: f64, member_day_discount: i32) -> Result<(f64, f64, f64, i64), String> {
    let (member_id, package_id, package_price, package_hours): (Option<i64>, Option<i64>, Option<f64>, Option<f64>) = conn
        .query_row(
            "SELECT member_id, package_id, package_price, package_hours FROM orders WHERE id = ?1",
            params![order_id],
            |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?)),
        )
        .map_err(|e| e.to_string())?;

    let settings = load_settings();
    let billing_params = BillingParams::new(
        settings.billing_rules.free_minutes,
        settings.billing_rules.billing_interval,
        settings.billing_rules.apply_rounding,
    );
    let bill_min = calc_bill_minutes_with_params(duration, &billing_params);

    let mut total = 0.0;
    if let (Some(_), Some(price), Some(hours)) = (package_id, package_price, package_hours) {
        total = price;
        let pkg_duration = (hours * 60.0) as i64;
        if duration > pkg_duration {
            let extra_mins = duration - pkg_duration;
            let extra_params = BillingParams::new(0, billing_params.billing_interval, billing_params.apply_rounding);
            let extra_fee = calc_extra_minutes_with_params(extra_mins, hourly_rate, &extra_params);
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
        // P1 #2 + P1 #12 修复: 取更优策略 + 折扣限制
        let effective_member_discount = member_discount.clamp(0.05, 1.0);
        let member_day_factor = 1.0 - (member_day_discount as f64 / 100.0);
        let final_factor = effective_member_discount.min(member_day_factor).max(0.05).min(1.0);
        discount = total * (1.0 - final_factor);
    }

    let net = (total - discount).max(0.0);
    Ok((total, discount, net, bill_min))
}

/// 获取所有活跃订单的实时计费状态（供前端轮询）
pub fn get_realtime_billing_status() -> Vec<RealtimeBillingStatus> {
    let member_day_discount = get_member_day_discount();
    let conn = DB.lock();

    let mut stmt = match conn.prepare(
        "SELECT o.id, o.table_id, COALESCE(t.name, ''), o.member_id, COALESCE(m.name, '散客'),
                o.start_time, o.deposit, o.package_name, o.package_hours
         FROM orders o
         LEFT JOIN tables t ON o.table_id = t.id
         LEFT JOIN members m ON o.member_id = m.id
         WHERE o.status = '进行中'"
    ) {
        Ok(s) => s,
        Err(_) => return vec![],
    };

    let orders: Vec<(i64, i64, String, Option<i64>, String, String, f64, Option<String>, Option<f64>)> =
        match stmt.query_map([], |row| {
            Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?, row.get(4)?,
                   row.get(5)?, row.get(6)?, row.get(7)?, row.get(8)?))
        }) {
            Ok(iter) => iter.filter_map(|r| r.ok()).collect(),
            Err(_) => return vec![],
        };
    drop(stmt);

    orders.into_iter().map(|(order_id, table_id, table_name, member_id, member_name,
                             start_time, deposit, package_name, package_hours)| {
        let start = DateTime::parse_from_rfc3339(&start_time).ok();
        let duration = start.map(|s| (chrono::Utc::now() - s.with_timezone(&Utc)).num_minutes().max(1)).unwrap_or(1);
        let hourly_rate: f64 = conn
        .query_row(
            "SELECT COALESCE(NULLIF(t.rate_per_hour, 0), a.rate_per_hour, 30.0)
             FROM tables t LEFT JOIN areas a ON t.area_id = a.id
             WHERE t.id = ?1",
            params![table_id],
            |r| r.get(0),
        )
        .unwrap_or(30.0);

        let (current_amount, discount_amount, net_amount, bill_min) =
            calc_order_billing(&conn, order_id, duration, hourly_rate, member_day_discount)
                .unwrap_or((0.0, 0.0, 0.0, 0));

        let member_balance: f64 = member_id
            .and_then(|mid| conn.query_row("SELECT balance FROM members WHERE id = ?1", params![mid], |r| r.get(0)).ok())
            .unwrap_or(0.0);

        let available = deposit + member_balance.max(0.0);

        // 计算剩余可用分钟数
        let remaining_minutes = if hourly_rate > 0.0 {
            let remaining = available - net_amount;
            if remaining <= 0.0 { 0 } else { ((remaining / (hourly_rate / 60.0)) as i64).max(0) }
        } else { 999 };

        // 套餐剩余分钟数
        let package_remaining_minutes = if let Some(ph) = package_hours {
            ((ph * 60.0) as i64 - duration).max(0)
        } else { -1 };

        // 预警级别
        let (warning_level, remaining_warning) = if remaining_minutes == 0 {
            ("exhausted".to_string(), "余额已耗尽".to_string())
        } else if remaining_minutes <= 5 {
            ("critical".to_string(), format!("仅剩 {} 分钟", remaining_minutes))
        } else if remaining_minutes <= 10 {
            ("low".to_string(), format!("余额不足，约剩 {} 分钟", remaining_minutes))
        } else {
            ("normal".to_string(), format!("约剩 {} 分钟", remaining_minutes))
        };

        RealtimeBillingStatus {
            order_id,
            table_id,
            table_name,
            member_id,
            member_name,
            start_time,
            duration_minutes: duration,
            bill_minutes: bill_min,
            current_amount: round_to_two(current_amount),
            hourly_rate,
            package_name,
            package_remaining_minutes,
            deposit: round_to_two(deposit),
            member_balance: round_to_two(member_balance),
            available: round_to_two(available),
            remaining_minutes,
            remaining_warning,
            warning_level,
            discount_amount: round_to_two(discount_amount),
            net_amount: round_to_two(net_amount),
        }
    }).collect()
}

pub fn auto_close_exhausted() -> Vec<AutoCloseResult> {
    let settings = load_settings();
    if !settings.auto_close.enabled {
        return vec![];
    }

    let billing_params = BillingParams::new(
        settings.billing_rules.free_minutes,
        settings.billing_rules.billing_interval,
        settings.billing_rules.apply_rounding,
    );

    let mut conn = DB.lock();
    let tx = match conn.transaction() {
        Ok(t) => t,
        Err(_) => return vec![],
    };

    let mut stmt = match tx.prepare(
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
    let mut failed_orders = vec![];

    for (order_id, table_id, table_name, member_id, deposit, pkg_id, pkg_price, pkg_hours, start_time) in orders {
        let start = match DateTime::parse_from_rfc3339(&start_time) {
            Ok(s) => s,
            Err(_) => continue,
        };
        let duration = (chrono::Utc::now() - start.with_timezone(&Utc)).num_minutes().max(1);

        let hourly_rate: f64 = tx
            .query_row(
                "SELECT COALESCE(NULLIF(t.rate_per_hour, 0), a.rate_per_hour, 30.0) FROM tables t LEFT JOIN areas a ON t.area_id = a.id WHERE t.id = ?1",
                params![table_id],
                |r| r.get(0),
            )
            .unwrap_or(30.0);

        let mut total = 0.0;
        if let (Some(_), Some(price), Some(hours)) = (pkg_id, pkg_price, pkg_hours) {
            total = price;
            let pkg_duration = (hours * 60.0) as i64;
            if duration > pkg_duration {
                let extra_mins = duration - pkg_duration;
                let extra_params = BillingParams::new(0, billing_params.billing_interval, billing_params.apply_rounding);
            let extra_fee = calc_extra_minutes_with_params(extra_mins, hourly_rate, &extra_params);
                total += extra_fee;
            }
        }
        if total == 0.0 {
            let bill_min = calc_bill_minutes_with_params(duration.max(1), &billing_params);
            total = (bill_min as f64 / 60.0) * hourly_rate;
        }

        let mut available = deposit;
        let mut discount = 0.0;
        if let Some(mid) = member_id {
            let member_discount: f64 = tx
                .query_row(
                    "SELECT discount FROM members WHERE id = ?1",
                    params![mid],
                    |r| r.get(0),
                )
                .unwrap_or(1.0);
            let member_day_discount = get_member_day_discount();
            let final_factor = (member_discount * (1.0 - (member_day_discount as f64 / 100.0))).max(0.0).min(1.0);
            discount = total * (1.0 - final_factor);
            let balance: f64 = tx
                .query_row("SELECT balance FROM members WHERE id = ?1", params![mid], |r| r.get(0))
                .unwrap_or(0.0);
            available += balance.max(0.0);
        }

        let final_total = (total - discount).max(0.0);

        if available < final_total {
            let req = CloseTableRequest { payment_method: Some("auto".to_string()), discount_amount: None };
            match close_order_with_tx(&tx, order_id, req, 0, &billing_params) {
                Ok(_order) => {
                    info!("[AutoClose] Order {} closed: balance/deposit exhausted", order_id);
                    results.push(AutoCloseResult {
                        order_id,
                        table_name,
                        reason: "余额/押金不足".to_string(),
                        total_amount: final_total,
                        available_amount: available,
                    });
                }
                Err(e) => {
                    warn!("[AutoClose] Failed to close order {}: {}", order_id, e);
                    failed_orders.push(order_id);
                }
            }
        }
    }

    if !failed_orders.is_empty() {
        warn!("[AutoClose] Rolling back {} failed orders", failed_orders.len());
        tx.rollback().ok();
        return vec![];
    }

    if let Err(e) = tx.commit() {
        error!("[AutoClose] Commit failed: {}", e);
        return vec![];
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
