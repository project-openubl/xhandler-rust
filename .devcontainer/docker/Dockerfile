FROM mcr.microsoft.com/devcontainers/rust:bullseye
RUN apt-get update && apt-get -y install pkg-config libssl-dev libxml2-dev libclang-dev
RUN apt update && \
    apt install sudo && \
    usermod -aG sudo vscode  && \
    echo "%sudo ALL=(ALL) NOPASSWD:ALL" | tee -a /etc/sudoers > /dev/null

RUN chown vscode:vscode -R /home/vscode
USER vscode