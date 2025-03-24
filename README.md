# A Toy Browser written in Rust
This repo stores a toy browser written in Rust, and is based on the book [sababook](https://github.com/d0iasm/sababook) by [d0iasm](https://github.com/d0iasm).

## Build Commands
This project wouldn't compile with a normal `cargo build`. Use either

```shell
./run_on_wasabi.sh
```
to run the browser in the toy OS provided ([wasabi](https://github.com/hikalium/wasabi)), or

```shell
cargo build --no-default-features
```
to build without running the OS.
