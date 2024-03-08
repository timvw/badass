use crate::settings::Settings;
use anyhow::{Context, Result};
use minijinja::{context, Environment};
use std::fs;
use std::path::PathBuf;

pub fn do_compile(settings: &Settings) -> Result<()> {
    let template_files = crate::infra::list_template_files(settings).with_context(|| {
        format!(
            "Failed to list files in {}",
            &settings.models.location.display()
        )
    })?;
    fs::create_dir_all(&settings.output.location).with_context(|| {
        format!(
            "Failed to ensure directory {} exists",
            &settings.output.location.display()
        )
    })?;
    let results = template_files
        .flatten()
        .map(|source| compile_file(&source, &settings.output.location))
        .collect::<Vec<_>>();
    crate::infra::flatten_errors(results).map(|_| ())
}

fn compile_file(source: &PathBuf, target_dir: &PathBuf) -> Result<()> {
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
    Ok(())
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
