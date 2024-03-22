use anyhow::{anyhow, Context, Error, Result};
use camino::Utf8PathBuf;
use itertools::Itertools;
use serde::Deserialize;
use std::fmt::Debug;

#[derive(Debug, Deserialize, Default)]
#[allow(unused)]
pub struct Model {
    pub file: Utf8PathBuf,
}

impl Model {
    pub fn name(&self) -> &str {
        self.file.file_stem().unwrap()
    }
}

pub fn list_models(dir: &Utf8PathBuf) -> Result<Vec<Model>> {
    let template_files = list_template_files(dir)?;
    let models = template_files
        .into_iter()
        .map(|f| Model { file: f })
        .collect();
    Ok(models)
}

pub fn list_template_files(dir: &Utf8PathBuf) -> Result<Vec<Utf8PathBuf>> {
    let pattern = format!("{}/*.sql", dir);
    let paths = glob::glob(&pattern)
        .with_context(|| format!("failed to find template files matching {}", &pattern))?;
    let utf8_paths = paths
        .into_iter()
        .flatten()
        .flat_map(Utf8PathBuf::from_path_buf)
        .collect();
    Ok(utf8_paths)
}

pub fn flatten_errors<T: Debug>(results: Vec<Result<T>>) -> Result<Vec<T>> {
    let mut oks: Vec<T> = Vec::new();
    let mut errs: Vec<Error> = Vec::new();

    results.into_iter().for_each(|item| match item {
        Ok(v) => oks.push(v),
        Err(e) => errs.push(e),
    });

    log::trace!("ok items: {oks:?}");
    log::trace!("err items: {errs:?}");

    if errs.is_empty() {
        Ok(oks)
    } else {
        Err(anyhow!(
            "{}",
            errs.iter().map(|e| format!("{:#}", e)).format("\n")
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_template_files() {
        let files = list_template_files(&Utf8PathBuf::from("./demo/models")).unwrap();
        assert!(files.len() > 1);
    }

    #[test]
    fn test_flatten_errors() {
        assert!(flatten_errors(vec![Ok(1), Err(Error::msg("two")), Ok(3)]).is_err());
    }

    #[test]
    fn test_flatten_errors_all_good() {
        assert!(flatten_errors(vec![Ok(1), Ok(2)]).is_ok_and(|items| items.len() == 2));
    }

    #[test]
    fn test_list_models() {
        let models = list_models(&Utf8PathBuf::from("./demo/models")).unwrap();
        assert!(models.iter().any(|m| m.name() == "demo"));
        assert!(models.iter().any(|m| m.name() == "interactions"));
    }
}
