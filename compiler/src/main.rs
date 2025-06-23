mod pages;
mod persistence;
mod posts;

fn main() {
    println!("generating static site...");

    let posts_dir =  std::env::var("POSTS_DIR").unwrap_or("../posts".to_string());
    let posts_path = std::path::Path::new(&posts_dir);

    let build_dir = std::env::var("BUILD_DIR").unwrap_or("../build".to_string());
    let build_path = std::path::Path::new(&build_dir);

    let info_file = std::env::var("INFO_FILE").unwrap_or("./blag_info.json".to_string());
    let info_path = std::path::Path::new(&info_file);

    let mut file_info = persistence::read_info(&info_path);
    posts::generate_posts(&posts_path, &mut file_info, &build_path);
    pages::generate_pages(&file_info, &build_path);
    persistence::write_info(&file_info, &info_path);
    
    println!("static site generated successfully.");
}
