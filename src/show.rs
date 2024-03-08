use crate::args::{BadassArgs, ShowArgs};
use crate::settings::Settings;
use postgres::{Client, NoTls};

pub fn do_show(settings: &Settings, show_args: &ShowArgs) -> anyhow::Result<()> {
    log::info!("Need to show {show_args:?}");

    // find the model sql...
    let compiled_model_sql = r"
    SELECT 1 AS number
    UNION ALL
    SELECT 3 AS number
    ";

    let mut client = Client::connect("host=localhost user=tim", NoTls)?;

    for row in client.query(compiled_model_sql, &[])? {
        for column in row.columns() {
            println!("column: {}", column.name());
        }
        let number: i32 = row.get(0);
        println!("found row: {}", number,);
    }

    Ok(())
}