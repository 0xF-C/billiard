use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, Default)]
pub struct BillingParams {
    pub free_minutes: i32,
    pub billing_interval: i32,
    pub apply_rounding: bool,
}

impl BillingParams {
    pub fn new(free_minutes: i32, billing_interval: i32, apply_rounding: bool) -> Self {
        BillingParams { free_minutes, billing_interval, apply_rounding }
    }

    pub fn default_rules() -> Self {
        BillingParams { free_minutes: 5, billing_interval: 30, apply_rounding: true }
    }
}

/// Money type that stores value in cents (i64) to avoid floating point precision issues.
/// Serializes/deserializes as f64 (yuan) for API compatibility.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Money(i64);

#[allow(dead_code)]
impl Money {
    /// Create Money from yuan (f64). Rounds to nearest cent.
    pub fn from_yuan(yuan: f64) -> Self {
        Money((yuan * 100.0).round() as i64)
    }

    /// Get value in yuan (f64)
    pub fn to_yuan(self) -> f64 {
        self.0 as f64 / 100.0
    }

    /// Get value in cents
    pub fn cents(self) -> i64 {
        self.0
    }

    /// Create Money from cents
    pub fn from_cents(cents: i64) -> Self {
        Money(cents)
    }

    /// Add two Money values
    pub fn add(self, other: Money) -> Money {
        Money(self.0 + other.0)
    }

    /// Subtract two Money values
    pub fn sub(self, other: Money) -> Money {
        Money(self.0 - other.0)
    }

    /// Multiply by a scalar
    pub fn mul(self, factor: f64) -> Money {
        Money((self.0 as f64 * factor).round() as i64)
    }

    /// Multiply by integer
    pub fn mul_i64(self, factor: i64) -> Money {
        Money(self.0 * factor)
    }

    /// Apply discount (0.0 - 1.0)
    pub fn apply_discount(self, discount: f64) -> Money {
        let d = discount.max(0.0).min(1.0);
        Money((self.0 as f64 * d).round() as i64)
    }

    /// Is zero?
    pub fn is_zero(self) -> bool {
        self.0 == 0
    }

    /// Is negative?
    pub fn is_negative(self) -> bool {
        self.0 < 0
    }

    /// Absolute value
    pub fn abs(self) -> Money {
        Money(self.0.abs())
    }

    /// Format as ¥X.XX
    pub fn format_yuan(self) -> String {
        format!("¥{:.2}", self.to_yuan())
    }
}

impl Serialize for Money {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_f64(self.to_yuan())
    }
}

impl<'de> Deserialize<'de> for Money {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let yuan = f64::deserialize(deserializer)?;
        Ok(Money::from_yuan(yuan))
    }
}

/// Safely round a f64 to 2 decimal places (legacy compatibility)
pub fn round_to_two(v: f64) -> f64 {
    Money::from_yuan(v).to_yuan()
}

/// Calculate bill amount from duration and hourly rate
#[allow(dead_code)]
pub fn calc_bill_amount(duration_minutes: i64, rate_per_hour: f64) -> Money {
    let billable_minutes = calc_bill_minutes(duration_minutes);
    let rate_per_minute = Money::from_yuan(rate_per_hour).to_yuan() / 60.0;
    Money::from_yuan(billable_minutes as f64 * rate_per_minute)
}

/// Calculate billable minutes with configurable rounding rules.
/// freeMinutes: first N minutes are free.
/// billingInterval: minimum billing unit (30 means 0-29 min rounds to 30).
/// applyRounding: if false, charges per actual minute (no rounding).
pub fn calc_bill_minutes(duration: i64) -> i64 {
    calc_bill_minutes_with_params(duration, &BillingParams::default_rules())
}

pub fn calc_bill_minutes_with_params(duration: i64, params: &BillingParams) -> i64 {
    let duration = duration.max(0);
    if !params.apply_rounding {
        return (duration - params.free_minutes as i64).max(0);
    }
    let free = params.free_minutes.max(0) as i64;
    let interval = params.billing_interval.max(1) as i64;
    let grace = (params.billing_interval.max(1) / 6).max(1) as i64;
    if duration < free {
        return 0;
    }
    let billable = duration - free;
    if billable == 0 {
        return 0;
    }
    if billable < interval {
        return free + interval;
    }
    let full_units = billable / interval;
    let rem = billable % interval;
    let rounded_units = if rem >= grace { full_units + 1 } else { full_units };
    free + rounded_units * interval
}

pub fn calc_extra_minutes(extra_mins: i64, hourly_rate: f64) -> f64 {
    calc_extra_minutes_with_params(extra_mins, hourly_rate, &BillingParams::default_rules())
}

pub fn calc_extra_minutes_with_params(extra_mins: i64, hourly_rate: f64, params: &BillingParams) -> f64 {
    if extra_mins <= 0 {
        return 0.0;
    }
    let bill_mins = calc_bill_minutes_with_params(extra_mins, params);
    (bill_mins as f64 / 60.0) * hourly_rate
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_money_from_yuan() {
        assert_eq!(Money::from_yuan(1.00).cents(), 100);
        assert_eq!(Money::from_yuan(1.999).cents(), 200);
        assert_eq!(Money::from_yuan(1.994).cents(), 199);
    }

    #[test]
    fn test_money_arithmetic() {
        let a = Money::from_yuan(10.50);
        let b = Money::from_yuan(3.25);
        assert_eq!(a.add(b).to_yuan(), 13.75);
        assert_eq!(a.sub(b).to_yuan(), 7.25);
        assert_eq!(a.mul(2.0).to_yuan(), 21.00);
    }

    #[test]
    fn test_money_discount() {
        let price = Money::from_yuan(100.00);
        assert_eq!(price.apply_discount(0.9).to_yuan(), 90.00);
        assert_eq!(price.apply_discount(0.85).to_yuan(), 85.00);
    }

    #[test]
    fn test_no_precision_loss() {
        // This is the classic f64 problem: 0.1 + 0.2 != 0.3
        let a = Money::from_yuan(0.1);
        let b = Money::from_yuan(0.2);
        assert_eq!(a.add(b).to_yuan(), 0.3);
    }
}
