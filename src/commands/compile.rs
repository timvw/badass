use crate::infra;
use crate::infra::{flatten_errors, Model};
use crate::settings::Settings;
use anyhow::{Context, Result};
use camino::Utf8PathBuf;
use minijinja::{context, Environment};
use std::fs;

pub fn do_compile(settings: &Settings) -> Result<()> {
    let source_dir = &settings.models.location;
    let target_dir = &settings.output.compiled;
    let compilation_results = compile_files(source_dir, target_dir)?;
    let results = compilation_results
        .into_iter()
        .map(|(target_result, _)| match target_result {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        })
        .collect::<Vec<Result<_>>>();
    flatten_errors(results).map(|_| ())
}

pub fn compile_files(
    source_dir: &Utf8PathBuf,
    target_dir: &Utf8PathBuf,
) -> Result<Vec<(Result<Utf8PathBuf>, Utf8PathBuf)>> {
    let template_files = infra::list_template_files(source_dir)?;
    fs::create_dir_all(target_dir)
        .with_context(|| format!("Failed to ensure directory {} exists", &target_dir))?;
    let compilation_results = template_files
        .into_iter()
        .map(|source| (compile_file(&source, target_dir), source))
        .collect::<Vec<_>>();
    Ok(compilation_results)
}

fn compile_file(source: &Utf8PathBuf, target_dir: &Utf8PathBuf) -> Result<Utf8PathBuf> {
    log::debug!("Compiling {source:?} into {target_dir:?}");
    let target = target_dir.join(
        source
            .file_name()
            .with_context(|| format!("Failed to build build target path for {:?}", &source))?,
    );
    let file_content =
        fs::read_to_string(source).with_context(|| format!("Failed to read {:?}", &source))?;
    let compiled_content =
        compile(&file_content).with_context(|| format!("Failed to compile {:?}", &source))?;
    fs::write(&target, compiled_content)
        .with_context(|| format!("Failed to write compiled template to {:?}", &target))?;
    log::debug!("Compiled {source:?} into {target:?}");
    Ok(target)
}

pub fn compile_model(model: &Model, settings: &Settings) -> Result<Utf8PathBuf> {
    let source = &model.file;
    let target_dir = &settings.output.compiled;
    let result = compile_file(source, target_dir)?;
    Ok(result)
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
