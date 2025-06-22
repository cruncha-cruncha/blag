pub const SUB_DIR: &str = "posts";

pub fn generate_posts(input_dir: &std::path::Path, output_dir: &std::path::Path) {
    let read_dir = std::fs::read_dir(input_dir).expect("Invalid input directory");

    for file in read_dir {
        let file = file.expect("Failed to read file");
        let path = file.path();
        if !path.is_file() {
            println!("Skipping non-file: {:?}", path);
            continue;
        }

        let last_updated = file
            .metadata()
            .and_then(|m| m.modified())
            .unwrap_or(std::time::SystemTime::UNIX_EPOCH);
        let last_updated = format_datetime(last_updated);
        let last_updated = format!(
            r#"<p class="last-updated"><i>Last updated: {}<i></p>"#,
            last_updated
        );

        let content = std::fs::read_to_string(&path).expect("Failed to read file content");
        let mut html_content = String::new();
        let parser = pulldown_cmark::Parser::new(&content);
        pulldown_cmark::html::push_html(&mut html_content, parser);

        let title = path.file_stem().and_then(|s| s.to_str()).unwrap_or("Post");

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

        let safe_title = safe_title(title);
        let output_path = output_dir
            .to_owned()
            .join(SUB_DIR)
            .join(format!("{}.html", safe_title));

        std::fs::create_dir_all(output_path.parent().unwrap())
            .expect("Failed to create output directory");
        std::fs::write(output_path, full_out).expect("Failed to write output file");
        println!("Generated post: {}", safe_title);
    }
}

pub fn safe_title(title: &str) -> String {
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
    datetime.format("%B %e, %Y").to_string()
}
