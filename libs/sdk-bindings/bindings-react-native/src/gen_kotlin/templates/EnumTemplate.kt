{%- let e = ci.get_enum_definition(name).unwrap() %}
{%- if e.is_flat() %}

fun as{{ type_name }}(type: String): {{ type_name }} {
    return {{ type_name }}.valueOf(camelToUpperSnakeCase(type))
}

{%- else %}

fun as{{ type_name }}({{ type_name|var_name|unquote }}: ReadableMap): {{ type_name }}? {
    val type = {{ type_name|var_name|unquote }}.getString("type")

    {% for variant in e.variants() -%}
        if (type == "{{ variant.name()|var_name|unquote }}") {
            {% if variant.has_fields() -%}
            return {{ type_name }}.{{ variant.name() }}( {{ variant.fields()[0].type_()|render_from_map(ci, type_name|var_name|unquote, variant.fields()[0].name()|var_name|unquote, false) }})                         
            {%- else %}
            return {{ type_name }}.{{ variant.name() }}          
            {%- endif %}       
        }        
    {% endfor -%}    

    return null
}

fun readableMapOf({{ type_name|var_name|unquote }}: {{ type_name }}): ReadableMap? {    
    val map = Arguments.createMap()
    when ({{ type_name|var_name|unquote }}) {
    {% for variant in e.variants() -%}        
    is {{ type_name }}.{{ variant.name() }} -> {
        pushToMap(map, "type", "{{ variant.name()|var_name|unquote }}")
        {% for f in variant.fields() -%}
        pushToMap(map, "{{ f.name()|var_name|unquote }}", {{ f.type_()|render_to_map(ci,type_name|var_name|unquote,f.name()|var_name|unquote, false) }})                    
        {% endfor -%}
    }
    {% endfor %}
    }
    return map     
}

{%- endif %}

fun as{{ type_name }}List(arr: ReadableArray): List<{{ type_name }}> {
    val list = ArrayList<{{ type_name }}>()
    for (value in arr.toArrayList()) {
        when (value) {
{%- if e.is_flat() %}
            is String -> list.add(as{{ type_name }}(value)!!)            
{%- else %}
            is ReadableMap -> list.add(as{{ type_name }}(value)!!)            
{%- endif %}
            else -> throw SdkException.Generic(errUnexpectedType("${value::class.java.name}"))
        }
    }
    return list
}