use crate::args::ShowArgs;
use crate::compile::compile_model;
use crate::infra::list_models;
use crate::settings::Settings;
use anyhow::Context;
use postgres::{Client, NoTls};
use std::fs;

pub fn do_show(settings: &Settings, show_args: &ShowArgs) -> anyhow::Result<()> {
    log::info!("Need to show {show_args:?}");

    let models = list_models(&settings.models.location)?;
    let model = models
        .into_iter()
        .find(|model| model.name() == show_args.model)
        .with_context(|| format!("Could not find model {}.", show_args.model))?;
    let compiled_model_file = compile_model(&model, &settings)?;
    let compiled_model_sql = fs::read_to_string(&compiled_model_file)?;

    let mut client = Client::connect("host=localhost user=tim", NoTls)?;

    for row in client.query(&compiled_model_sql, &[])? {
        for column in row.columns() {
            println!("column: {}", column.name());
        }
        let number: i32 = row.get(0);
        println!("found row: {}", number,);
    }

    Ok(())
}
