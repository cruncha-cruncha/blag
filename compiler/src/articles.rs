use std::path::{Path, PathBuf};

use crate::{
    INPUT_DIR, OUTPUT_DIR,
    info::{ArticleInfo, InfoWrangler},
    utils::Utils,
};

pub struct Articles {}

impl Articles {
    pub fn must_get_article_paths() -> Vec<PathBuf> {
        let read_dir = match std::fs::read_dir(INPUT_DIR) {
            Ok(rd) => rd,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                std::fs::create_dir_all(INPUT_DIR).expect("Failed to create output directory");
                return vec![];
            }
            Err(e) => {
                panic!("Failed to read input directory ({}): {}", INPUT_DIR, e);
            }
        };

        let mut paths = Vec::new();
        for file in read_dir {
            let path = file.expect("Failed to read file").path();
            paths.push(path);
        }

        paths
    }

    pub fn process(path: &Path, info_wrangler: &mut InfoWrangler) {
        let content = match std::fs::read_to_string(path) {
            Ok(c) => c,
            Err(e) => {
                panic!("Failed to read file {:?}: {}", path, e);
            }
        };

        info_wrangler.update_content(path, &content);

        let mut html_content = String::new();
        let parser = pulldown_cmark::Parser::new(&content);
        pulldown_cmark::html::push_html(&mut html_content, parser);

        let article_info = match info_wrangler.get_article(path) {
            Some(info) => info,
            None => {
                panic!("Article info not found for path {:?}", path);
            }
        };
        let full_html = Self::compile_full_html(article_info, &html_content);

        let file_path = Path::new(OUTPUT_DIR).join(article_info.safe_filename.clone() + ".html");
        std::fs::create_dir_all(file_path.parent().unwrap())
            .expect("Failed to create output directory");
        std::fs::write(file_path, full_html).expect("Failed to write output file");
    }

    fn compile_full_html(article_info: &ArticleInfo, html_content: &str) -> String {
        let tags = if article_info.tags.is_empty() {
            "".to_string()
        } else {
            let tag_spans: Vec<String> = article_info
                .tags
                .iter()
                .map(|tag| format!(r#"<span class="tag">{}</span>"#, tag))
                .collect();
            format!(
                r#"<p class="tags"><i>tags</i>: {}</p>"#,
                tag_spans.join(" ")
            )
        };

        let last_updated = match Utils::format_datetime(article_info.updated_at) {
            Some(dt) => dt,
            None => "unknown".to_string(),
        };
        let last_updated = format!(
            r#"<p class="last-updated"><i>last updated</i>: {}</p>"#,
            last_updated
        );

        // TODO: button for previous / next article?

        format!(
            r#"<!DOCTYPE html>
<html lang="en">
    <head>
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <meta charset="UTF-8">
        <title>{}</title>
        <style>
            table {{
                border-collapse: collapse;
                width: 100%;
            }}
            th, td {{
                border: 1px solid #ddd;
                text-align: right;
                padding: 8px; /* Adds spacing between columns */
            }}
        </style>
    </head>
    <body>
        <div style="max-width:800px;margin-left:auto;margin-right:auto;">
            {}
            {}
            {}
        </div>
    </body>
</html>
"#,
            article_info.original_filename, html_content, tags, last_updated,
        )
    }
}
