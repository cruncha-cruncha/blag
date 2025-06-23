pub const SUB_DIR: &str = "posts";

pub fn generate_posts(
    posts_path: &std::path::Path,
    file_info: &mut crate::persistence::FileInfoMap,
    build_path: &std::path::Path,
) {
    let read_dir = std::fs::read_dir(posts_path).expect("Invalid input directory");
    let mut seen_files: std::collections::HashSet<String> = std::collections::HashSet::new();
    let mut count = 0;

    for file in read_dir {
        // make sure it's a markdown file
        let file = file.expect("Failed to read file");
        let path = file.path();
        if !path.is_file() || path.extension().map_or(false, |ext| ext != "md") {
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
        let last_updated: u64;
        let content_hash = crate::persistence::hash_content(&content);
        if let Some(info) = file_info.get_mut(title) {
            if info.content_hash != content_hash {
                info.content_hash = content_hash;
                info.updated_at = crate::persistence::get_timestamp();
                println!("Post updated: {}", safe_title);
            }
            last_updated = info.updated_at;
        } else {
            if std::env::var("MODIFIED_AT_OS").is_ok() {
                last_updated = path.metadata().and_then(|m| m.modified()).map_or_else(
                    |_| crate::persistence::get_timestamp(),
                    |t| crate::persistence::system_to_timestamp(t),
                );
            } else {
                last_updated = crate::persistence::get_timestamp();
            }
            
            file_info.insert(
                title.to_string(),
                crate::persistence::PersistentFileInfo {
                    title: title.to_string(),
                    content_hash: content_hash.clone(),
                    updated_at: last_updated,
                },
            );

            println!("Post created: {}", safe_title);
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
        let output_path = build_path
            .to_owned()
            .join(SUB_DIR)
            .join(format!("{}.html", safe_title));
        std::fs::create_dir_all(output_path.parent().unwrap())
            .expect("Failed to create output directory");
        std::fs::write(output_path, full_out).expect("Failed to write output file");
        
        count += 1;
    }

    // log
    println!("Generated {} posts", count);

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

pub fn format_datetime(timestamp: u64) -> String {
    let sys_time = std::time::SystemTime::UNIX_EPOCH
        .checked_add(std::time::Duration::from_secs(timestamp))
        .expect("Timestamp is too large");
    let datetime: chrono::DateTime<chrono::Local> = sys_time.into();
    // Month day, Year
    datetime.format("%B %e, %Y").to_string()
}
