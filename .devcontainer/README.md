# Devcontainers

Use [Devcontainers](https://code.visualstudio.com/docs/devcontainers/containers) to prepare a fully automated working environment.

### Docker

Docker defaults should work fine therefore there is nothing to do.

### Podman

Start Podman service for a regular user (rootless) and make it listen to a socket:

```shell
systemctl --user enable --now podman.socket
```

Restart your OS if necessary and verify that podman listens:

```shell
systemctl --user status podman.socket
```

## VSCode

Install the extension https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers

Only if you use podman, therefore Optional:

Go to the Extension Settings:

- `Dev › Containers: Docker Compose Path` set `podman-compose`
- `Dev › Containers: Docker Path` set `podman`
- `Dev › Containers: Docker Socket Path` set `/run/podman/podman.sock`

To open the repository with DevContainers do `Ctrl + Shift + P` and enter `Dev Containers: Rebuild and Reopen in Container` or `Dev Containers: Reopen in Container`. For more options see the Extension documentation.
