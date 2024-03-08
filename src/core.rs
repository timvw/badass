use crate::settings::Settings;
use anyhow::Context;
use glob::Paths;

pub fn list_template_files(settings: &Settings) -> anyhow::Result<Paths> {
    glob::glob(&format!("{}/*.sql", settings.models.location.display()))
        .with_context(|| "failed to find models")
}
