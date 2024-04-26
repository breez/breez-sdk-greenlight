
RCT_EXTERN_METHOD(
{%- if func.arguments().len() == 0 %}
    {{ func.name()|fn_name|unquote }}: (RCTPromiseResolveBlock)resolve{# -#}
{% else -%}
    {%- for arg in func.arguments() %}
        {%- if loop.first %}
    {{ func.name()|fn_name|unquote }}: ({{arg.type_()|extern_type_name(ci)}}){{ arg.name()|var_name|unquote }}
        {%- else %}
    {{ arg.name()|var_name|unquote }}: ({{arg.type_()|extern_type_name(ci)}}){{ arg.name()|var_name|unquote }}
        {%- endif -%}
    {% endfor %}
    resolve: (RCTPromiseResolveBlock)resolve
{%- endif %}
    reject: (RCTPromiseRejectBlock)reject
)