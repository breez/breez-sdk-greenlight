
{% macro arg_list(func) %}
    {%- for arg in func.arguments() -%}
        {{ arg.name()|var_name -}}
        {%- if !loop.last %}, {% endif -%}
    {%- endfor %}
{%- endmacro %}

{%- macro field_list(rec) %}
    {%- for f in rec.fields() %}
        {{ f.name()|var_name|unquote }},
    {%- endfor %}
{%- endmacro -%}

{%- macro field_list_decl(rec) %}
    {%- for f in rec.fields() %}
    {%- match f.type_() %}
    {%- when Type::Optional(inner) %}
    {%- let unboxed = inner.as_ref() %}
    {{ f.name()|var_name }}?: {{ unboxed|type_name }}
    {%- else %}
    {{ f.name()|var_name }}: {{ f.type_()|type_name }}
    {%- endmatch %}
    {%- endfor %}
{%- endmacro -%}

{% macro arg_list_decl(func) %}
    {%- for arg in func.arguments() -%}
        {{ arg.name()|var_name }}: {{ arg.type_()|absolute_type_name }}{{- arg.type_()|default_value -}}
        {%- if !loop.last %}, {% endif -%}
    {%- endfor %}
{%- endmacro %}
