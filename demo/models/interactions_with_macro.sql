{%- from "ref.sql" import ref %}
SELECT * FROM {{ ref('inbound_calls') }}
UNION ALL
SELECT * FROM {{ ref('modelled', 'outbound_calls') }}
UNION ALL
SELECT * FROM {{ ref('modelled', 'repair_appointment', '2') }}