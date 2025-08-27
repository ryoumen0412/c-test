use chrono::{NaiveDate, Datelike};

pub fn format_date(date: &NaiveDate) -> String {
    date.format("%d/%m/%Y").to_string()
}

pub fn parse_date(date_str: &str) -> Option<NaiveDate> {
    NaiveDate::parse_from_str(date_str, "%d/%m/%Y").ok()
        .or_else(|| NaiveDate::parse_from_str(date_str, "%Y-%m-%d").ok())
}

pub fn validate_rut(rut: &str) -> bool {
    let re = regex::Regex::new(r"^[0-9]{7,8}-[0-9Kk]$").unwrap();
    re.is_match(rut)
}

pub fn validate_email(email: &str) -> bool {
    let re = regex::Regex::new(r"^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}$").unwrap();
    re.is_match(email)
}

pub fn truncate_text(text: &str, max_len: usize) -> String {
    if text.len() <= max_len {
        text.to_string()
    } else {
        format!("{}...", &text[..max_len.saturating_sub(3)])
    }
}

pub fn format_optional_date(date: &Option<NaiveDate>) -> String {
    match date {
        Some(d) => format_date(d),
        None => "N/A".to_string(),
    }
}

pub fn calculate_age(birth_date: &NaiveDate) -> i32 {
    let today = chrono::Local::now().date_naive();
    let mut age = today.year() - birth_date.year();
    
    if today.month() < birth_date.month() || 
       (today.month() == birth_date.month() && today.day() < birth_date.day()) {
        age -= 1;
    }
    
    age
}
