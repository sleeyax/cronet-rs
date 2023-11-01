# cronet-rs 🦀

Bindings to the Chromium Networking Stack (a.k.a cronet) in Rust.

> :warning: This crate is under active development and the API is not yet stable.

## Developers

First of all, clone the project:

```bash
$ git clone https://github.com/sleeyax/cronet-rs.git
$ cd cronet-rs
```

Then, follow the steps to build the project:

1. Get the latest cronet binaries: [build from source](https://chromium.googlesource.com/chromium/src/+/refs/heads/main/components/cronet/build_instructions.md) or get prebuilt binaries from somewhere (if you know a reputable source, [let me know](https://github.com/sleeyax/cronet-rs/issues/new)!).
2. Place all `.h` header files in `src` and all binaries (`.so`, `.dll`, `.dylib`) in `bin`.
3. Run `cargo build`. This should trigger `bindgen` to (re)generate the bindings.

## Related projects

Other projects that are related to this project and might interest you:

- [Cronet in C#](https://github.com/sleeyax/CronetSharp)
- [Cronet in go](https://github.com/SagerNet/cronet-go)
- [NaïveProxy](https://github.com/klzgrad/naiveproxy)
