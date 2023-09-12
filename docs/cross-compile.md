# Cross Compile

#### embedded-tsdb

```shell
# MAC m2
# CROSS_CONTAINER_OPTS="--platform linux/amd64" 
cross build --package embedded-tsdb --target aarch64-unknown-linux-gnu
```

#### embedded-tsdb-server

```shell
# Need add pre build bin, like protoc
cd bin/server
# MAC m2
# CROSS_CONTAINER_OPTS="--platform linux/amd64" 
cross build --package embedded-tsdb-server --target aarch64-unknown-linux-gnu
```