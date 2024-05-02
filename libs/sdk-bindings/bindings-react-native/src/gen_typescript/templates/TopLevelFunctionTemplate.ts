{%- match func.return_type() -%}
{%- when Some with (return_type) %}
export const {{ func.name()|fn_name }} = async ({%- call ts::arg_list_decl(func) -%}): Promise<{{ return_type|return_type_name }}> => {
    const response = await BreezSDK.{{func.name()|fn_name}}({%- call ts::arg_list(func) -%})
    return response
}
{%- when None %}
export const {{ func.name()|fn_name }} = async ({%- call ts::arg_list_decl(func) -%}): Promise<void> => {
    await BreezSDK.{{ func.name()|fn_name }}({%- call ts::arg_list(func) -%})
}
{%- endmatch %}
