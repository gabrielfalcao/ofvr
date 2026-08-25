use std::{
    cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd},
    collections::{BTreeMap, HashMap},
    fmt,
    hash::{Hash, Hasher},
    ops::{Add, Deref, Div, Mul, Rem, Sub},
    str::FromStr,
};

use iocore::Path;
use serde::{
    Deserialize,
    Deserializer,
    Serialize,
    Serializer,
    de::{Error as SerdeError, Visitor},
};

pub trait StringKeyMapValue:
PartialEq + PartialOrd + Eq + Ord + Hash + Deserialize + Serialize + FromStr
{
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct StringKeyMap<T: StringKeyMapValue> {
    #[serde(flatten)]
    data: BTreeMap<String, T>,
}

impl<T> StringKeyMap<T> where T: PartialEq + PartialOrd + Eq + Ord + Hash + Deserialize + Serialize + FromStr {
    pub fn len(&self)->usize {
        self.data.len()
    }
}

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//  <<<  SSSSS   EEEEEEE  RRRRRR   DDDDD    EEEEEEE >>>
// <<<  SS       EE       RR   RR  DD  DD   EE       >>>
// <<<   SSSSS   EEEEE    RRRRRR   DD   DD  EEEEE    >>>
// <<<       SS  EE       RR  RR   DD   DD  EE       >>>
//  <<<  SSSSS   EEEEEEE  RR   RR  DDDDDD   EEEEEEE >>>

impl Serialize for StringKeyMap {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.len()))?;
        for (k, v) in self.data.iter {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

struct ValueVisitor;

impl<'de> Visitor<'de> for ValueVisitor {
    type StringKeyMap = StringKeyMap;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a serializable value")
    }

    fn visit_str<E>(self, value: &str) -> std::result::Result<Self::StringKeyMap, E>
    where
        E: SerdeError,
    {
        match StringKeyMap::from_str(value) {
            Ok(value) => Ok(value),
            Err(error) => {
                Err(E::custom(format!("invalid hexadecimal string of length 2: {error}")))
            }
        }
    }

    fn visit_string<E>(self, value: String) -> std::result::Result<Self::StringKeyMap, E>
    where
        E: SerdeError,
    {
        self.visit_str(&value)
    }
}

impl<'de> Deserialize<'de> for StringKeyMap {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(ValueVisitor)
    }
}
//  <<<     //  SSSSS   EEEEEEE  RRRRRR   DDDDD    EEEEEEE >>>
// <<<     /// SS       EE       RR   RR  DD  DD   EE       >>>
// <<<    ///   SSSSS   EEEEE    RRRRRR   DD   DD  EEEEE    >>>
// <<<   ///        SS  EE       RR  RR   DD   DD  EE       >>>
//  <<< ///     SSSSS   EEEEEEE  RR   RR  DDDDDD   EEEEEEE >>>
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
