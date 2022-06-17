[serde_string_trim][docsrs]:: serde deserialize_with string trim
========================================
[docsrs]: https://docs.rs/serde_string_trim

[![Chrono GitHub Actions](https://github.com/baoyachi/serde_string_trim/workflows/build/badge.svg)](https://github.com/baoyachi/serde_string_trim/actions?query=workflow%3Abuild)
[![Crates.io](https://img.shields.io/crates/v/serde_string_trim.svg)](https://crates.io/crates/serde_string_trim)
[![Docs.rs](https://docs.rs/serde_string_trim/badge.svg)](https://docs.rs/serde_string_trim)
[![Download](https://img.shields.io/crates/d/serde_string_trim)](https://crates.io/crates/serde_string_trim)
[![DepStatus](https://deps.rs/repo/github/baoyachi/serde_string_trim/status.svg)](https://deps.rs/repo/github/baoyachi/serde_string_trim)
[![Coverage Status](https://coveralls.io/repos/github/baoyachi/serde_string_trim/badge.svg)](https://coveralls.io/github/baoyachi/serde_string_trim)

## how to use
```rust
fn main() {
    #[derive(Debug, Deserialize)]
    struct Foo {
        #[serde(deserialize_with = "string_trim")]
        name: String,
    }
    let json = r#"{"name":" "}"#;
    let foo = serde_json::from_str::<Foo>(json).unwrap();
    assert_eq!(foo.name, "");
}
```