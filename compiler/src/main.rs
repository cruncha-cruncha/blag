mod pages;
mod persistence;
mod posts;

fn main() {
    println!("generating static site...");
    let input_dir = std::path::Path::new("/blag/posts");
    let output_dir = std::path::Path::new("/blag/build");
    let mut file_info = persistence::read_info();
    posts::generate_posts(&input_dir, &mut file_info, &output_dir);
    pages::generate_pages(&file_info, &output_dir);
    persistence::write_info(&file_info);
    println!("static site generated successfully.");
}
