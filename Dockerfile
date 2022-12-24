# Use the official Rust image.
# https://hub.docker.com/_/rust
FROM rust:1.27.0
ENV HOME="/usr/dev/hello-doggy/www" \
    LANG=C.UTF-8 \
    TZ=Asia/Tokyo
# Copy local code to the container image.
WORKDIR /usr/dev/hello-doggy
COPY ./ /usr/dev/hello-doggy
ENV USER=aknk
RUN rustup default nightly && rustup update
# Install production dependencies and build a release artifact.
RUN cargo build
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
RUN wasm-pack build
RUN curl -sL https://deb.nodesource.com/setup_16.x | sh
RUN apt update && apt install nodejs npm -y
# Service must listen to $PORT environment variable.
# This default value facilitates local development.
RUN npm install --prefix ./www
ENV HOST 0.0.0.0
EXPOSE 8080
CMD ["npm", "run", "start", "--prefix", "./www"]
