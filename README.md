<center>

# clonelicious

[![](https://img.shields.io/crates/v/clonelicious.svg)](https://crates.io/crates/clonelicious)
[![](https://img.shields.io/crates/d/clonelicious.svg)](https://img.shields.io/crates/d/clonelicious.svg)
[![](https://docs.rs/clonelicious/badge.svg)](https://docs.rs/clonelicious)
[![](https://github.com/crates-dev/clonelicious/workflows/Rust/badge.svg)](https://github.com/crates-dev/clonelicious/actions?query=workflow:Rust)
[![](https://img.shields.io/crates/l/clonelicious.svg)](./LICENSE)

</center>

[Official Documentation](https://docs.ltpp.vip/clonelicious/)

> A Rust macro library that simplifies cloning and closure execution. The `clone!` macro automatically clones variables and immediately executes the closure with the cloned values, streamlining common patterns in Rust programming.

## Installation

To install `clonelicious` run cmd:

```sh
cargo add clonelicious
```

## Usage

```rust
use clonelicious::*;

let s1: String = String::from("Hello");
let s2: String = String::from("World");
let res: String = clone!(s1, s2 => {
    assert_eq!(s1, String::from("Hello"));
    assert_eq!(s2, String::from("World"));
    format!("{} {}", s1, s2)
});
assert_eq!(res, format!("{} {}", s1, s2));

let s1: String = String::from("Hello");
let s2: String = String::from("World");
let res: String = clone!(s1, s2 => async move {
    assert_eq!(s1, String::from("Hello"));
    assert_eq!(s2, String::from("World"));
    format!("{} {}", s1, s2)
})
.await;
assert_eq!(res, format!("{} {}", s1, s2));

let s1: String = String::from("Hello");
let s2: String = String::from("World");
let res = clone!(s1, s2 => |data| {
    assert_eq!(s1, String::from("Hello"));
    assert_eq!(s2, String::from("World"));
    format!("{} {}{}", s1, s2, data)
});
assert_eq!(res("!"), format!("{} {}{}", s1, s2, "!"));

let s1: String = String::from("Hello");
let s2: String = String::from("World");
let res = clone!(s1, s2 =>  |data| async move {
    assert_eq!(s1, String::from("Hello"));
    assert_eq!(s2, String::from("World"));
    format!("{} {}{}", s1, s2, data)
});
assert_eq!(res("!").await, String::from("Hello World!"));

let s1: String = String::from("Hello");
let s2: String = String::from("World");
let res = clone!(s1, s2 => |data: &str| {
    assert_eq!(s1, String::from("Hello"));
    assert_eq!(s2, String::from("World"));
    format!("{} {}{}", s1, s2, data)
});
assert_eq!(res("!"), format!("{} {}{}", s1, s2, "!"));

let s1: String = String::from("Hello");
let s2: String = String::from("World");
let res = clone!(s1, s2 => |data: String| async move {
    assert_eq!(s1, String::from("Hello"));
    assert_eq!(s2, String::from("World"));
    format!("{} {}{}", s1, s2, data)
});
assert_eq!(res("!".to_owned()).await, format!("{} {}{}", s1, s2, "!"));

let s1: String = String::from("Hello");
let s2: String = String::from("World");
let res = clone!(s1, s2 => move |data| {
    assert_eq!(s1, String::from("Hello"));
    assert_eq!(s2, String::from("World"));
    format!("{} {}{}", s1, s2, data)
});
assert_eq!(res("!"), format!("{} {}{}", s1, s2, "!"));

let s1: String = String::from("Hello");
let s2: String = String::from("World");
let res = clone!(s1, s2 => move |data| async move {
    assert_eq!(s1, String::from("Hello"));
    assert_eq!(s2, String::from("World"));
    format!("{} {}{}", s1, s2, data)
});
assert_eq!(res("!").await, format!("{} {}{}", s1, s2, "!"));

let s1: String = String::from("Hello");
let s2: String = String::from("World");
let res = clone!(s1, s2 => move |data: &str| {
    assert_eq!(s1, String::from("Hello"));
    assert_eq!(s2, String::from("World"));
    format!("{} {}{}", s1, s2, data)
});
assert_eq!(res("!"), format!("{} {}{}", s1, s2, "!"));

let s1: String = String::from("Hello");
let s2: String = String::from("World");
let res = clone!(s1, s2 => move |data: String| async move {
    assert_eq!(s1, String::from("Hello"));
    assert_eq!(s2, String::from("World"));
    format!("{} {}{}", s1, s2, data)
});
assert_eq!(res("!".to_owned()).await, format!("{} {}{}", s1, s2, "!"));
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## Contact

For any inquiries, please reach out to the author at [root@ltpp.vip](mailto:root@ltpp.vip).

## Star History

![](https://api.star-history.com/svg?repos=crates-dev/clonelicious&type=Date)
