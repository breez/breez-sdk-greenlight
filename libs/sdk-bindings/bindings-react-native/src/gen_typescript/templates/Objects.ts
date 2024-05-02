{%- for type_ in ci.iter_types() %}
{%- let type_name = type_|type_name %}
{%- match type_ %}
{%- when Type::Object ( name ) %}
{% let obj = ci.get_object_definition(name).unwrap() %}
{%- for func in obj.methods() -%}
{%- include "TopLevelFunctionTemplate.ts" %}
{% endfor %}
{%- else -%}
{%- endmatch -%}    
{%- endfor %}

