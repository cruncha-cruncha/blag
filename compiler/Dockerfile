FROM rust:1.87 AS build

# create a new empty shell project
RUN USER=root cargo new --bin blag-compiler
WORKDIR /blag-compiler

# copy over the manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# cache dependencies
RUN cargo fetch

# remove default rust source files
RUN rm src/*.rs

# copy the source tree
COPY ./src ./src

# build for release
RUN cargo build --release

# our final base
FROM gcr.io/distroless/cc AS runtime

# copy artifact from the build stage
COPY --from=build /blag-compiler/target/release/blag-compiler .

# tell the script where to find things
ENV INPUT_DIR=/blag/site
ENV OUTPUT_DIR=/blag/build
ENV TRACKING_FILE=/blag_info.json
ENV REPO_NAME=blag

# set the startup command to run your binary
ENTRYPOINT ["/blag-compiler"]

