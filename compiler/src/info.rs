use crate::INFO_FILE_NAME;
use crate::utils::Utils;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

pub struct InfoWrangler {
    info_file: InfoFile,
    lookup: HashMap<String, usize>, // maps safe_filename to index in info_files.articles
}

#[derive(Clone, Serialize, Deserialize)]
struct InfoFile {
    articles: Vec<ArticleInfo>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ArticleInfo {
    pub original_filename: String, // the original filename, without extension
    pub safe_filename: String,     // the url-safe filename, without extension
    pub created_at: u64,           // unix timestamp in seconds
    pub updated_at: u64,           // unix timestamp in seconds
    pub content_hash: String,      // base64-encoded sha256
    pub tags: Vec<String>,         // bloom filter of tags, maybe base64 encoded? or bigint?
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicInfoFile {
    pub page_num: u32,
    pub search_term: String,
    pub articles: Vec<PublicArticleInfo>,
    pub results: Vec<PublicArticleInfo>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicArticleInfo {
    pub original_title: String,
    pub safe_title: String,
    pub created_at: u64,
    pub tags: Vec<String>,
}

impl PublicInfoFile {
    fn from(info_file: &InfoFile) -> Self {
        let articles = info_file
            .articles
            .iter()
            .map(|article| PublicArticleInfo::from(article))
            .collect();

        PublicInfoFile {
            articles,
            page_num: 0,
            search_term: String::new(),
            results: vec![],
        }
    }
}

impl PublicArticleInfo {
    fn from(article: &ArticleInfo) -> Self {
        PublicArticleInfo {
            original_title: article.original_filename.clone(),
            safe_title: article.safe_filename.clone(),
            created_at: article.created_at,
            tags: article.tags.clone(),
        }
    }
}

impl InfoFile {
    fn new() -> Self {
        InfoFile { articles: vec![] }
    }
}

impl InfoWrangler {
    fn new() -> Self {
        InfoWrangler {
            info_file: InfoFile::new(),
            lookup: HashMap::new(),
        }
    }

    pub fn init() -> Self {
        let info_file_path = Path::new(".").join(INFO_FILE_NAME);
        let file = match std::fs::File::open(&info_file_path) {
            Ok(f) => f,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                return InfoWrangler::new();
            }
            Err(e) => {
                panic!(
                    "Failed to open tracking info file ({:?}) : {}",
                    info_file_path, e
                );
            }
        };

        let mut reader = std::io::BufReader::new(file);
        let info_file = match serde_json::from_reader::<_, InfoFile>(&mut reader) {
            Ok(v) => v,
            Err(e) => {
                panic!(
                    "Failed to deserialize tracking info (file {:?}) from JSON: {}",
                    info_file_path, e
                );
            }
        };

        InfoWrangler {
            info_file,
            lookup: HashMap::new(),
        }
    }

    pub fn get_public_info(&self) -> PublicInfoFile {
        PublicInfoFile::from(&self.info_file)
    }

    pub fn save(&mut self) {
        // save to regular location
        self.sort_alphabetical();
        let info_file_path = Path::new(".").join(INFO_FILE_NAME);
        let file = std::fs::File::create(&info_file_path).expect(&format!(
            "Failed to create tracking info file ({:?})",
            info_file_path
        ));

        let mut writer = std::io::BufWriter::new(file);
        serde_json::to_writer_pretty(&mut writer, &self.info_file)
            .expect("Failed to serialize tracking info to JSON");

        // // save to output directory
        // self.sort_created_at();
        // let small_info = PublicInfoFile::from(&self.info_file);

        // let info_file_path = Path::new(OUTPUT_DIR).join(INFO_FILE_NAME);
        // std::fs::create_dir_all(info_file_path.parent().unwrap())
        //     .expect("Failed to create output directory");
        // let file = std::fs::File::create(&info_file_path).expect(&format!(
        //     "Failed to create tracking info file ({:?})",
        //     info_file_path
        // ));

        // let mut writer = std::io::BufWriter::new(file);
        // serde_json::to_writer(&mut writer, &small_info)
        //     .expect("Failed to serialize tracking info to JSON");
    }

    pub fn upsert(&mut self, path: &Path) {
        let original_filename = match Utils::extract_filename(path) {
            Some(name) => name,
            None => return,
        };

        let safe_filename = match Utils::format_safe_filename(&original_filename) {
            Some(name) => name,
            None => return,
        };

        for (i, article) in self.info_file.articles.iter().enumerate() {
            if article.safe_filename == safe_filename {
                if self.lookup.insert(safe_filename.clone(), i).is_some() {
                    panic!("Duplicate safe filename detected: {}", safe_filename);
                }
                return;
            }
        }

        let now = match Utils::get_timestamp() {
            Some(ts) => ts,
            None => return,
        };

        let new_article = ArticleInfo {
            original_filename: original_filename,
            safe_filename: safe_filename.clone(),
            created_at: now,
            updated_at: now,
            content_hash: String::new(),
            tags: vec![],
        };

        self.info_file.articles.push(new_article);
        self.lookup
            .insert(safe_filename, self.info_file.articles.len() - 1);
    }

    pub fn update_content(&mut self, path: &Path, content: &str) {
        let original_filename = match Utils::extract_filename(path) {
            Some(name) => name,
            None => return,
        };

        let safe_filename = match Utils::format_safe_filename(&original_filename) {
            Some(name) => name,
            None => return,
        };

        let index = match self.lookup.get(&safe_filename) {
            Some(idx) => idx,
            None => return,
        };

        let bytes = <sha2::Sha256 as sha2::Digest>::digest(content.as_bytes());
        let content_hash =
            base64::Engine::encode(&base64::engine::general_purpose::STANDARD, bytes);

        self.info_file.articles.get_mut(*index).map(|article| {
            if article.content_hash != content_hash {
                article.content_hash = content_hash;
                article.updated_at = Utils::get_timestamp().unwrap_or(article.updated_at);
            }
        });
    }

    pub fn get_article(&self, path: &Path) -> Option<&ArticleInfo> {
        let original_filename = Utils::extract_filename(path)?;
        let safe_filename = Utils::format_safe_filename(&original_filename)?;
        let index = self.lookup.get(&safe_filename)?;
        self.info_file.articles.get(*index)
    }

    pub fn sort_created_at(&mut self) {
        self.info_file
            .articles
            .sort_by(|a, b| a.created_at.cmp(&b.created_at));
    }

    pub fn sort_alphabetical(&mut self) {
        self.info_file
            .articles
            .sort_by(|a, b| a.original_filename.cmp(&b.original_filename));
    }
}
