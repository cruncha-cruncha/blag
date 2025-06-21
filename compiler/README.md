```
docker build -t blag-compiler .
```

```
docker run \
  -v "$(pwd)/../posts:/blag/posts" \
  -v "$(pwd)/../built:/blag/build" \
  blag-compiler
```