const FILE_PATH: &str = "/blag_info.bin";

pub type FileInfoMap = std::collections::HashMap<String, PersistentFileInfo>;

#[derive(Clone, bincode::Encode, bincode::Decode)]
pub struct PersistentFileInfo {
    pub title: String,
    pub content_hash: Vec<u8>,
    pub updated_at: std::time::SystemTime,
}

pub fn read_info() -> FileInfoMap {
    let file_path = std::path::Path::new(FILE_PATH);
    if !file_path.exists() {
        println!("blag_info.bin not found");
        return std::collections::HashMap::new();
    }

    let file = std::fs::File::open(file_path).expect("Failed to open blag_info.bin");
    let mut reader = std::io::BufReader::new(file);
    match bincode::decode_from_std_read::<FileInfoMap, _, _>(
        &mut reader,
        bincode::config::standard(),
    ) {
        Ok(file_info) => {
            println!("blag_info.bin loaded successfully.");
            file_info
        }
        Err(e) => {
            eprintln!("Failed to deserialize blag_info.bin: {}", e);
            std::collections::HashMap::new()
        }
    }
}

pub fn hash_content(content: &str) -> Vec<u8> {
    <sha2::Sha256 as sha2::Digest>::digest(content.as_bytes()).to_vec()
}

pub fn write_info(file_info: &FileInfoMap) {
    let file_path = std::path::Path::new(FILE_PATH);
    let file = std::fs::File::create(file_path).expect("Failed to create blag_info.bin");
    let mut writer = std::io::BufWriter::new(file);
    bincode::encode_into_std_write(file_info, &mut writer, bincode::config::standard())
        .expect("Failed to serialize blag_info.bin");
}
