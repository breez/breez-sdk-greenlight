{%- for type_ in ci.iter_types() -%}
{%- let type_name = type_|type_name -%}
{%- match type_ -%}
{%- when Type::Record ( name ) %}
    {%- include "RecordTemplate.kt" %}
{%- when Type::Enum ( name ) %}
    {%- include "EnumTemplate.kt" %}
{%- when Type::Object ( name ) %}
    {% let obj = ci.get_object_definition(name).unwrap() -%}
    {%- for func in obj.methods() -%}
        {%- match func.return_type() -%}
        {%- when Some with (return_type) -%}
            {%- match return_type -%} 
            {%- when Type::Optional(inner) -%}
                {%- let unboxed = inner.as_ref() -%}
                {%- match unboxed -%}
                {%- when Type::Sequence(inner_type) -%}
                {{- self.add_sequence_type(inner_type|type_name) -}}
                {%- else -%}
                {%- endmatch -%}
            {%- when Type::Sequence(inner_type) -%}
            {{- self.add_sequence_type(inner_type|type_name) -}}
            {%- else -%}
            {%- endmatch -%}
        {%- else -%}
        {%- endmatch -%}    
    {% endfor -%}
{%- else -%}
{%- endmatch -%}
{%- endfor -%}