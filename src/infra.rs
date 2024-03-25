use anyhow::{anyhow, Error, Result};
use itertools::Itertools;
use std::fmt::Debug;

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
    fn test_flatten_errors() {
        assert!(flatten_errors(vec![Ok(1), Err(Error::msg("two")), Ok(3)]).is_err());
    }

    #[test]
    fn test_flatten_errors_all_good() {
        assert!(flatten_errors(vec![Ok(1), Ok(2)]).is_ok_and(|items| items.len() == 2));
    }
}
