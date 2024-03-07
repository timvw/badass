use std::fs;
use std::path::Path;
use minijinja::{Environment, context};
use anyhow::{Context, Result};
use glob::{glob, Paths};

pub fn compile_file(file: &Path) -> Result<String> {
    let file_content = fs::read_to_string(file).with_context(|| format!("Failed to read {:?}", file.display()))?;
    compile(&file_content)
}

pub fn compile(template: &str) -> Result<String> {
    let mut env = Environment::new();
    env.render_str(template, context!{ }).with_context(|| "Failed to render template")
}

pub fn list_models() -> Result<Paths> {
    glob("./demo/models/*.sql").with_context(||"failed to find models")
}

#[cfg(test)]
mod tests {
    use std::fs;
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

        assert_eq!(compiled, expected);
    }
}
