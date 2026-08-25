use std::collections::{BTreeMap, HashMap};

use iocore::Path;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    models::author::Author,
    traits::{FileSystemBytes, PlainBytes},
};

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Deserialize, Serialize)]
pub struct Accountability {
    #[serde(rename = "type")]
    kind: String,
    legally_bound: bool,
    certificates: AccountabilityCertificate,
    // #[serde(flatten)]
    // extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Deserialize, Serialize)]
pub enum AccountabilityPurpose {
    Signature,
    Encryption,
}



#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Deserialize, Serialize)]
pub struct AccountabilityCertificate {
    common_name: String,
    purpose: AccountabilityPurpose,
}
