use std::borrow::Cow;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
struct Item<'a> {
    text: &'a str,
    number: f32,
    extra: Value,
    comments: Cow<'a, str>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
struct Container<'a> {
    #[serde(borrow)] // bind 'de (of the Deserializer impl for Container) to 'a
    inner: Vec<Item<'a>>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let json = include_str!("../example.json");

    // parse a JSON &str into the typed structure
    let mut data: Container = serde_json::from_str(json)?;

    dbg!(&data);

    // match on Value and modify Cow<&str> (in place)
    for item in data.inner.iter_mut() {
        if let Value::Bool(b) = item.extra {
            *item.comments.to_mut() = format!("extra is bool and equal to {}", b);
        }
    }

    // serialize the modified data back to a JSON string
    println!("{}", serde_json::to_string_pretty(&data)?);

    Ok(())
}
