use std::collections::*;

use serde::{de, Deserialize};
pub use trim_in_place::*;

pub fn string_trim<'de, D>(d: D) -> Result<String, D::Error>
where
    D: de::Deserializer<'de>,
{
    let mut de_string = String::deserialize(d)?;
    de_string.trim_in_place();
    Ok(de_string)
}

pub fn option_string_trim<'de, D>(d: D) -> Result<Option<String>, D::Error>
where
    D: de::Deserializer<'de>,
{
    let mut de_string: Option<String> = Option::deserialize(d)?;
    if let Some(ref mut de_string) = de_string {
        if de_string.trim_in_place().is_empty() {
            return Ok(None);
        }
    }
    Ok(de_string)
}

macro_rules! iter_trim {
    ($fn_name:ident,$t:ty) => {
        pub fn $fn_name<'de, D>(d: D) -> Result<$t, D::Error>
        where
            D: de::Deserializer<'de>,
        {
            collection_trim(d)
        }
    };
}

macro_rules! iter_non_empty_trim {
    ($fn_name:ident,$t:ty) => {
        pub fn $fn_name<'de, D>(d: D) -> Result<$t, D::Error>
        where
            D: de::Deserializer<'de>,
        {
            collection_non_empty_trim(d)
        }
    };
}

iter_trim!(btreeset_string_trim, BTreeSet<String>);
iter_trim!(vec_string_trim, Vec<String>);
iter_trim!(hashset_string_trim, HashSet<String>);
iter_trim!(vecdeque_string_trim, VecDeque<String>);
iter_trim!(linkedlist_string_trim, LinkedList<String>);
iter_trim!(binaryheap_string_trim, BinaryHeap<String>);

iter_non_empty_trim!(btreeset_non_empty_string_trim, BTreeSet<String>);
iter_non_empty_trim!(vec_non_empty_string_trim, Vec<String>);
iter_non_empty_trim!(hashset_non_empty_string_trim, HashSet<String>);
iter_non_empty_trim!(vecdeque_non_empty_string_trim, VecDeque<String>);
iter_non_empty_trim!(linkedlist_non_empty_string_trim, LinkedList<String>);
iter_non_empty_trim!(binaryheap_non_empty_string_trim, BinaryHeap<String>);

fn collection_trim<'de, C, D, S>(d: D) -> Result<C, D::Error>
where
    S: TrimInPlace,
    C: Deserialize<'de> + IntoIterator<Item = S> + FromIterator<String>,
    D: de::Deserializer<'de>,
{
    let de_string: C = C::deserialize(d)?
        .into_iter()
        .map(|mut x| x.trim_in_place().to_string())
        .collect();
    Ok(de_string)
}

