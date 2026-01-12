The file `articles.json` tracks all articles. Can define tags in there. It never clears articles though (big TODO). 

To run locally:
```
cargo run
```
`cd ../docs`, then serve:
```
python3 -m http.server
```

Links probably need to start with `/blag` in order to work when deployed.