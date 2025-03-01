sudo dnf install -y @development-tools
sudo dnf install -y @c-development
sudo dnf install -y libxml2-devel openssl-devel gcc gcc-c++ cmake perl

## Install Rust
sudo dnf install -y rustup

## Install NVM
curl -s https://raw.githubusercontent.com/devcontainers/features/refs/heads/main/src/node/install.sh | sudo VERSION=22 bash

## Configure Rust
rustup-init -y
. "$HOME/.cargo/env"
rustup update
