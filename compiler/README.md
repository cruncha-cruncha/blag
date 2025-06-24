Try not to modify blag_info.json manually.

Read over `./init.sh`, run it if you want.

The `Dockerfile` is used by a Github action.

```
docker build -t ghcr.io/cruncha-cruncha/blag-compiler:latest .
```

```
docker push ghcr.io/cruncha-cruncha/blag-compiler:latest
```

but actually, we need to build for linux/amd64/v3, so:

```
docker buildx create --use
```

```
docker buildx build --platform linux/amd64 -t ghcr.io/cruncha-cruncha/blag-compiler:latest --push .
```