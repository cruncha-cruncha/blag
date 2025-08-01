mod pages;
mod persistence;
mod posts;
mod utils;

// TODO: get these from the github workflow
pub const ROOT_SUBPATH: &str = "blag";
pub const GITHUB_LINK: &str = "https://github.com/cruncha-cruncha/blag";

fn main() {
    println!("generating static site...");

    // where are the markdown files for the site?
    let site_dir =  std::env::var("SITE_DIR").unwrap_or("../site".to_string());
    let site_path = std::path::Path::new(&site_dir);

    // where should the compiled markdown files end up?
    let build_dir = std::env::var("BUILD_DIR").unwrap_or(format!("../{}", crate::ROOT_SUBPATH));
    let build_path = std::path::Path::new(&build_dir);

    // how do we track file changes?
    let info_file = std::env::var("INFO_FILE").unwrap_or("./blag_info.json".to_string());
    let info_path = std::path::Path::new(&info_file);

    let mut tracking_info = persistence::TrackingInfo::read_from_file(&info_path);
    posts::generate_posts(&site_path, &mut tracking_info, &build_path);
    pages::generate_pages(&tracking_info, &build_path);
    tracking_info.write_to_file(&info_path);
    
    println!("static site generated successfully.");
}
