pub const SUB_DIR: &str = "pages";
const POSTS_PER_PAGE: usize = 50;

pub struct PostInfo {
    pub modified: std::time::SystemTime,
    pub html_content: String,
}

pub fn generate_pages(input_dir: &std::path::Path, output_dir: &std::path::Path) {
    let read_dir = std::fs::read_dir(input_dir).expect("Invalid input directory");
    let mut posts: Vec<PostInfo> = Vec::new();

    for file in read_dir {
        let file = file.expect("Failed to read file");
        let path = file.path();
        if !path.is_file() {
            continue;
        }

        let title = path.file_stem().and_then(|s| s.to_str()).unwrap_or("Post");
        let safe_title = crate::posts::safe_title(title);

        let modified = file
            .metadata()
            .expect("Failed to get metadata")
            .modified()
            .unwrap_or(std::time::SystemTime::UNIX_EPOCH);

        let link = format!(r#"/blag/{}/{}.html"#, crate::posts::SUB_DIR, safe_title);
        let html_content = format!(r#"<a href="{}">{}</a>"#, link, title);

        posts.push(PostInfo {
            modified,
            html_content,
        });
    }

    posts.sort_by(|a, b| b.modified.cmp(&a.modified));
    let mut pages: Vec<String> = Vec::new();

    let total_chunks = (posts.len() + POSTS_PER_PAGE - 1) / POSTS_PER_PAGE;
    for (i, chunk) in posts.chunks(POSTS_PER_PAGE).enumerate() {
        let mut buffer = format!(
            r#"
<!DOCTYPE html>
<html lang="en">
    <head>
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <meta charset="UTF-8">
        <title>blag</title>
    </head>
    <body class="page">
"#
        );

        for post in chunk {
            buffer.push_str(&post.html_content);
        }

        add_navigation(&mut buffer, i, total_chunks);

        buffer.push_str(
            r#"
    </body>
</html>
"#,
        );

        pages.push(buffer);
    }

    for (i, page) in pages.iter().enumerate() {
        let output_path = output_dir
            .to_owned()
            .join(SUB_DIR)
            .join(format!("{}.html", i + 1));
        std::fs::create_dir_all(output_path.parent().unwrap())
            .expect("Failed to create output directory");
        std::fs::write(output_path, page).expect("Failed to write output file");
        println!("Generated page: {}", i + 1);
    }
}

fn add_navigation(buffer: &mut String, current_page: usize, total_pages: usize) {
    buffer.push_str(r#"<nav>"#);

    buffer.push_str(r#"<span>navigation: </span>"#);

    if current_page > 0 {
        buffer.push_str(&format!(
            r#"<a href="/{}/{}.html">previous page</a>"#,
            SUB_DIR,
            current_page - 1
        ));
    } else {
        buffer.push_str(r#"<span>previous page</span>"#);
    }

    buffer.push_str(r#"<span>, </span>"#);

    if current_page + 1 < total_pages {
        buffer.push_str(&format!(
            r#"<a href="/{}/{}.html">next page</a>"#,
            SUB_DIR,
            current_page + 1
        ));
    } else {
        buffer.push_str(r#"<span>next page</span>"#);
    }

    buffer.push_str(r#"<span>, </span>"#);

    buffer.push_str(r#"<a href="https://github.com/cruncha-cruncha/blag">github</a>"#);

    buffer.push_str(r#"</nav>"#);
}
