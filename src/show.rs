use crate::args::{BadassArgs, ShowArgs};
use crate::settings::Settings;

pub fn do_show(settings: &Settings, show_args: &ShowArgs) -> anyhow::Result<()> {
    log::info!("Need to show {show_args:?}");
    Ok(())
}