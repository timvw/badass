use crate::args::MaterializeArgs;
use crate::commands::compile::compile_model;
use crate::infra::{find_models, flatten_errors, Model};
use crate::settings::Settings;
use anyhow::{Context, Result};
use camino::Utf8PathBuf;
use std::fs;

pub fn do_materialize(settings: &Settings, materialize_args: &MaterializeArgs) -> Result<()> {
    let models = find_models(settings, &materialize_args.model)?;
    let compiled_models: Vec<(Result<Utf8PathBuf>, Model)> = models
        .into_iter()
        .map(|x| (compile_model(&x, settings), x))
        .collect();

    // only consider the models we've been able to compile... (perhaps we should bail out?)
    let compiled_files = compiled_models
        .into_iter()
        .filter_map(|(target_result, source)| match target_result {
            Ok(target) => Some((source, target)),
            Err(_) => None,
        })
        .collect::<Vec<_>>();

    let materialized_dir = &settings.output.materialized;

    fs::create_dir_all(materialized_dir)
        .with_context(|| format!("Failed to ensure directory {} exists", materialized_dir))?;

    let materialized_files = compiled_files
        .into_iter()
        .map(|(_, compiled)| {
            log::trace!("Materializing {compiled:?} in {materialized_dir:?}");
            let file_name = &compiled.file_name().unwrap();
            let materialized_file = materialized_dir.join(file_name);
            let table_name = &compiled.file_stem().unwrap();
            materialize_table(table_name, compiled.clone(), materialized_file)
        })
        .collect::<Vec<_>>();
    flatten_errors(materialized_files).map(|_| ())
}

fn materialize_table(
    table_name: &str,
    compiled_sql_file: Utf8PathBuf,
    materialized_sql_file: Utf8PathBuf,
) -> Result<(Utf8PathBuf, Utf8PathBuf)> {
    let compiled_sql = fs::read_to_string(&compiled_sql_file)?;
    let materialized_sql = format!("CREATE TABLE {} AS {}", table_name, compiled_sql);
    fs::write(&materialized_sql_file, materialized_sql)?;
    log::debug!("Materialized table {table_name} into {materialized_sql_file:?}");
    Ok((compiled_sql_file, materialized_sql_file))
}