[serde_trim][docsrs]:: serde deserialize_with string trim
========================================
[docsrs]: https://docs.rs/serde_trim

[![GitHub Actions](https://github.com/baoyachi/serde_trim/workflows/check/badge.svg)](https://github.com/baoyachi/serde_trim/actions?query=workflow%3Acheck)
[![Crates.io](https://img.shields.io/crates/v/serde_trim.svg)](https://crates.io/crates/serde_trim)
[![Docs.rs](https://docs.rs/serde_trim/badge.svg)](https://docs.rs/serde_trim)
[![Download](https://img.shields.io/crates/d/serde_trim)](https://crates.io/crates/serde_trim)
[![DepStatus](https://deps.rs/repo/github/baoyachi/serde_trim/status.svg)](https://deps.rs/repo/github/baoyachi/serde_trim)
[![Coverage Status](https://coveralls.io/repos/github/baoyachi/serde_trim/badge.svg)](https://coveralls.io/github/baoyachi/serde_trim)

## Support trim
* `String`
* `Option<String>`
* `Vec<String>`
* `BTreeSet<String>`
* `HashSet<String>`
* `VecDeque<String>`
* `LinkedList<String>`
* `BinaryHeap<String>`

Supports multiple [std::collections](https://doc.rust-lang.org/stable/std/collections/) types
 


## how to use
```rust
use serde_derive::Deserialize;
use serde_trim::*;
use std::collections::*;

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

    #[derive(Deserialize)]
    struct OptionBar {
        #[serde(default, deserialize_with = "option_string_trim")]
        name: Option<String>,
        addr: String,
    }
    let json = r#"{"addr":"ABC"}"#;
    let foo = serde_json::from_str::<OptionBar>(json).unwrap();
    assert_eq!(foo.name, None);
    assert_eq!(foo.addr, "ABC");

    #[derive(Deserialize)]
    struct VecFoo {
        #[serde(deserialize_with = "vec_string_trim")]
        name: Vec<String>,
    }
    let json = r#"{"name":["   ","foo","b ar","hello ","  rust"]}"#;
    let foo = serde_json::from_str::<VecFoo>(json).unwrap();
    assert_eq!(foo.name, vec!["", "foo", "b ar", "hello", "rust"]);

    #[derive(Deserialize)]
    struct BTreeSetFoo {
        #[serde(deserialize_with = "btreeset_string_trim")]
        name: BTreeSet<String>,
    }
    let json = r#"{"name":["   ","foo","b ar","hello ","  rust"]}"#;
    let foo = serde_json::from_str::<BTreeSetFoo>(json).unwrap();
    let expected: BTreeSet<String> = BTreeSet::from_iter([
        "".into(),
        "foo".into(),
        "b ar".into(),
        "hello".into(),
        "rust".into(),
    ]);
    assert_eq!(foo.name, expected);

    #[derive(Deserialize)]
    struct HashSetFoo {
        #[serde(deserialize_with = "hashset_string_trim")]
        name: HashSet<String>,
    }
    let json = r#"{"name":["   ","foo","b ar","hello ","  rust"]}"#;
    let foo = serde_json::from_str::<HashSetFoo>(json).unwrap();
    let expected: HashSet<String> = HashSet::from_iter([
        "".into(),
        "foo".into(),
        "b ar".into(),
        "hello".into(),
        "rust".into(),
    ]);
    assert_eq!(foo.name, expected);

    #[derive(Deserialize)]
    struct VecDequeFoo {
        #[serde(deserialize_with = "vecdeque_string_trim")]
        name: VecDeque<String>,
    }
    let json = r#"{"name":["   ","foo","b ar","hello ","  rust"]}"#;
    let foo = serde_json::from_str::<VecDequeFoo>(json).unwrap();
    assert_eq!(foo.name, vec!["", "foo", "b ar", "hello", "rust"]);

    #[derive(Deserialize)]
    struct LinkedListFoo {
        #[serde(deserialize_with = "linkedlist_string_trim")]
        name: LinkedList<String>,
    }
    let json = r#"{"name":["   ","foo","b ar","hello ","  rust"]}"#;
    let foo = serde_json::from_str::<LinkedListFoo>(json).unwrap();
    assert_eq!(
        foo.name,
        LinkedList::from_iter([
            "".into(),
            "foo".into(),
            "b ar".into(),
            "hello".into(),
            "rust".into(),
        ])
    );

    #[derive(Deserialize)]
    struct BinaryHeapFoo {
        #[serde(deserialize_with = "binaryheap_string_trim")]
        name: BinaryHeap<String>,
    }
    let json = r#"{"name":["   ","foo","b ar","hello ","  rust"]}"#;
    let foo = serde_json::from_str::<BinaryHeapFoo>(json).unwrap();
    assert_eq!(
        foo.name.into_vec(),
        vec!["rust", "hello", "b ar", "", "foo"]
    );
}

```
