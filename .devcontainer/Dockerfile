# [Choice] Debian OS version (use bullseye on local arm64/Apple Silicon): buster, bullseye
ARG VARIANT="bullseye"
FROM mcr.microsoft.com/vscode/devcontainers/rust:1-${VARIANT}
# FROM node:18-bullseye-slim
# [Optional] Uncomment this section to install additional packages.
# RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
#     && apt-get -y install --no-install-recommends <your-package-list-here>
# RUN apt-get install software-properties-common
# RUN apt-get update && add-apt-repository --remove ppa:ubuntu-toolchain-r/test -y \
#   apt update \
# env ACCEPT_EULA=Y apt upgrade \
#   apt-get purge gcc-5 gcc-5-base gcc-6-base \
#   apt install gcc-multilib
WORKDIR /workspace
COPY . .

RUN apt-get update && apt-get -y install postgresql-client npm nodejs
RUN rustup update && rustup update nightly
RUN rustup target add  wasm32-unknown-unknown
RUN cargo install diesel_cli
RUN curl -L git.io/nodebrew | perl - setup
RUN curl -y https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
RUN npm i --save
ENV TOKEN="eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJyb2xlIjoiZXhlY191c2VyIn0.sGEcLCy9zzmKScoWga7DVUyBWr8Cz2caAh-uF29_n9U"
CMD ["cd ./bin", "./postgrest ../postgrest.conf"]
ENV PATH="$HOME/.nodebrew/current/bin:$PATH"
RUN npm install -g n && n stable
# CMD export PATH=export PATH=$HOME/bin:$PATH
EXPOSE 3000 8080
