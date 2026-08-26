# 🦀 LibTux 🐧

*Current Version: 0.1*

Rust's Standard Library extension for Linux systems, leveraging the standard libc and Linux specific syscalls for easier implementation of Linux-only software.

## 🚀 Quick Start

Edit your Cargo.toml file or run the following:
```sh
cargo add --git https://github.com/Yurand2000/libtux
```

## 📦 Library

The following is a non-exhaustive list of currently implemented features:

- Custom `Time` struct.
    + Time serialization with `serde`.
- Clock utilities from libc:
    + `Monotonic` and `Realtime` clocks supported.

## 🧩 Cargo-Features

| Feature Name | Default | Since | Docs | Description |
| -----------: | :-----: | ----: | :--: | :---------- |
| `clock` | ✔️ | 0.1 | ❌ | `Monotonic` and `Realtime` clock functions support |
| `serde` | ✔️ | 0.1 | ❌ | Enable Serialization/Deserialization of supported struct using `serde` |
| `time_extra` | ❌ | 0.1 | ❌ | Extra `Time`-specific structs and functions |

## 🔗 Dependencies

| Crate | Default | Features |
| ----: | :-----: | :------ |
| [`libc`](https://crates.io/crates/libc) | ✔️ | |
| [`serde`](https://crates.io/crates/serde) | ❌ | `serde` |

## 📄 License

This project is licensed under the Apache 2.0 license - see the [LICENSE](LICENSE) file for details.

## 👤 Author

This software was developed by:
- **Yuri Andriaccio** [![github](https://www.readmecodegen.com/api/social-icon?name=github&size=16&theme=dark&color=%23ffffff)](https://github.com/Yurand2000) - [yurand2000@gmail.com](mailto:yurand2000@gmail.com).

---

**LibTux**
