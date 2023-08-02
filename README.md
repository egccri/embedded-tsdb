# embedded-tsdb
> Light-weight embedded TSDB

```shell
cargo build --package embedded-tsdb-server
cargo build --package client

./target/debug/embedded-tsdb-server 0.0.0.0:9999
./target/debug/embedded-tsdb-client -s 127.0.0.1:9999 exec "select 1"
```
