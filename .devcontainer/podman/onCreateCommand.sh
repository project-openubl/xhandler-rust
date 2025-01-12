sudo dnf install -y @development-tools
sudo dnf install -y @c-development
sudo dnf install -y libxml2-devel openssl-devel gcc gcc-c++ cmake perl

sudo dnf install -y rustup
rustup-init -y
. "$HOME/.cargo/env"
rustup update
