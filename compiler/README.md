# REDO PLANS

- keep the markdown parser
- simplify folder structure; make the blag more focused; remove subdirectories, and generate a single landing page
- rename blag_info.json to articles.json, and keep the single articles.json file for coordination, but add a lot to it
- file name is converted into URL. Title on main page is taken from articles.json (defaults to the file name). Articles will not show up on the main page unless they exist in articles.json and are set to archive: false
- look into last modified / created at behaviour:
    - we want to take the created_at time if it exists in articles.json
    - we want to always check the hash of articles, and update the updated_at time if it doesn't match the hash value in articles.json
- want to be able to define "tags" in articles.json. Generate a really simple bloom filter for the tags:
    - split every tag into trigrams; tag `"farming"` becomes `["far", "arm", "rmi", "min", "ing"]`. Tags shorter than 3 letters are ignored lol (with a compile time warning?)
    - run it through a hash
    - take the hash mod 1024
    - unset most of the bits deterministically, using some sort of deterministic sampling algorithm (use fisher-yates shuffle somehow, or 'reservoir'), so we're left with 2 or 3 bits set
    - 'OR' into a bigint (or a custom data structure?)
    - save to articles.json so we can use it on the frontend
- on the frontend, always order articles by date created
- on the frontend, want a search bar. Can search by title (just look through them lol) or by tag (using the bloom filters)
- at the bottom of each article, append: date created, date last modified, tags, next article, previous article
- on the frontend main page, show ten articles at most on a page. Should have 'next' and 'previous' buttons, as well as a link to my github.
- want the main page to read search and page number from URL search params
- layout of main page:
```
       Bug Blog <link to github> <link to legal disclaimer?>
       [search bar (no button)]
date - article title
date - article title
date - article title
       <prev> <next>
```
- favicon.ico


Use a bloom filter 2048 bits large, with two hash functions (k = 2), then we can store around 500 elements before the error rate goes above 5%. For the hash functions, actually use a single SHA1. First 11 bits (2^11 = 2048) -> first k, second 11 bits -> second k. Do all this in JS using bigint and subtle crypto, then store in localstorage or indexdb so we don't have to re-compute.

use TFIDF term frequency inverse document frequency?

instead of fetching articles.json, could build it into the html directly


# Compiler

High level:
- takes markdown files from INPUT_DIR and compiles them into OUTPUT_DIR
- organizes files into pages for easier reference
- tracks 'updated at' time outside of the filesystem
- is designed to work with git and github

Lower level:
- generates a landing page (index.html) from the first markdown file in INPUT_DIR. Errors if more than one markdown file found.
- steps into each folder in INPUT_DIR, and compiles any markdown files found there. Takes file `<INPUT_DIR>/rambling-thoughts/Cash is Green.md` and produces `<OUTPUT_DIR>/rambling-thoughts/cash-is-green.html`
- generates a set of pages for each folder in OUTPUT_DIR. So if there are 41 markdown files in `<INPUT_DIR>/rambling-thoughts/`, PAGE_SIZE is 20, and PAGES_INDEX_NAME is 'compendium', then it will generate three pages (41 = 20 + 20 + 1): `/<OUTPUT_DIR>/compendium/rambling-thoughts/1.html`, `/<OUTPUT_DIR>/compendium/rambling-thoughts/2.html`, and `/<OUTPUT_DIR>/compendium/rambling-thoughts/3.html`
- sorts posts on pages by their 'updated_at' time (the most recently updated post will be on page 1), which comes not from the filesystem but from whenever this compiler is run. Uses TRACKING_FILE_PATH (defaults to `blag_info.json`) to facilitate this.

Lower level:
- if OUTPUT_DIR is not specified, uses `../<REPO_NAME>` or `../build` (if REPO_NAME is undefined or blank) as the OUTPUT_DIR
- more info on env vars and how they're used can be found in the `Config` section of `main.rs`
- the updated at time is very strict, and a file is considered to be updated if the hash of it's contents is different from the hash stored in the tracking file

## Setup and Deployment

Read over `./init.sh` and run it if you want. It will set up a git pre-commit hook that runs the compiler before commit, which makes sure that the tracking file (blag_info.json) is up to date. This will ensure that updated at times are correct, and the posts on pages are sorted correctly. If the blag_info.json tracking file is out of date, posts may be sorted randomly. 

This folder is designed to work with a github action, which runs the code inside a Docker container and then publishes the resulting html. See the `Dockerfile` for more details.

To build and publish the docker image:
```
docker buildx build --platform linux/amd64 -t ghcr.io/cruncha-cruncha/blag-compiler:latest --push .
```

To run locally:
```
cargo run
```
`cd ..`, then serve:
```
python3 -m http.server
```
Hopefully all the links should work out as they would if the site was built for production.