fn collection_non_empty_trim<'de, C, D, S>(d: D) -> Result<C, D::Error>
where
    S: TrimInPlace,
    C: Deserialize<'de> + IntoIterator<Item = S> + FromIterator<String>,
    D: de::Deserializer<'de>,
{
    let de_string: C = C::deserialize(d)?
        .into_iter()
        .filter_map(|mut x| {
            let x = x.trim_in_place().to_string();
            if x.is_empty() {
                None
            } else {
                Some(x)
            }
        })
        .collect();
    Ok(de_string)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_trim() {
        #[derive(Deserialize)]
        struct Foo {
            #[serde(deserialize_with = "string_trim")]
            name: String,
        }
        let json = r#"{"name":" "}"#;
        let foo = serde_json::from_str::<Foo>(json).unwrap();
        assert_eq!(foo.name, "");
    }

    #[test]
    fn test_option_string_trim() {
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
    }

    #[test]
    fn test_vec_string_trim() {
        #[derive(Deserialize)]
        struct VecFoo {
            #[serde(deserialize_with = "vec_string_trim")]
            name: Vec<String>,
        }
        let json = r#"{"name":["   ","foo","b ar","hello ","  rust"]}"#;
        let foo = serde_json::from_str::<VecFoo>(json).unwrap();
        assert_eq!(foo.name, vec!["", "foo", "b ar", "hello", "rust"]);
    }

    #[test]
    fn test_btreeset_string_trim() {
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
    }

    #[test]
    fn test_hashset_string_trim() {
        #[derive(Deserialize)]
        struct HashSetFoo {
            #[serde(deserialize_with = "hashset_string_trim")]
            name: HashSet<String>,
        }
        let json = r#"{"name":["   ","foo","b ar","hello ","  rust","  rust"]}"#;
        let foo = serde_json::from_str::<HashSetFoo>(json).unwrap();
        let expected: HashSet<String> = HashSet::from_iter([
            "".into(),
            "foo".into(),
            "b ar".into(),
            "hello".into(),
            "rust".into(),
        ]);
        assert_eq!(foo.name, expected);
    }

    #[test]
    fn test_vecdeque_string_trim() {
        #[derive(Deserialize)]
        struct VecDequeFoo {
            #[serde(deserialize_with = "vecdeque_string_trim")]
            name: VecDeque<String>,
        }
        let json = r#"{"name":["   ","foo","b ar","hello ","  rust"]}"#;
        let foo = serde_json::from_str::<VecDequeFoo>(json).unwrap();
        assert_eq!(foo.name, vec!["", "foo", "b ar", "hello", "rust"]);
    }

    #[test]
    fn test_linkedlist_string_trim() {
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
    }

    #[test]
    fn test_binaryheap_string_trim() {
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

    #[test]
    fn test_vec_non_empty_string_trim() {
        #[derive(Deserialize)]
        struct VecFoo {
            #[serde(deserialize_with = "vec_non_empty_string_trim")]
            name: Vec<String>,
        }
        let json = r#"{"name":["   ","foo","b ar","hello ","  rust"]}"#;
        let foo = serde_json::from_str::<VecFoo>(json).unwrap();
        assert_eq!(foo.name, vec!["foo", "b ar", "hello", "rust"]);
    }

    #[test]
    fn test_btreeset_non_empty_string_trim() {
        #[derive(Deserialize)]
        struct BTreeSetFoo {
            #[serde(deserialize_with = "btreeset_non_empty_string_trim")]
            name: BTreeSet<String>,
        }
        let json = r#"{"name":["   ","foo","b ar","hello ","  rust"]}"#;
        let foo = serde_json::from_str::<BTreeSetFoo>(json).unwrap();
        let expected: BTreeSet<String> =
            BTreeSet::from_iter(["foo".into(), "b ar".into(), "hello".into(), "rust".into()]);
        assert_eq!(foo.name, expected);
    }

    #[test]
    fn test_hashset_non_empty_string_trim() {
        #[derive(Deserialize)]
        struct HashSetFoo {
            #[serde(deserialize_with = "hashset_non_empty_string_trim")]
            name: HashSet<String>,
        }
        let json = r#"{"name":["   ","foo","b ar","hello ","  rust","  rust"]}"#;
        let foo = serde_json::from_str::<HashSetFoo>(json).unwrap();
        let expected: HashSet<String> =
            HashSet::from_iter(["foo".into(), "b ar".into(), "hello".into(), "rust".into()]);
        assert_eq!(foo.name, expected);
    }

    #[test]
    fn test_vecdeque_non_empty_string_trim() {
        #[derive(Deserialize)]
        struct VecDequeFoo {
            #[serde(deserialize_with = "vecdeque_non_empty_string_trim")]
            name: VecDeque<String>,
        }
        let json = r#"{"name":["   ","foo","b ar","hello ","  rust"]}"#;
        let foo = serde_json::from_str::<VecDequeFoo>(json).unwrap();
        assert_eq!(foo.name, vec!["foo", "b ar", "hello", "rust"]);
    }

    #[test]
    fn test_linkedlist_non_empty_string_trim() {
        #[derive(Deserialize)]
        struct LinkedListFoo {
            #[serde(deserialize_with = "linkedlist_non_empty_string_trim")]
            name: LinkedList<String>,
        }
        let json = r#"{"name":["   ","foo","b ar","hello ","  rust"]}"#;
        let foo = serde_json::from_str::<LinkedListFoo>(json).unwrap();
        assert_eq!(
            foo.name,
            LinkedList::from_iter(["foo".into(), "b ar".into(), "hello".into(), "rust".into(),])
        );
    }

    #[test]
    fn test_binaryheap_non_empty_string_trim() {
        #[derive(Deserialize)]
        struct BinaryHeapFoo {
            #[serde(deserialize_with = "binaryheap_non_empty_string_trim")]
            name: BinaryHeap<String>,
        }
        let json = r#"{"name":["   ","foo","b ar","hello ","  rust"]}"#;
        let foo = serde_json::from_str::<BinaryHeapFoo>(json).unwrap();
        assert_eq!(foo.name.into_vec(), vec!["rust", "hello", "b ar", "foo"]);
    }
}
