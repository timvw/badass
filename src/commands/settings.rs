use crate::settings::Settings;

pub fn do_show() -> anyhow::Result<()> {
    match Settings::new() {
        Ok(settings) => {
            println!("The settings are: \n\n{settings:#?}")
        }
        Err(e) => println!("Failed to parse settings because {e}"),
    }
    Ok(())
}
