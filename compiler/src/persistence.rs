#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct PersistentFileInfo {
    // title is here because this struct serves double duty as the source of truth for constructing pages
    pub title: String,        // unsafe title of the file
    pub content_hash: String, // base64-encoded sha256
    pub updated_at: u64,      // unix timestamp in seconds
    pub created_at: u64,      // unix timestamp in seconds
}

// first key is the (safe) directory, second key is the (safe) title
type TrackingInfoMap =
    std::collections::HashMap<String, std::collections::HashMap<String, PersistentFileInfo>>;

pub struct TrackingInfo {
    tracked: std::collections::HashSet<String>, // key is (safe) directory + (safe) title
    info: TrackingInfoMap,
}

impl TrackingInfo {
    pub fn new() -> Self {
        TrackingInfo {
            tracked: std::collections::HashSet::new(),
            info: std::collections::HashMap::new(),
        }
    }

    pub fn read_from_file(config: &crate::Config) -> Self {
        if !config.tracking_file_path.exists() {
            println!(
                "tracking info file not found ({}), starting fresh",
                config.tracking_file_path.display()
            );
            return TrackingInfo::new();
        }

        let file = std::fs::File::open(&config.tracking_file_path).expect(&format!(
            "Failed to open tracking info file ({})",
            config.tracking_file_path.display()
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
                    config.tracking_file_path.display(),
                    e
                );
            }
        }
    }

    pub fn write_to_file(&self, config: &crate::Config) {
        let file = std::fs::File::create(&config.tracking_file_path).expect(&format!(
            "Failed to create tracking info file ({})",
            config.tracking_file_path.display()
        ));

        let mut writer = std::io::BufWriter::new(file);
        serde_json::to_writer_pretty(&mut writer, &self.info)
            .expect("Failed to serialize tracking info to JSON");
    }

    fn format_tracking_key(dir: &str, title: &str) -> String {
        let safe_dir = crate::utils::format_safe_text(dir);
        let safe_title = crate::utils::format_safe_text(title);
        format!("{}:{}", safe_dir, safe_title)
    }

    pub fn track_file(&mut self, file_data: &crate::posts::FileData) -> u64 {
        let dir_key = crate::utils::format_safe_text(&file_data.dir);
        let file_key = crate::utils::format_safe_text(&file_data.title);
        let tracking_key = Self::format_tracking_key(&file_data.dir, &file_data.title);

        if !self.tracked.insert(tracking_key) {
            panic!(
                "File {} already tracked in subdirectory {}",
                file_data.title, file_data.dir
            );
        }

        let files = self
            .info
            .entry(dir_key)
            .or_insert_with(std::collections::HashMap::new);

        let now = crate::persistence::get_timestamp();
        match files.entry(file_key) {
            std::collections::hash_map::Entry::Vacant(entry) => {
                entry.insert(PersistentFileInfo {
                    title: file_data.title.clone(),
                    content_hash: crate::persistence::hash_text(&file_data.content),
                    updated_at: now,
                    created_at: now,
                });
                return now;
            }
            std::collections::hash_map::Entry::Occupied(mut entry) => {
                // If the entry exists, compare the content hash
                let new_content_hash = crate::persistence::hash_text(&file_data.content);
                let file_info = entry.get_mut();
                if file_info.content_hash != new_content_hash {
                    file_info.content_hash = new_content_hash;
                    file_info.updated_at = now;
                }
                return file_info.updated_at;
            }
        }
    }

    pub fn purge_untracked(&mut self) {
        self.info.retain(|dir_key, files| {
            files.retain(|file_key, _| {
                let tracking_key = Self::format_tracking_key(dir_key, file_key);
                self.tracked.contains(&tracking_key)
            });
            !files.is_empty()
        });
    }

    #[allow(dead_code)]
    pub fn debug(&self) {
        println!("TrackingInfo:");
        for (dir_key, files) in &self.info {
            println!("  Subdirectory: {}", dir_key);
            for (file_key, file_info) in files {
                println!(
                    "    File: {}, Hash: {}, Updated At: {}",
                    file_key, file_info.content_hash, file_info.updated_at
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
