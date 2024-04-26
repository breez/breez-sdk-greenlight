{%- let rec = ci.get_record_definition(name).unwrap() %}

export type {{ type_name }} = {
    {%- call ts::field_list_decl(rec) %}
}
