pub fn format_safe_text(title: &str) -> String {
    // Use only lowercase letters a–z, digits 0–9, hyphens, and underscores if possible
    let title = title
        .to_lowercase()
        .replace(" ", "_")
        .replace("/", "_")
        .replace(":", "_")
        .replace("?", "_")
        .replace("!", "_")
        .replace(".", "_");
    let re = regex::Regex::new(r"[^a-z0-9_-]").unwrap();
    let safe_title = re.replace_all(&title, "");
    safe_title.to_string()
}

pub fn format_datetime(timestamp: u64) -> String {
    let sys_time = std::time::SystemTime::UNIX_EPOCH
        .checked_add(std::time::Duration::from_secs(timestamp))
        .expect("Timestamp is too large");
    let datetime: chrono::DateTime<chrono::Local> = sys_time.into();
    // Month day, Year
    datetime.format("%B %e, %Y").to_string()
}