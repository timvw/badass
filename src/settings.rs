use config::{Config, ConfigError, Environment};
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Models {
    pub location: PathBuf,
}

impl Default for Models {
    fn default() -> Self {
        Models {
            location: PathBuf::from("./models"),
        }
    }
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Output {
    pub compiled: PathBuf,
    pub materialized: PathBuf,
}

impl Default for Output {
    fn default() -> Self {
        Output {
            compiled: PathBuf::from("./target/compiled"),
            materialized: PathBuf::from("./target/materialized"),
        }
    }
}

#[derive(Debug, Deserialize, Default)]
#[allow(unused)]
pub struct Settings {
    pub models: Models,
    pub output: Output,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let s = Config::builder()
            .set_default("models.location", "./models")
            .unwrap()
            .set_default("output.compiled", "./target/compiled")
            .unwrap()
            .set_default("output.materialized", "./target/materialized")
            .unwrap()
            .add_source(Environment::with_prefix("BADASS").separator("_"))
            .build()?;

        // You can deserialize (and thus freeze) the entire configuration as
        s.try_deserialize()
    }
}
