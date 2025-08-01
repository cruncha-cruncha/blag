pub fn generate_posts(
    site_path: &std::path::Path,
    tracking_info: &mut crate::persistence::TrackingInfo,
    build_path: &std::path::Path,
) {
    let mut post_count = 0;
    let mut sub_dir_count = 0;
    let mut found_root_file = false;
    let read_dir = std::fs::read_dir(site_path).expect("Invalid input directory");

    for file in read_dir {
        let file = file.expect("Failed to read file");
        let path = file.path();
        if path_is_md(&path) {
            if found_root_file {
                panic!("Multiple markdown files found in root directory, expected only one.");
            } else {
                found_root_file = true;
            }

            let file_data = read_file(&path).expect("Failed to read root markdown file");
            // let last_updated = tracking_info.track_file(&file_data);
            let last_updated = crate::persistence::get_timestamp();
            let output = compile_full_html(&file_data, last_updated);
            let output_path = build_path.to_owned().join("index.html");
            std::fs::create_dir_all(output_path.parent().unwrap())
                .expect("Failed to create output directory");
            std::fs::write(output_path, output).expect("Failed to write output file");
            post_count += 1;
            sub_dir_count += 1;
        } else if path.is_dir() {
            let sub_dir_name = path
                .file_name()
                .and_then(|s| s.to_str())
                .expect("Failed to get subdirectory name");
            if sub_dir_name == crate::pages::INDEX_NAME {
                panic!(
                    "Subdirectory named '{}' found, which conflicts with the pages module.",
                    crate::pages::INDEX_NAME
                );
            }

            let mut has_posts = false;
            let read_sub_dir = std::fs::read_dir(&path).expect("Failed to read subdirectory");
            for sub_file in read_sub_dir {
                let sub_file = sub_file.expect("Failed to read sub-file");
                let sub_path = sub_file.path();
                if path_is_md(&sub_path) {
                    let file_data = read_file(&sub_path).expect("Failed to read sub markdown file");
                    let last_updated = tracking_info.track_file(&file_data);
                    let output = compile_full_html(&file_data, last_updated);
                    let output_path = format_output_path(build_path, &file_data);
                    std::fs::create_dir_all(output_path.parent().unwrap())
                        .expect("Failed to create output directory");
                    std::fs::write(output_path, output).expect("Failed to write output file");
                    post_count += 1;
                    has_posts = true;
                }
            }
            if has_posts {
                sub_dir_count += 1;
            }
        }
    }

    tracking_info.purge_untracked();
    println!(
        "Generated {} posts, from {} subdirectories.",
        post_count, sub_dir_count
    );
}

pub struct FileData {
    pub dir: String,
    pub title: String,
    pub content: String,
}

fn read_file(path: &std::path::Path) -> Result<FileData, String> {
    let parent_name = path
        .parent()
        .and_then(|p| p.file_name())
        .and_then(|s| s.to_str())
        .map(|s| s.to_string())
        .ok_or("Failed to get parent directory name")?;
    let title = path
        .file_stem()
        .and_then(|s| s.to_str())
        .map(|s| s.to_string())
        .ok_or("Failed to parse file title")?;
    let content = std::fs::read_to_string(&path).map_err(|e| e.to_string())?;
    Ok(FileData {
        dir: parent_name,
        title,
        content,
    })
}

pub fn convert_md_to_html(md: &str) -> String {
    let mut html_content = String::new();
    let parser = pulldown_cmark::Parser::new(&md);
    pulldown_cmark::html::push_html(&mut html_content, parser);
    html_content
}

fn format_output_path(build_path: &std::path::Path, info: &FileData) -> std::path::PathBuf {
    let safe_title = crate::utils::format_safe_title(&info.title);
    build_path
        .to_owned()
        .join(&info.dir)
        .join(format!("{}.html", safe_title))
}

fn path_is_md(path: &std::path::Path) -> bool {
    path.is_file() && path.extension().map_or(false, |ext| ext == "md")
}

fn compile_full_html(info: &FileData, last_updated: u64) -> String {
    let html_content = convert_md_to_html(&info.content);

    let last_updated = crate::utils::format_datetime(last_updated);
    let last_updated = format!(
        r#"<p class="last-updated"><i>last updated: {}<i></p>"#,
        last_updated
    );

    format!(
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
        &info.title, html_content, last_updated,
    )
}
