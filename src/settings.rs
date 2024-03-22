use camino::Utf8PathBuf;
use config::{Config, ConfigError, Environment};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Models {
    pub location: Utf8PathBuf,
}

impl Default for Models {
    fn default() -> Self {
        Models {
            location: Utf8PathBuf::from("./models"),
        }
    }
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Output {
    pub compiled: Utf8PathBuf,
    pub materialized: Utf8PathBuf,
}

impl Default for Output {
    fn default() -> Self {
        Output {
            compiled: Utf8PathBuf::from("./target/compiled"),
            materialized: Utf8PathBuf::from("./target/materialized"),
        }
    }
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct QueryEngine {
    pub params: String,
}

impl Default for QueryEngine {
    fn default() -> Self {
        QueryEngine {
            params: String::from("host=localhost user=tim"),
        }
    }
}

#[derive(Debug, Deserialize, Default)]
#[allow(unused)]
pub struct Settings {
    pub models: Models,
    pub output: Output,
    pub query_engine: QueryEngine,
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
