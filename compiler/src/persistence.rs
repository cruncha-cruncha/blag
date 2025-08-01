#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct PersistentFileInfo {
    pub title: String,
    pub content_hash: String, // base64-encoded sha256
    pub updated_at: u64,      // unix timestamp in seconds
}

// TODO: convert this to a sorted data structure, so file modifications in git are more sane
// first key is the directory, second key is a hash of (title + content_hash)
type TrackingInfoMap =
    std::collections::HashMap<String, std::collections::HashMap<String, PersistentFileInfo>>;

pub struct TrackingInfo {
    tracked: std::collections::HashSet<String>,
    info: TrackingInfoMap,
}

impl TrackingInfo {
    pub fn new() -> Self {
        TrackingInfo {
            tracked: std::collections::HashSet::new(),
            info: std::collections::HashMap::new(),
        }
    }

    pub fn read_from_file(file_path: &std::path::Path) -> Self {
        if !file_path.exists() {
            println!(
                "tracking info file not found ({}), starting fresh",
                file_path.display()
            );
            return TrackingInfo::new();
        }

        let file = std::fs::File::open(file_path).expect(&format!(
            "Failed to open tracking info file ({})",
            file_path.display()
        ));

        let mut reader = std::io::BufReader::new(file);
        match serde_json::from_reader::<_, TrackingInfoMap>(&mut reader) {
            Ok(info) => TrackingInfo {
                tracked: std::collections::HashSet::new(),
                info,
            },
            Err(e) => {
                panic!(
                    "Failed to deserialize tracking info (file {}) from JSON: {}",
                    file_path.display(),
                    e
                );
            }
        }
    }

    pub fn write_to_file(&self, file_path: &std::path::Path) {
        let file = std::fs::File::create(file_path).expect(&format!(
            "Failed to create tracking info file ({})",
            file_path.display()
        ));

        let mut writer = std::io::BufWriter::new(file);
        serde_json::to_writer_pretty(&mut writer, &self.info)
            .expect("Failed to serialize tracking info to JSON");
    }

    fn format_dir_key(dir: &str) -> String {
        format!("{}", dir)
    }

    fn format_file_key(title: &str, content_hash: &str) -> String {
        let combined_text = format!("{}:{}", title, content_hash);
        crate::persistence::hash_text(&combined_text)
    }

    fn format_tracking_key(dir: &str, title: &str, content_hash: &str) -> String {
        let dir_key = Self::format_dir_key(dir);
        let file_key = Self::format_file_key(title, content_hash);
        format!("{}:{}", dir_key, file_key)
    }

    pub fn track_file(&mut self, file_data: &crate::posts::FileData) -> u64 {
        let dir_key = Self::format_dir_key(&file_data.dir);
        let content_hash = crate::persistence::hash_text(&file_data.content);
        let file_key = Self::format_file_key(&file_data.title, &content_hash);
        let tracking_key =
            Self::format_tracking_key(&file_data.dir, &file_data.title, &content_hash);

        self.tracked.insert(tracking_key.clone());

        let files = self
            .info
            .entry(dir_key.clone())
            .or_insert_with(std::collections::HashMap::new);

        let file_info = files
            .entry(file_key.clone())
            .or_insert_with(|| PersistentFileInfo {
                title: file_data.title.clone(),
                content_hash: content_hash.clone(),
                updated_at: crate::persistence::get_timestamp(),
            });

        file_info.updated_at
    }

    pub fn purge_untracked(&mut self) {
        self.info.retain(|sub_dir, files| {
            files.retain(|_, file| {
                let tracking_key =
                    Self::format_tracking_key(&sub_dir, &file.title, &file.content_hash);
                self.tracked.contains(&tracking_key)
            });
            !files.is_empty()
        });
    }

    #[allow(dead_code)]
    pub fn debug(&self) {
        println!("TrackingInfo:");
        for (sub_dir, files) in &self.info {
            println!("  Subdirectory: {}", sub_dir);
            for (_, file_info) in files {
                println!(
                    "    File: {}, Hash: {}, Updated At: {}",
                    file_info.title, file_info.content_hash, file_info.updated_at
                );
            }
        }
    }

    pub fn into_iter(&self) -> TrackingInfoIter {
        TrackingInfoIter::new(self)
    }
}

pub fn hash_text(text: &str) -> String {
    let bytes = <sha2::Sha256 as sha2::Digest>::digest(text.as_bytes());
    base64::Engine::encode(&base64::engine::general_purpose::STANDARD, bytes)
}

pub fn get_timestamp() -> u64 {
    system_time_to_timestamp(std::time::SystemTime::now())
}

pub fn system_time_to_timestamp(time: std::time::SystemTime) -> u64 {
    time.duration_since(std::time::UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs()
}

pub struct TrackingInfoIter<'a> {
    iter: std::collections::hash_map::Iter<
        'a,
        String,
        std::collections::HashMap<String, PersistentFileInfo>,
    >,
}

impl<'a> TrackingInfoIter<'a> {
    pub fn new(tracking_info: &'a TrackingInfo) -> Self {
        TrackingInfoIter {
            iter: tracking_info.info.iter(),
        }
    }
}

impl<'a> Iterator for TrackingInfoIter<'a> {
    type Item = (
        &'a String,
        &'a std::collections::HashMap<String, PersistentFileInfo>,
    );

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}
