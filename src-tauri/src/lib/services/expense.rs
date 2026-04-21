use crate::lib::db::DB;
use crate::lib::models::*;
use crate::lib::utils::today_local;
use rusqlite::params;

pub fn get_expenses(remark: Option<String>) -> Vec<Expense> {
    let conn = DB.lock();
    let query = "SELECT id, type, amount, date, remark, created_at FROM expenses WHERE (?1 IS NULL OR remark LIKE '%' || ?1 || '%') ORDER BY date DESC, id DESC";
    let mut stmt = match conn.prepare(query) { Ok(s) => s, Err(_) => return vec![] };
    let result: Vec<Expense> = match stmt.query_map(params![remark], |row| {
        Ok(Expense { id: row.get(0)?, expense_type: row.get(1)?, amount: row.get(2)?, date: row.get(3)?, remark: row.get(4)?, created_at: row.get(5)? })
    }) { Ok(iter) => iter.filter_map(|r| r.ok()).collect(), Err(_) => vec![] };
    result
}

pub fn create_expense(req: CreateExpenseRequest) -> Result<Expense, String> {
    let conn = DB.lock();
    let date = req.date.unwrap_or_else(|| today_local());
    conn.execute("INSERT INTO expenses (type, amount, date, remark) VALUES (?1, ?2, ?3, ?4)", params![req.expense_type, req.amount, date, req.remark]).map_err(|e| e.to_string())?;
    let id = conn.last_insert_rowid();
    Ok(Expense { id, expense_type: req.expense_type, amount: req.amount, date, remark: req.remark, created_at: Some(chrono::Utc::now().to_rfc3339()) })
}

pub fn update_expense(id: i64, req: CreateExpenseRequest) -> Result<Expense, String> {
    let conn = DB.lock();
    conn.execute("UPDATE expenses SET type = ?1, amount = ?2, remark = ?3 WHERE id = ?4", params![req.expense_type, req.amount, req.remark, id]).map_err(|e| e.to_string())?;
    Ok(Expense { id, expense_type: req.expense_type, amount: req.amount, date: req.date.unwrap_or_else(|| today_local()), remark: req.remark, created_at: None })
}

pub fn delete_expense(id: i64) -> Result<(), String> {
    let conn = DB.lock();
    conn.execute("DELETE FROM expenses WHERE id = ?1", params![id]).map_err(|e| e.to_string())?;
    Ok(())
}
