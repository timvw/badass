use anyhow::{anyhow, Context};
use glob::Paths;
use itertools::Itertools;
use std::path::PathBuf;

pub fn list_template_files(dir: &PathBuf) -> anyhow::Result<Paths> {
    let pattern = format!("{}/*.sql", dir.display());
    glob::glob(&pattern)
        .with_context(|| format!("failed to find template files matching {}", &pattern))
}

pub fn flatten_errors<T>(
    results: Vec<anyhow::Result<T>>,
) -> anyhow::Result<Vec<T>> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_template_files() {
        let files = list_template_files(&PathBuf::from("./demo/models")).unwrap();
        assert_eq!(files.count(), 2);
    }

    #[test]
    fn test_flatten_errors() {
        assert!(flatten_errors(vec![Ok(1), Err(anyhow::Error::msg("two")), Ok(3)]).is_err());
    }

    #[test]
    fn test_flatten_errors_all_good() {
        assert!(flatten_errors(vec![Ok(1), Ok(2)]).is_ok_and(|items| items.len() == 2));
    }
}
