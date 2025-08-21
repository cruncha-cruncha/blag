pub fn generate_pages(config: &crate::Config, tracking_info: &crate::persistence::TrackingInfo) {
    let mut page_count = 0;
    let mut sub_dir_count = 0;

    for (sub_dir, posts) in tracking_info.into_iter() {
        let mut pages = Vec::new();
        let mut buffer = PageBuffer::new(config, sub_dir);

        println!("Generating pages for subdirectory: {}", sub_dir);

        let mut files = posts.values().cloned().collect::<Vec<_>>();
        files.sort_by(|a, b| b.created_at.cmp(&a.created_at));

        let total_chunks = (files.len() + config.posts_per_page - 1) / config.posts_per_page;

        let mut wrote_page = false;
        for (i, chunk) in files.chunks(config.posts_per_page).enumerate() {
            buffer.write_header();
            for post in chunk {
                buffer.add_post(&post.title);
            }
            buffer.add_navigation(i, total_chunks);
            buffer.write_footer();
            pages.push(buffer.consume());
            page_count += 1;
            wrote_page = true;
        }

        if wrote_page {
            sub_dir_count += 1;
        }

        for (i, page) in pages.iter().enumerate() {
            let output_path = config
                .output_path
                .clone()
                .join(&config.pages_index_name)
                .join(sub_dir)
                .join(format!("{}.html", i + 1));
            std::fs::create_dir_all(output_path.parent().unwrap())
                .expect("Failed to create output directory");
            std::fs::write(output_path, page).expect("Failed to write output file");
        }
    }

    println!(
        "Generated {} pages in {} subdirectories",
        page_count, sub_dir_count
    );
}

struct PageBuffer<'a> {
    config: &'a crate::Config,
    safe_dir: String,
    content: String,
}

impl<'a> PageBuffer<'a> {
    fn new(config: &'a crate::Config, safe_dir: &str) -> Self {
        PageBuffer {
            config,
            safe_dir: safe_dir.to_string(),
            content: String::new(),
        }
    }

    pub fn write_header(&mut self) {
        self.content += r#"
<!DOCTYPE html>
<html lang="en">
    <head>
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <meta charset="UTF-8">
        <title>blag</title>
    </head>
    <body class="page">
"#
    }

    pub fn write_footer(&mut self) {
        self.content += r#"
    </body>
</html>
"#;
    }

    pub fn add_post(&mut self, title: &str) {
        let safe_title = crate::utils::format_safe_text(title);
        let link = format!(
            r#"/{}/{}/{}.html"#,
            self.config.repo_name, self.safe_dir, safe_title
        );
        let html_content = format!(r#"<p><a href="{}">{}</a></p>"#, link, title);
        self.content.push_str(&html_content);
        self.content.push('\n');
    }

    pub fn add_navigation(&mut self, current_page: usize, total_pages: usize) {
        self.content += r#"<nav>"#;

        self.content += r#"<span>navigation: </span>"#;

        if current_page > 0 {
            self.content += &format!(
                r#"<a href="/{}/{}/{}/{}.html">previous page</a>"#,
                self.config.repo_name,
                self.config.pages_index_name,
                self.safe_dir,
                current_page - 1
            );
        } else {
            self.content += r#"<span>previous page</span>"#;
        }

        self.content += r#"<span>, </span>"#;

        self.content += &format!(r#"<a href="{}">github</a>"#, self.config.github_link);

        self.content += r#"<span>, </span>"#;

        if current_page + 1 < total_pages {
            self.content += &format!(
                r#"<a href="/{}/{}/{}/{}.html">next page</a>"#,
                self.config.repo_name,
                self.config.pages_index_name,
                self.safe_dir,
                current_page + 1
            );
        } else {
            self.content += r#"<span>next page</span>"#;
        }

        self.content += r#"</nav>"#;
    }

    pub fn consume(&mut self) -> String {
        let content = self.content.clone();
        self.content.clear();
        content
    }
}
