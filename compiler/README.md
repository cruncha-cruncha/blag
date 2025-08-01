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