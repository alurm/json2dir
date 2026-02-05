use super::{Error, Result};
use serde_json as json;
use std::collections::HashMap;

#[derive(Debug)]
pub(crate) struct Selection(pub Option<HashMap<String, Selection>>);

pub(crate) fn from_json(json: json::Value) -> Result<Selection> {
    Ok(Selection(match json {
        json::Value::Null => None,
        serde_json::Value::Object(map) => Some({
            let mut hm = HashMap::new();
            for (k, v) in map {
                hm.insert(k, from_json(v)?);
            }
            hm
        }),
        _ => return Err(Error::InvalidJsonType),
    }))
}

#[test]
fn test() {
    let _everything = Selection(None);

    let _nothing = Selection(Some(HashMap::from([])));

    let _foo = Selection(Some(HashMap::from([("foo".into(), Selection(None))])));
}
