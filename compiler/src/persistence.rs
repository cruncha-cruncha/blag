pub type FileInfoMap = std::collections::HashMap<String, PersistentFileInfo>;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct PersistentFileInfo {
    pub title: String,
    pub content_hash: String, // base64-encoded sha256
    pub updated_at: u64,      // unix timestamp in seconds
}

pub fn read_info(file_path: &std::path::Path) -> FileInfoMap {
    if !file_path.exists() {
        println!("blag_info.json not found");
        return std::collections::HashMap::new();
    }

    let file = std::fs::File::open(file_path).expect("Failed to open blag_info.json");
    let mut reader = std::io::BufReader::new(file);
    match serde_json::from_reader::<_, FileInfoMap>(&mut reader) {
        Ok(file_info) => file_info,
        Err(e) => {
            eprintln!("Failed to deserialize file info from JSON: {}", e);
            std::collections::HashMap::new()
        }
    }
}

pub fn hash_content(content: &str) -> String {
    let bytes = <sha2::Sha256 as sha2::Digest>::digest(content.as_bytes());
    base64::Engine::encode(&base64::engine::general_purpose::STANDARD, bytes)
}

pub fn get_timestamp() -> u64 {
    system_to_timestamp(std::time::SystemTime::now())
}

pub fn system_to_timestamp(time: std::time::SystemTime) -> u64 {
    time.duration_since(std::time::UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs()
}

pub fn write_info(file_info: &FileInfoMap, file_path: &std::path::Path) {
    let file = std::fs::File::create(file_path).expect("Failed to create blag_info.json");
    let mut writer = std::io::BufWriter::new(file);
    serde_json::to_writer_pretty(&mut writer, &file_info)
        .expect("Failed to serialize file info to JSON");
}
