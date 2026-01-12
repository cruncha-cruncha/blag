# REDO PLANS

Use a bloom filter 2048 bits large, with two hash functions (k = 2), then we can store around 500 elements before the error rate goes above 5%. For the hash functions, actually use a single SHA1. First 11 bits (2^11 = 2048) -> first k, second 11 bits -> second k. Do all this in JS using bigint and subtle crypto, then store in localstorage or indexdb so we don't have to re-compute.

use TFIDF term frequency inverse document frequency?


To run locally:
```
cargo run
```
`cd ..`, then serve:
```
python3 -m http.server
```
Hopefully all the links should work out as they would if the site was built for production.