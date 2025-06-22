```
docker build -t blag-compiler .
```

```
docker run \
  -v "$(pwd)/../posts:/blag/posts" \
  -v "$(pwd)/../built:/blag/build" \
  -v "$(pwd)/blag_info.bin:/blag_info.bin" \
  blag-compiler
```

don't touch blag_info.bin