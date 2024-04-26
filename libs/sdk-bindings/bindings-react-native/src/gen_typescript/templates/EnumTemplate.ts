{%- let e = ci.get_enum_definition(name).unwrap() %}
{%- if e.is_flat() %}

export enum {{ type_name }} {
    {% for variant in e.variants() -%}
    {{ variant.name()|enum_variant }} = "{{ variant.name()|var_name }}"{% if !loop.last %},
    {% endif %}
    {%- endfor %}
}

{%- else %}

export enum {{ type_name }}Variant {
    {% for variant in e.variants() -%}
    {{ variant.name()|enum_variant }} = "{{ variant.name()|var_name }}"{% if !loop.last %},
    {% endif %}
    {%- endfor %}
}

export type {{ type_name }} = {% for variant in e.variants() -%}{
    type: {{ type_name }}Variant.{{ variant.name()|enum_variant }}{% if variant.has_fields() %}, 
    {%- call ts::field_list_decl(variant) -%}{% endif %}
}{% if !loop.last %} | {% endif %}
{%- endfor %}

{%- endif %}