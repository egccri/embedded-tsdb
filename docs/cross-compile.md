# Cross Compile Local

#### Install Cross

Use cross need docker.

```shell
cargo install cross
```

#### embedded-tsdb

```shell
# MAC m2
# CROSS_CONTAINER_OPTS="--platform linux/amd64" 
cross build --package embedded-tsdb --target aarch64-unknown-linux-gnu
```

#### embedded-tsdb-server

```shell
# MAC m2
# CROSS_CONTAINER_OPTS="--platform linux/amd64" 
cross build --package embedded-tsdb-server --target aarch64-unknown-linux-gnu
```

#### cli

If protoc has updated, See carte `cli`.