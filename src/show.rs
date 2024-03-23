use crate::args::ShowArgs;
use crate::compile::compile_model;
use crate::infra::list_models;
use crate::settings::Settings;
use anyhow::Context;
use chrono::{DateTime, Utc};
use postgres::{Client, Column, NoTls, Row};
use std::fs;
use std::time::SystemTime;
use tabled::builder::Builder;
use tabled::settings::Style;

pub fn do_show(settings: &Settings, show_args: &ShowArgs) -> anyhow::Result<()> {
    log::info!("Need to show {show_args:?}");

    let models = list_models(&settings.models.location)?;

    log::debug!("Found the following models: {models:#?}");

    let model = models
        .into_iter()
        .find(|model| model.name == show_args.model)
        .with_context(|| format!("Could not find model {}.", show_args.model))?;
    let compiled_model_file = compile_model(&model, settings)?;
    let compiled_model_sql = fs::read_to_string(compiled_model_file)?;

    let mut client = Client::connect(&settings.query_engine.params, NoTls)?;

    let result_rows = client.query(&compiled_model_sql, &[])?;
    let rows: Vec<Vec<String>> = result_rows.iter().map(get_display_values).collect();

    let mut builder = Builder::default();
    for row in rows {
        builder.push_record(row);
    }

    let mut table = builder.build();
    table.with(Style::ascii_rounded());
    print!("{}", table);

    Ok(())
}

fn get_display_values(row: &Row) -> Vec<String> {
    row.columns()
        .iter()
        .map(|column| get_display_value(row, column))
        .collect()
}

fn get_display_value(row: &Row, column: &Column) -> String {
    match column.type_().name() {
        "text" | "varchar" | "citext" | "name" => row.get::<&str, String>(column.name()),
        "bool" => row
            .try_get::<&str, bool>(column.name())
            .map_or(String::from(""), |x| x.to_string()),
        "char" => row
            .try_get::<&str, i8>(column.name())
            .map_or(String::from(""), |x| x.to_string()),
        "smallint" | "smallserial" => row
            .try_get::<&str, i16>(column.name())
            .map_or(String::from(""), |x| x.to_string()),
        "int" | "serial" | "int4" => row
            .try_get::<&str, i32>(column.name())
            .map_or(String::from(""), |x| x.to_string()),
        "oid" => row
            .try_get::<&str, u32>(column.name())
            .map_or(String::from(""), |x| x.to_string()),
        "bigint" | "bigserial" => row
            .try_get::<&str, i64>(column.name())
            .map_or(String::from(""), |x| x.to_string()),
        "real" => row
            .try_get::<&str, f32>(column.name())
            .map_or(String::from(""), |x| x.to_string()),
        "double precision" => row
            .try_get::<&str, f64>(column.name())
            .map_or(String::from(""), |x| x.to_string()),
        "timestamp" | "timestamptz" => {
            row.try_get::<&str, SystemTime>(column.name())
                .map_or(String::from(""), |x| {
                    let datetime: DateTime<Utc> = x.into();
                    datetime.to_rfc3339()
                })
        }
        _ => row
            .try_get::<&str, String>(column.name())
            .unwrap_or(format!("Unsupported {}", column.type_().name())),
    }
}
