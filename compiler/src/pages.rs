pub const SUB_DIR: &str = "pages";
const POSTS_PER_PAGE: usize = 50;

pub fn generate_pages(file_info: &crate::persistence::FileInfoMap, build_path: &std::path::Path) {
    let mut pages: Vec<String> = Vec::new();
    let mut files = file_info.values().cloned().collect::<Vec<_>>();
    files.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));

    let total_chunks = (files.len() + POSTS_PER_PAGE - 1) / POSTS_PER_PAGE;
    for (i, chunk) in files.chunks(POSTS_PER_PAGE).enumerate() {
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
            let safe_title = crate::posts::format_safe_title(&post.title);
            let link = format!(r#"/blag/{}/{}.html"#, crate::posts::SUB_DIR, safe_title);
            let html_content = format!(r#"<p><a href="{}">{}</a></p>"#, link, post.title);
            buffer.push_str(&html_content);
            buffer.push('\n');
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

    let mut count = 0;
    for (i, page) in pages.iter().enumerate() {
        let output_path = build_path
            .to_owned()
            .join(SUB_DIR)
            .join(format!("{}.html", i + 1));
        std::fs::create_dir_all(output_path.parent().unwrap())
            .expect("Failed to create output directory");
        std::fs::write(output_path, page).expect("Failed to write output file");
        count += 1;
    }

    // log
    println!("Generated {} pages", count);
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
