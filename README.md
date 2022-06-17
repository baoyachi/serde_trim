[serde_trim][docsrs]:: serde deserialize_with string trim
========================================
[docsrs]: https://docs.rs/serde_trim

[![GitHub Actions](https://github.com/baoyachi/serde_trim/workflows/check/badge.svg)](https://github.com/baoyachi/serde_trim/actions?query=workflow%3Acheck)
[![Crates.io](https://img.shields.io/crates/v/serde_trim.svg)](https://crates.io/crates/serde_trim)
[![Docs.rs](https://docs.rs/serde_trim/badge.svg)](https://docs.rs/serde_trim)
[![Download](https://img.shields.io/crates/d/serde_trim)](https://crates.io/crates/serde_trim)
[![DepStatus](https://deps.rs/repo/github/baoyachi/serde_trim/status.svg)](https://deps.rs/repo/github/baoyachi/serde_trim)
[![Coverage Status](https://coveralls.io/repos/github/baoyachi/serde_trim/badge.svg)](https://coveralls.io/github/baoyachi/serde_trim)

## how to use
```rust
fn main() {
    #[derive(Deserialize)]
    struct Foo {
        #[serde(deserialize_with = "string_trim")]
        name: String,
    }
    let json = r#"{"name":" "}"#;
    let foo = serde_json::from_str::<Foo>(json).unwrap();
    assert_eq!(foo.name, "");

    #[derive(Deserialize)]
    struct OptionFoo {
        #[serde(deserialize_with = "option_string_trim")]
        name: Option<String>,
    }
    let json = r#"{"name":" "}"#;
    let foo = serde_json::from_str::<OptionFoo>(json).unwrap();
    assert_eq!(foo.name, None);
}
```