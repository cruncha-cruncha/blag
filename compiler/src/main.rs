mod posts;
mod pages;

fn main() {
    let input_dir = std::path::Path::new("/blag/posts");
    let output_dir = std::path::Path::new("/blag/build");
    posts::generate_posts(&input_dir, &output_dir);
    pages::generate_pages(&input_dir, &output_dir);
}
