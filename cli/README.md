# Embedded TSDB Cli
> A cli helper use embedded tsdb lib

Every change proto files, build `cli` and publish generate code.

```shell
# Build with your build script:
BUILD_ENABLED=1 cargo build --package cli

# Build without your build script:
[BUILD_ENABLED=0] cargo build --package cli
```