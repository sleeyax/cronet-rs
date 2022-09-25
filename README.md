# cronet-rs
Bindings and wrapper to cronet (Chromium Network Stack) in Rust. 

## Development
### Project setup
Clone & build the project:

```bash
$ git clone https://github.com/sleeyax/cronet-rs.git
$ cd cronet-rs
$ cargo build
```

### Generate bindings
1) Get the latest cronet binaries: [build from source](https://chromium.googlesource.com/chromium/src/+/refs/heads/main/components/cronet/build_instructions.md) or [download from a 3rd party](https://github.com/klzgrad/naiveproxy/releases).
2) Place all `.h` header files in `src` and all binaries (`.so`, `.dll`, `.dylib`) in `bin`.
3) Run `cargo build`. This should trigger `bindgen` to (re)generate the bindings.
