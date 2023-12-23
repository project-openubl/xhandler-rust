## dev-env

Starting:

```shell
docker-compose -f deploy/compose/compose.yaml up
```

```shell
RUST_LOG=info cargo run -p ublhub-cli -- server --db-user user --db-password password
```
