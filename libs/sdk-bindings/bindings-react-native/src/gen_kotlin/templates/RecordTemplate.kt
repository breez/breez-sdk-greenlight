{%- let rec = ci.get_record_definition(name).unwrap() %}
fun as{{ type_name }}({{ type_name|var_name|unquote }}: ReadableMap): {{ type_name }}? {
    if (!validateMandatoryFields({{ type_name|var_name|unquote }}, arrayOf(
        {%- for field in rec.fields() %}
            {%- match field.type_() %} 
            {%- when Type::Optional(_) %}
            {%- else %}
            "{{ field.name()|var_name |unquote }}",
            {%- endmatch %}
        {%- endfor %}
    ))) {
        return null
    }

    {%- for field in rec.fields() %}
    val {{field.name()|var_name|unquote}} = {{ field.type_()|render_from_map(ci, type_name|var_name|unquote, field.name()|var_name|unquote, false) }}    
    {%- endfor %}
    return {{ type_name }}({%- call kt::field_list(rec) -%})    
}

fun readableMapOf({{ type_name|var_name|unquote }}: {{ type_name }}): ReadableMap {
    return readableMapOf(
        {%- for field in rec.fields() %}
            {%- match field.type_() %} 
            {%- when Type::Optional(inner) %}
                {%- let unboxed = inner.as_ref() %}
                {%- match unboxed %}
                {%- when Type::Sequence(inner_type) %}
                {{- self.add_sequence_type(inner_type|type_name) }}
                {%- else %}
                {%- endmatch %}
            {%- when Type::Sequence(inner_type) %}
            {{- self.add_sequence_type(inner_type|type_name) }}
            {%- else %}
            {%- endmatch %}
            "{{ field.name()|var_name|unquote }}" to {{ field.type_()|render_to_map(ci,type_name|var_name|unquote, field.name()|var_name|unquote, false) }},
        {%- endfor %}       
    )
}

fun as{{ type_name }}List(arr: ReadableArray): List<{{ type_name }}> {
    val list = ArrayList<{{ type_name }}>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(as{{ type_name }}(value)!!)            
            else -> throw SdkException.Generic(errUnexpectedType("${value::class.java.name}"))
        }
    }
    return list
}