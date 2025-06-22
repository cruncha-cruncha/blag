pub const SUB_DIR: &str = "posts";

pub fn generate_posts(
    input_dir: &std::path::Path,
    file_info: &mut crate::persistence::FileInfoMap,
    output_dir: &std::path::Path,
) {
    let read_dir = std::fs::read_dir(input_dir).expect("Invalid input directory");
    let mut seen_files: std::collections::HashSet<String> = std::collections::HashSet::new();

    for file in read_dir {
        // make sure it's a markdown file
        let file = file.expect("Failed to read file");
        let path = file.path();
        if !path.is_file() || path.extension().map_or(false, |ext| ext != "md") {
            println!("Skipping non-markdown-file: {:?}", path);
            continue;
        }

        // read and parse file contents
        let content = std::fs::read_to_string(&path).expect("Failed to read file content");
        let mut html_content = String::new();
        let parser = pulldown_cmark::Parser::new(&content);
        pulldown_cmark::html::push_html(&mut html_content, parser);

        // use filename as title
        let title = path.file_stem().and_then(|s| s.to_str()).unwrap_or("Post");
        // mark this file as 'seen', so we can delete unseen (aka deleted) files from file_info later
        seen_files.insert(title.to_string());
        // convert title to HTML-safe format
        let safe_title = format_safe_title(title);

        // get last_updated from file_info, updating it if necessary
        let last_updated: std::time::SystemTime;
        let content_hash = crate::persistence::hash_content(&content);
        if let Some(info) = file_info.get_mut(title) {
            if info.content_hash != content_hash {
                info.content_hash = content_hash;
                info.updated_at = std::time::SystemTime::now();
            }
            last_updated = info.updated_at;
        } else {
            last_updated = std::time::SystemTime::now();
            file_info.insert(
                title.to_string(),
                crate::persistence::PersistentFileInfo {
                    title: title.to_string(),
                    content_hash: content_hash.clone(),
                    updated_at: last_updated,
                },
            );
        }

        // pretty-print last_updated
        let last_updated = format_datetime(last_updated);
        let last_updated = format!(
            r#"<p class="last-updated"><i>Last updated: {}<i></p>"#,
            last_updated
        );

        // generate final HTML content
        let full_out = format!(
            r#"
<!DOCTYPE html>
<html lang="en">
    <head>
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <meta charset="UTF-8">
        <title>{}</title>
    </head>
    <body class="post">
        {}
        {}
    </body>
</html>
"#,
            title, html_content, last_updated
        );

        // save the HTML content to output directory
        let output_path = output_dir
            .to_owned()
            .join(SUB_DIR)
            .join(format!("{}.html", safe_title));
        std::fs::create_dir_all(output_path.parent().unwrap())
            .expect("Failed to create output directory");
        std::fs::write(output_path, full_out).expect("Failed to write output file");
        println!("Generated post: {}", safe_title);
    }

    // remove files from file_info that were not seen in this run
    file_info.retain(|name, _| seen_files.contains(name));
}

pub fn format_safe_title(title: &str) -> String {
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

pub fn format_datetime(sys_time: std::time::SystemTime) -> String {
    let datetime: chrono::DateTime<chrono::Local> = sys_time.into();
    // Month day, Year
    datetime.format("%B %e, %Y").to_string()
}
