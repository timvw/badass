use crate::settings::Settings;
use anyhow::{anyhow, Context};
use glob::Paths;
use itertools::Itertools;

pub fn list_template_files(settings: &Settings) -> anyhow::Result<Paths> {
    glob::glob(&format!("{}/*.sql", settings.models.location.display()))
        .with_context(|| "failed to find models")
}

pub fn flatten_errors<T>(results: Vec<anyhow::Result<T>>) -> anyhow::Result<Vec<T>> {
    let mut oks: Vec<T> = Vec::new();
    let mut errs: Vec<anyhow::Error> = Vec::new();

    results.into_iter().for_each(|item| match item {
        Ok(v) => oks.push(v),
        Err(e) => errs.push(e),
    });

    if errs.is_empty() {
        Ok(oks)
    } else {
        Err(anyhow!(
            "{}",
            errs.iter().map(|e| format!("{:#}", e)).format("\n")
        ))
    }
}
