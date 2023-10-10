use std::fmt;

use serde::ser::{Serialize, SerializeMap, Serializer};
use serde_json::Value as JSValue;

#[derive(Debug)]
pub enum DugResult {
    Records(Vec<String>),
    Failure(String),
}

impl DugResult {
    pub fn from_records<I: IntoIterator<Item = String>>(iter: I) -> Self {
        DugResult::Records(Vec::from_iter(iter))
    }

    #[allow(clippy::needless_pass_by_value)]
    pub fn from_err(err: anyhow::Error) -> Self {
        DugResult::Failure(err.to_string())
    }
}

impl fmt::Display for DugResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DugResult::Records(recs) => write!(f, "{}", &recs.join(" ")),
            DugResult::Failure(fail) => write!(f, "{fail}"),
        }
    }
}

impl From<Result<Vec<String>, anyhow::Error>> for DugResult {
    fn from(res: Result<Vec<String>, anyhow::Error>) -> Self {
        match res {
            Ok(v) => DugResult::from_records(v),
            Err(e) => DugResult::from_err(e),
        }
    }
}

impl From<DugResult> for JSValue {
    fn from(dug_res: DugResult) -> JSValue {
        match dug_res {
            DugResult::Records(recs) => JSValue::from_iter(recs),
            DugResult::Failure(fail) => JSValue::from(fail),
        }
    }
}

#[derive(Debug)]
pub struct Resolution {
    pub name: String,
    pub source: String,
    pub result: DugResult,
}

impl Resolution {
    pub fn new(name: &str, src: &str, result: DugResult) -> Self {
        Resolution {
            name: name.to_string(),
            source: src.to_string(),
            result,
        }
    }

    pub fn with_err(name: &str, src: &str, err: anyhow::Error) -> Self {
        Resolution {
            name: name.to_string(),
            source: src.to_string(),
            result: DugResult::from_err(err),
        }
    }

    pub fn with_records<I: IntoIterator<Item = String>>(name: &str, src: &str, iter: I) -> Self {
        Resolution {
            name: name.to_string(),
            source: src.to_string(),
            result: DugResult::from_records(iter),
        }
    }
}

impl Serialize for Resolution {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(2))?;
        map.serialize_entry("name", &self.name)?;
        map.serialize_entry("source", &self.source)?;
        match &self.result {
            DugResult::Records(recs) => map.serialize_entry("records", &recs)?,
            DugResult::Failure(fail) => map.serialize_entry("failure", &fail)?,
        }
        map.end()
    }
}
