use std::path::Path;

pub struct Utils {}

const MAX_SAFE_FILENAME_LENGTH: usize = 256;

impl Utils {
    pub fn extract_filename(path: &Path) -> Option<String> {
        path.file_stem()
            .and_then(|os_str| os_str.to_str())
            .map(|s| s.to_string())
    }

    // replace / coerce every character to: [a-z0-9_-], replacing spaces with underscores, and other special characters with hyphens
    // also slice down to at most 256 characters long
    pub fn format_safe_filename(filename: &str) -> Option<String> {
        let filename = filename.to_lowercase();

        let filename = filename.replace(" ", "_");

        let re = regex::Regex::new(r"[\\/:?!()\[\]<>,#&]").unwrap();
        let filename = re.replace_all(&filename, "-");

        let re = regex::Regex::new(r"[^a-z0-9_-]").unwrap();
        let filename = re.replace_all(&filename, "");

        let filename = if filename.len() > MAX_SAFE_FILENAME_LENGTH {
            &filename[..MAX_SAFE_FILENAME_LENGTH]
        } else {
            &filename
        };

        Some(filename.to_string())
    }

    pub fn get_timestamp() -> Option<u64> {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|duration| duration.as_secs())
            .ok()
    }

    pub fn format_datetime(timestamp: u64) -> Option<String> {
        let sys_time = std::time::SystemTime::UNIX_EPOCH
            .checked_add(std::time::Duration::from_secs(timestamp))?;

        let datetime: chrono::DateTime<chrono::Local> = sys_time.into();

        // Month day, Year
        Some(datetime.format("%B %e, %Y").to_string())
    }
}