use std::fmt;

#[derive(Debug)]
pub enum ValidationError {
    EmptyField(&'static str),
    TooLong(&'static str, usize, usize),
    TooShort(&'static str, usize, usize),
    InvalidFormat(&'static str, &'static str),
    OutOfRange(&'static str, f64, f64),
    Negative(&'static str),
    InvalidPhone(String),
    InvalidId(&'static str),
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValidationError::EmptyField(field) => write!(f, "{} 不能为空", field),
            ValidationError::TooLong(field, len, max) => write!(f, "{} 过长 ({} > {})", field, len, max),
            ValidationError::TooShort(field, len, min) => write!(f, "{} 过短 ({} < {})", field, len, min),
            ValidationError::InvalidFormat(field, format) => write!(f, "{} 格式不正确，应为 {}", field, format),
            ValidationError::OutOfRange(field, val, min) => write!(f, "{} 超出范围 ({} < {})", field, val, min),
            ValidationError::Negative(field) => write!(f, "{} 不能为负数", field),
            ValidationError::InvalidPhone(phone) => write!(f, "手机号格式不正确: {}", phone),
            ValidationError::InvalidId(field) => write!(f, "{} 不存在或无效", field),
        }
    }
}

impl From<ValidationError> for String {
    fn from(err: ValidationError) -> Self {
        err.to_string()
    }
}

pub type ValidationResult = Result<(), ValidationError>;

// --- Validation helpers ---

pub fn validate_not_empty(value: &str, field: &'static str) -> ValidationResult {
    if value.trim().is_empty() {
        Err(ValidationError::EmptyField(field))
    } else {
        Ok(())
    }
}

pub fn validate_max_len(value: &str, field: &'static str, max: usize) -> ValidationResult {
    if value.len() > max {
        Err(ValidationError::TooLong(field, value.len(), max))
    } else {
        Ok(())
    }
}

#[allow(dead_code)]
pub fn validate_min_len(value: &str, field: &'static str, min: usize) -> ValidationResult {
    if value.len() < min {
        Err(ValidationError::TooShort(field, value.len(), min))
    } else {
        Ok(())
    }
}

pub fn validate_non_negative(value: f64, field: &'static str) -> ValidationResult {
    if value < 0.0 {
        Err(ValidationError::Negative(field))
    } else {
        Ok(())
    }
}

pub fn validate_range(value: f64, field: &'static str, min: f64, max: f64) -> ValidationResult {
    if value < min || value > max {
        Err(ValidationError::OutOfRange(field, value, min))
    } else {
        Ok(())
    }
}

pub fn validate_phone(phone: &str) -> ValidationResult {
    if phone.is_empty() {
        return Ok(()); // optional
    }
    // Chinese phone: 11 digits starting with 1
    let digits: String = phone.chars().filter(|c| c.is_ascii_digit()).collect();
    if digits.len() == 11 && digits.starts_with('1') {
        Ok(())
    } else {
        Err(ValidationError::InvalidPhone(phone.to_string()))
    }
}

pub fn validate_positive_id(id: i64, field: &'static str) -> ValidationResult {
    if id <= 0 {
        Err(ValidationError::InvalidId(field))
    } else {
        Ok(())
    }
}

// --- Request validators ---

pub fn validate_open_table_request(req: &crate::lib::models::OpenTableRequest) -> ValidationResult {
    validate_positive_id(req.table_id, "桌台ID")?;
    if let Some(ref name) = req.customer_name {
        validate_not_empty(name, "客户姓名")?;
        validate_max_len(name, "客户姓名", 50)?;
    }
    if let Some(ref phone) = req.customer_phone {
        validate_phone(phone)?;
    }
    if let Some(deposit) = req.deposit {
        validate_non_negative(deposit, "押金")?;
    }
    Ok(())
}

pub fn validate_close_table_request(req: &crate::lib::models::CloseTableRequest) -> ValidationResult {
    if let Some(discount) = req.discount_amount {
        validate_range(discount, "折扣金额", 0.0, 1000000.0)?;
    }
    Ok(())
}

pub fn validate_member_name(name: &str) -> ValidationResult {
    validate_not_empty(name, "姓名")?;
    validate_max_len(name, "姓名", 50)?;
    Ok(())
}

pub fn validate_member_phone(phone: &str) -> ValidationResult {
    validate_not_empty(phone, "手机号")?;
    validate_phone(phone)?;
    Ok(())
}

pub fn validate_recharge_amount(amount: f64) -> ValidationResult {
    validate_range(amount, "充值金额", 0.01, 1000000.0)?;
    Ok(())
}

#[allow(dead_code)]
pub fn validate_product_name(name: &str) -> ValidationResult {
    validate_not_empty(name, "商品名称")?;
    validate_max_len(name, "商品名称", 100)?;
    Ok(())
}

#[allow(dead_code)]
pub fn validate_price(price: f64) -> ValidationResult {
    validate_range(price, "价格", 0.0, 1000000.0)?;
    Ok(())
}

#[allow(dead_code)]
pub fn validate_quantity(qty: i32) -> ValidationResult {
    if qty <= 0 {
        Err(ValidationError::OutOfRange("数量", qty as f64, 1.0))
    } else {
        Ok(())
    }
}
