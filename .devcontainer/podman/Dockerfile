FROM quay.io/fedora/fedora:41

ARG USERNAME=vscode
ARG USER_UID=1000
ARG USER_GID=$USER_UID

COPY entrypoint.sh /entrypoint.sh

RUN dnf -y update && \
    dnf install -y @development-tools && \
    dnf install -y curl wget podman

RUN groupadd --gid $USER_GID $USERNAME && \
    useradd --uid $USER_UID --gid $USER_GID -m $USERNAME && \
    echo $USERNAME:10000:5000 > /etc/subuid && echo $USERNAME:10000:5000 > /etc/subgid && \
    # Allow user to execute 'sudo' without password
    usermod -aG wheel $USERNAME && \
    echo "%wheel ALL=(ALL) NOPASSWD:ALL" | tee -a /etc/sudoers > /dev/null

# Podman
# https://github.com/containers/podman/issues/2788#issuecomment-479923274
RUN chmod 4755 /usr/bin/newgidmap && chmod 4755 /usr/bin/newuidmap

# set permissions
RUN chown $USERNAME:$USERNAME -R /home/$USERNAME

RUN usermod -aG wheel $USERNAME && \
    # Allow user to execute 'sudo' without password
    echo "%wheel ALL=(ALL) NOPASSWD:ALL" | tee -a /etc/sudoers > /dev/null && \
    # https://github.com/containers/podman/issues/2788#issuecomment-479923274
    chmod 4755 /usr/bin/newgidmap && chmod 4755 /usr/bin/newuidmap

ENV _CONTAINERS_USERNS_CONFIGURED=""

ENTRYPOINT [ "/entrypoint.sh" ]
USER $USERNAME
CMD ["tail", "-f", "/dev/null"]
