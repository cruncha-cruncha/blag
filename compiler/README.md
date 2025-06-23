```
docker build -t blag-compiler .
```

```
docker run \
  -v "$(pwd)/../posts:/blag/posts" \
  -v "$(pwd)/../built:/blag/build" \
  -v "$(pwd)/blag_info.json:/blag_info.json" \
  blag-compiler
```

don't touch blag_info.json unless you want to

TODO: Define some commands in a Makefile: init, build. 'init' will attempt to build the rust code to an executable, and register a pre-commit hook. 'build' is designed to be run as a pre-commit hook, and will run the rust script (to update blag_info.json), then add blag_info.json to the commit. The github action will then build this blag using the already-generated blag_info.json (don't modify this file during the Github action)

TODO: build the docker image, upload it to docker hub, then pull it down using the github action; don't build the image during the action

```
docker build -t ghcr.io/cruncha-cruncha/blag-compiler:latest -f ./compiler/Dockerfile ./compiler
```

```
echo $CR_PAT | docker login ghcr.io -u cruncha-cruncha --password-stdin
```
```
docker push ghcr.io/cruncha-cruncha/blag-compiler:latest
```

```
- name: Pull Docker image
  run: docker pull ghcr.io/cruncha-cruncha/blag-compiler:latest

- name: Build static files
  run: |
    mkdir -p built
    docker run --rm \
      -v "$PWD/posts:/blag/posts" \
      -v "$PWD/built:/blag/build" \
      -v "$PWD/compiler/blag_info.json:/blag_info.json" \
      ghcr.io/cruncha-cruncha/blag-compiler:latest
```