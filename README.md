# 🦀 LibTux 🐧

*Current Version: 0.1.3*

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
- Get/Set process scheduler and affinity.
- Send signal to process.
- Setup signal handler for process.

## 🧩 Cargo-Features

| Feature Name | Default | Since | Docs | Description |
| -----------: | :-----: | ----: | :--: | :---------- |
| `clock` | ✔️ | 0.1.0 | ❌ | `Monotonic` and `Realtime` clock functions support |
| `cpuset` | ✔️ | 0.1.2 | ❌ | Get and set process cpu affinity |
| `time` | ✔️ | 0.1.2 | ❌ | Time constructs. |
||||| + Time serialization with `serde`. |
| `time_extra` | ❌ | 0.1.0 | ❌ | Extra Time-specific structs and functions |
| `time_iterators` | ✔️ | 0.1.1 | ❌ | Iterators for Time ranges |
| `sched` | ✔️ | 0.1.2 | ❌ | Get and set process scheduler |
| `signal` | ✔️ | 0.1.2 | ❌ | Send signal to process |
| |
| `serde` | ✔️ | 0.1.0 | ❌ | Enable Serialization/Deserialization of supported struct using `serde` |
| `parser` | ✔️ | 0.1.2 | ❌ | Enable complex string-parsers for supported types. |

## 🔗 Dependencies

| Crate | Optional | Features |
| ----: | :-----: | :------ |
| [`libc`](https://crates.io/crates/libc) | ✔️ | `clock`, `cpuset`, `sched`, `signal` |
| [`bitflags`](https://crates.io/crates/bitflags) | ✔️ | `sched` |
| [`serde`](https://crates.io/crates/serde) | ✔️ | `serde` |
| [`nom`](https://crates.io/crates/nom) | ✔️ | `parser` |

## 📄 License

This project is licensed under the Apache 2.0 license - see the [LICENSE](LICENSE) file for details.

## 👤 Author

This software was developed by:
- **Yuri Andriaccio** [![github](https://www.readmecodegen.com/api/social-icon?name=github&size=16&theme=dark&color=%23ffffff)](https://github.com/Yurand2000) - [yurand2000@gmail.com](mailto:yurand2000@gmail.com).
- **Giuseppe Capasso** [![github](https://www.readmecodegen.com/api/social-icon?name=github&size=16&theme=dark&color=%23ffffff)](https://github.com/alarmfox) - [capassog97@gmail.com](mailto:capassog97@gmail.com).

---

**LibTux**
