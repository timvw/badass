use crate::args::MaterializeArgs;
use crate::commands::compile::compile_model;
use crate::infra::flatten_errors;
use crate::model::{find_models, Model};
use crate::settings::Settings;
use anyhow::Result;
use camino::Utf8PathBuf;
use postgres::{Client, NoTls};
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

    log::debug!("Connecting to {:#?}", &settings.queryengine.params);
    let mut client = Client::connect(&settings.queryengine.params, NoTls)?;

    let materialized_files = compiled_files
        .into_iter()
        .map(|(model, compiled)| {
            log::trace!("Materializing {compiled:?}");
            let table_name = &model.name;
            materialize_table(table_name, &compiled, &mut client)
        })
        .collect::<Vec<_>>();
    flatten_errors(materialized_files).map(|_| ())
}

fn materialize_table(
    table_name: &str,
    compiled_sql_file: &Utf8PathBuf,
    client: &mut Client,
) -> Result<()> {
    let compiled_sql = fs::read_to_string(compiled_sql_file)?;
    let materialized_sql = format!("CREATE TABLE {} AS {}", table_name, compiled_sql);
    log::debug!("Run SQL: {materialized_sql:?}");
    client.execute(&materialized_sql, &[])?;
    Ok(())
}
