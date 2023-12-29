## dev-env

Starting:

```shell
docker-compose -f openubl/deploy/compose/compose.yaml up
```

```shell
RUST_LOG=info cargo watch -x 'run -p openubl-cli -- server --db-user user --db-password password'
```
