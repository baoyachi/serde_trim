use serde::{de, Deserialize};
use trim_in_place::TrimInPlace;

pub fn string_trim<'de, D>(d: D) -> Result<String, D::Error>
    where
        D: de::Deserializer<'de>,
{
    let mut de_string = String::deserialize(d)?;
    de_string.trim_in_place();
    Ok(de_string)
}

#[test]
fn test_string_trim() {
    #[derive(Debug, Deserialize)]
    struct FooDeserialize {
        #[serde(deserialize_with = "string_trim")]
        name: String,
    }

    #[derive(Debug, Deserialize)]
    struct Foo {
        name: String,
    }

    let json = r#"{"name":" "}"#;
    let person = serde_json::from_str::<FooDeserialize>(json).unwrap();
    assert_eq!(person.name, "");

    let person = serde_json::from_str::<Foo>(json).unwrap();
    assert_eq!(person.name, " ");
}