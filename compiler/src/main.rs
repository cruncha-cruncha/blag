mod articles;
mod html;
mod info;
mod utils;

const INFO_FILE_NAME: &str = "articles.json";
const INPUT_DIR: &str = "../articles";
const OUTPUT_DIR: &str = "../docs";

fn main() {
    let upsert_only = is_upsert_only();
    run(upsert_only);
}

fn is_upsert_only() -> bool {
    let args: Vec<String> = std::env::args().collect();

    for arg in &args[1..] {
        // skip program name at args[0]
        match arg.as_str() {
            "--upsert-only" | "-u" => {
                println!("read --upsert-only flag");
                return true;
            }
            _ => {}
        }
    }

    false
}

fn run(upsert_only: bool) {
    println!("generating static site...");

    let mut info_wrangler = info::InfoWrangler::init();
    let article_paths = articles::Articles::must_get_article_paths();

    for path in &article_paths {
        info_wrangler.upsert(path);
    }

    if upsert_only {
        info_wrangler.save();
        println!("done!");
        return;
    }

    for path in &article_paths {
        articles::Articles::process(path, &mut info_wrangler);
    }

    info_wrangler.save();
    html::IndexHtml::save(&mut info_wrangler);

    println!("done!");
}
