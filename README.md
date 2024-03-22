# BADASS
 
Badass is a CLI tool inspired by [DBT](https://github.com/dbt-labs/dbt-core) and [Airflow](https://airflow.apache.org/). 
Mainly a playground for me to become more familiar with [Rust](https://www.rust-lang.org/).

### Features

#### Compile SQL templates

```bash
badass compile
```

We leverage [minijinja](https://docs.rs/minijinja/latest/minijinja/) to generate SQL files.

```sql
{% set payment_methods = ["bank_transfer", "credit_card", "gift_card"] %}
select
    order_id,
    {%- for payment_method in payment_methods %}
    sum(case when payment_method = '{{payment_method}}' then amount end) as {{payment_method}}_amount,
    {%- endfor %}
    sum(amount) as total_amount
from app_data.payments
group by 1
```

Is compiled into the following:

```sql
select
    order_id,
    sum(case when payment_method = 'bank_transfer' then amount end) as bank_transfer_amount,
    sum(case when payment_method = 'credit_card' then amount end) as credit_card_amount,
    sum(case when payment_method = 'gift_card' then amount end) as gift_card_amount,
    sum(amount) as total_amount
from app_data.payments
group by 1
```

### View (compiled) SQL template query results

```bash
badass show
```

```bash

```

#### Materialize SQL templates

```bash
badass materialize
```

Use the (compiled) SQL templates to build database artifacts (tables, views, ...)

Currently we only render CTAS, eg:

```sql
SELECT * FROM foo
```

Becomes

```sql
CREATE TABLE xxx AS SELECT * FROM foo
```

