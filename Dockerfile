# Use the official Rust image.
# https://hub.docker.com/_/rust
FROM rust:1.27.0

# Copy local code to the container image.
WORKDIR /usr/dev/hello-doggy
COPY . .
ENV USER=aknk
RUN rustup default nightly && rustup update
# Install production dependencies and build a release artifact.
RUN cargo build
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
RUN wasm-pack build
RUN curl -sL https://deb.nodesource.com/setup_18.x -o nodesource_setup.sh
RUN apt-get update && apt-get install -y nodejs npm
# Service must listen to $PORT environment variable.
# This default value facilitates local development.
RUN npm install
RUN npm run start
ENV PORT 8080
