{%- for type_ in ci.iter_types() %}
{%- let type_name = type_|type_name %}
{%- match type_ %}
{%- when Type::Record ( name ) %}
    {%- include "RecordTemplate.swift" %}
{%- when Type::Enum ( name ) %}
    {%- include "EnumTemplate.swift" %}
{%- else %}
{%- endmatch -%}    

{%- endfor %}