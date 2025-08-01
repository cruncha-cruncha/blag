mod pages;
mod persistence;
mod posts;
mod utils;

fn main() {
    println!("generating static site...");

    let config = Config::init();

    let mut tracking_info = persistence::TrackingInfo::read_from_file(&config);
    posts::generate_posts(&config, &mut tracking_info);
    pages::generate_pages(&config, &tracking_info);
    tracking_info.write_to_file(&config);

    println!("static site generated successfully.");
}

pub struct Config {
    // where are all the markdown files?
    pub input_path: std::path::PathBuf,

    // where should the compiled files end up?
    pub output_path: std::path::PathBuf,

    // where is the tracking info file?
    // The tracking info file is used to determine updated_at, which determines sort order
    // Read over persistence.rs for more misunderstandings
    pub tracking_file_path: std::path::PathBuf,

    // how many posts per page?
    pub posts_per_page: usize,

    // what is the name of the pages index?
    // Example: if pages_index_name = "page-index", and posts_per_page = 20, and you have 41 markdown documents in a directory called 'posts-md',
    //   this will result in three pages (41 = 20 + 20 + 1): /page-index/posts-md/1.html, /page-index/posts-md/2.html, and /page-index/posts-md/3.html
    pub pages_index_name: String,

    // where do you want the 'github' link to go?
    pub github_link: String,

    // what is the name of this repo?
    // Must be URL-safe; this compiler expects your site to be served from <github_link>/<repo_name>
    // Example: if repo_name = "some-repo", and you have a file called "banana.md" in a directory called "posts-md",
    //   this will result in a page at <github_link>/<repo_name>/posts-md/banana.html
    //   and the pages generator needs to know about this so it can generate the correct links
    // repo_name is also used as the default output directory if OUTPUT_DIR is not set
    pub repo_name: String,
}

impl Config {
    pub fn init() -> Self {
        let repo_name = std::env::var("REPO_NAME").unwrap_or("blag".to_string());

        let github_link = std::env::var("GITHUB_LINK")
            .unwrap_or("https://github.com/cruncha-cruncha/".to_string());

        let input_dir = std::env::var("INPUT_DIR").unwrap_or("../site".to_string());
        let input_path = std::path::Path::new(&input_dir).to_owned();

        let fallback_output_dir = match repo_name.is_empty() {
            true => "../build".to_string(),
            false => format!("../{}", repo_name),
        };
        let output_dir = std::env::var("OUTPUT_DIR").unwrap_or(fallback_output_dir);
        let output_path = std::path::Path::new(&output_dir).to_owned();

        let tracking_file =
            std::env::var("TRACKING_FILE").unwrap_or("./blag_info.json".to_string());
        let tracking_file_path = std::path::Path::new(&tracking_file).to_owned();

        let posts_per_page = std::env::var("PER_PAGE")
            .unwrap_or("50".to_string())
            .parse::<usize>()
            .unwrap_or(50); // Default value, can be changed later

        let pages_index_name = std::env::var("PAGES_INDEX_NAME").unwrap_or("pages".to_string());

        Config {
            input_path,
            output_path,
            tracking_file_path,
            posts_per_page,
            pages_index_name,
            repo_name,
            github_link,
        }
    }
}
