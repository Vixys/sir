FROM mcr.microsoft.com/devcontainers/rust:0-1-bullseye

RUN apt-get update

USER vscode

RUN cargo install cargo-tarpaulin \
    && rustup component add llvm-tools-preview