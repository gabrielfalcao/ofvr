#![allow(unused)]
use std::collections::BTreeMap;
use clap::builder::PossibleValue;
use clap::ValueEnum;
use enum_to_string::EnumToString;

use iocore::Path;
use serde::{Deserialize, Serialize};

use crate::{Error, Result};

#[derive(
    Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Deserialize, Serialize, Debug, Default
)]
pub struct Config {
    pub state_base_path: Option<Path>,
    pub file_uuid_algorithm: FileUuidAlgorithm,
}

impl Config {
    pub fn load(path: &Path) -> Result<Config> {
        Ok(Config::from_toml_string(&path.read()?)?)
    }

    pub fn from_toml_string(config: &str) -> Result<Config> {
        Ok(toml::from_str::<Config>(config)?)
    }
}


#[derive(Clone, Copy, EnumToString, Deserialize, Serialize)]
pub enum FileUuidAlgorithm {
    Crc32,
    Md5,
    Sha1,
    Sha256,
    Sha512,
}
impl Default for FileUuidAlgorithm {
    fn default() -> Self {
        FileUuidAlgorithm::Crc32
    }
}
impl ValueEnum for FileUuidAlgorithm {
    fn value_variants<'a>() -> &'a [FileUuidAlgorithm] {
        FileUuidAlgorithm::variants()
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(
            PossibleValue::new(self.to_string())
                .alias(self.to_string().to_lowercase())
                .alias(self.to_string().to_uppercase()),
        )
    }

    fn from_str(val: &str, ignore_case: bool) -> std::result::Result<FileUuidAlgorithm, String> {
        let val = if ignore_case { val.to_lowercase() } else { val.to_string() };
        let val = val.trim();
        for (vmethod, smet) in FileUuidAlgorithm::variants()
            .iter()
            .map(|m| (m, if ignore_case { m.to_string().to_lowercase() } else { m.to_string() }))
        {
            if val == smet {
                return Ok(vmethod.clone());
            }
        }
        return Err(val.to_string());
    }
}
