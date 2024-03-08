use crate::infra;
use crate::settings::Settings;
use anyhow::{Context, Result};
use minijinja::{context, Environment};
use std::fs;
use std::path::PathBuf;

pub fn do_compile(settings: &Settings) -> Result<()> {
    let source_dir = &settings.models.location;
    let target_dir = &settings.output.location;
    let _ = compile_files(&source_dir, &target_dir)?;
    Ok(())
}

pub fn compile_files(
    source_dir: &PathBuf,
    target_dir: &PathBuf,
) -> Result<Vec<(PathBuf, PathBuf)>> {
    let template_files = infra::list_template_files(source_dir)?;
    fs::create_dir_all(&target_dir).with_context(|| {
        format!(
            "Failed to ensure directory {} exists",
            &target_dir.display()
        )
    })?;
    let results = template_files
        .flatten()
        .map(|source| match compile_file(&source, &target_dir) {
            Ok(compiled_file) => Ok((source, compiled_file)),
            Err(e) => Err(e),
        })
        .collect::<Vec<_>>();
    infra::flatten_errors(results)
}

fn compile_file(source: &PathBuf, target_dir: &PathBuf) -> Result<PathBuf> {
    let target = target_dir.join(
        source
            .file_name()
            .with_context(|| format!("Failed to build build target path for {:?}", &source))?,
    );
    let file_content =
        fs::read_to_string(&source).with_context(|| format!("Failed to read {:?}", &source))?;
    let compiled_content =
        compile(&file_content).with_context(|| format!("Failed to compile {:?}", &source))?;
    fs::write(&target, compiled_content)
        .with_context(|| format!("Failed to write compiled template to {:?}", &target))?;
    Ok(target)
}

fn mref(name: String) -> core::result::Result<String, minijinja::Error> {
    Ok(name)
}

fn compile(template: &str) -> Result<String> {
    let mut env = Environment::new();
    env.add_function("ref", mref);
    env.render_str(template, context! {})
        .with_context(|| "Failed to render template")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compile_simple() {
        let compiled = compile("Hello world").unwrap();
        assert_eq!(compiled, "Hello world");
    }

    #[test]
    fn test_compile_foreach() {
        let template = r#"
{% set payment_methods = ["bank_transfer", "credit_card", "gift_card"] %}
select
    order_id,
    {%- for payment_method in payment_methods %}
    sum(case when payment_method = '{{payment_method}}' then amount end) as {{payment_method}}_amount,
    {%- endfor %}
    sum(amount) as total_amount
from app_data.payments
group by 1
        "#;

        let compiled = compile(template).unwrap();

        let expected = r#"
select
    order_id,
    sum(case when payment_method = 'bank_transfer' then amount end) as bank_transfer_amount,
    sum(case when payment_method = 'credit_card' then amount end) as credit_card_amount,
    sum(case when payment_method = 'gift_card' then amount end) as gift_card_amount,
    sum(amount) as total_amount
from app_data.payments
group by 1
        "#;

        assert_eq!(compiled.trim(), expected.trim());
    }
}
