## dev-env

Starting:

```shell
docker-compose -f ublhub/deploy/compose/compose.yaml up
```

```shell
RUST_LOG=info cargo watch -x 'run -p ublhub-cli -- server --db-user user --db-password password'
```
