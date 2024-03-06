{% macro ref(package, table, version) -%}
    {%- if package is defined -%}
        {{ package }}.
    {%- endif -%}
    {{ table }}
    {%- if version is defined -%}
        @{{ version }}
    {%- endif -%}
{% endmacro -%}