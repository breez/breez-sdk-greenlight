{%- let e = ci.get_enum_definition(name).unwrap() %}
{%- if e.is_flat() %}

static func as{{ type_name }}({{ type_name|var_name|unquote }}: String) throws -> {{ type_name }} {
    switch({{ type_name|var_name|unquote }}) {
    {%- for variant in e.variants() %}

    case "{{variant.name()|var_name|unquote}}":         
        return {{ type_name }}.{{variant.name()|var_name|unquote}}

    {%- endfor %}
    
    default: throw SdkError.Generic(message: "Invalid variant \({{ type_name|var_name|unquote }}) for enum {{ type_name }}")
    }
}

static func valueOf({{ type_name|var_name|unquote }}: {{ type_name }}) -> String {
    switch({{ type_name|var_name|unquote }}) {
    {%- for variant in e.variants() %}

    case .{{variant.name()|var_name|unquote}}:         
        return "{{variant.name()|var_name|unquote}}"

    {%- endfor %}
            
    }
}

static func arrayOf({{ type_name|var_name|unquote|list_arg }}: [{{ type_name }}]) -> [String] {
    return {{ type_name|var_name|unquote|list_arg }}.map { (v) -> String in return valueOf({{ type_name|var_name|unquote }}: v) }
}

{%- else %}

static func as{{ type_name }}({{ type_name|var_name|unquote }}: [String: Any?]) throws -> {{ type_name }} {
    let type = {{ type_name|var_name|unquote }}["type"] as! String

    {%- for variant in e.variants() %}
        if (type == "{{ variant.name()|var_name|unquote }}") {
            {%- if variant.has_fields() %}
                {%- for field in variant.fields() %}
                {%- match field.type_() %}         
                {%- when Type::Optional(_) %}
                    {% if field.type_()|inline_optional_field(ci) -%}
                    let _{{field.name()|var_name|unquote}} = {{ type_name|var_name|unquote }}["{{field.name()|var_name|unquote}}"] as? {{field.type_()|rn_type_name(ci, true)}}
                    {% else -%}
                    var _{{field.name()|var_name|unquote}}: {{field.type_()|type_name}}
                    if let {{field.name()|var_name|unquote|temporary}} = {{ type_name|var_name|unquote }}["{{field.name()|var_name|unquote}}"] as? {{field.type_()|rn_type_name(ci, true)}} {
                        _{{field.name()|var_name|unquote}} = {{field.type_()|render_from_map(ci, field.name()|var_name|unquote|temporary)}}
                    }
                    {% endif -%}
                {%- else %}
                {% if field.type_()|inline_optional_field(ci) -%}
                guard let _{{field.name()|var_name|unquote}} = {{ type_name|var_name|unquote }}["{{field.name()|var_name|unquote}}"] as? {{field.type_()|rn_type_name(ci, true)}} else {
                    throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "{{field.name()|var_name|unquote}}", typeName: "{{ type_name }}"))
                }
                {%- else -%}
                guard let {{field.name()|var_name|unquote|temporary}} = {{ type_name|var_name|unquote }}["{{field.name()|var_name|unquote}}"] as? {{field.type_()|rn_type_name(ci, true)}} else {
                    throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "{{field.name()|var_name|unquote}}", typeName: "{{ type_name }}"))
                }
            let _{{field.name()|var_name|unquote}} = {{field.type_()|render_from_map(ci, field.name()|var_name|unquote|temporary)}}
            {% endif -%}        
            {% endmatch %}    
            {%- endfor %}            
            return {{ type_name }}.{{ variant.name()|var_name|unquote }}({%- call swift::field_list(variant, "_") -%})                         
            {%- else %}
            return {{ type_name }}.{{ variant.name()|var_name|unquote }}          
            {%- endif %}       
        }        
    {%- endfor %}    

    throw SdkError.Generic(message: "Unexpected type \(type) for enum {{ type_name }}")
}

static func dictionaryOf({{ type_name|var_name|unquote }}: {{ type_name }}) -> [String: Any?] {    
    switch ({{ type_name|var_name|unquote }}) {
    {%- for variant in e.variants() %}
    {% if variant.has_fields() %}
    case let .{{ variant.name()|var_name|unquote }}(
        {% for f in variant.fields() %}{{f.name()|var_name|unquote}}{%- if !loop.last %}, {% endif -%}{%- endfor %}
    ):
    {% else %}
    case .{{ variant.name()|var_name|unquote }}:  
    {% endif -%}
        return [
            "type": "{{ variant.name()|var_name|unquote }}",
            {%- for f in variant.fields() %}
            "{{ f.name()|var_name|unquote }}": {{ f.type_()|render_to_map(ci,"",f.name()|var_name|unquote, false) }},             
            {%- endfor %}
        ] 
    {%- endfor %}   
    }    
}

static func arrayOf({{ type_name|var_name|unquote|list_arg }}: [{{ type_name }}]) -> [Any] {
    return {{ type_name|var_name|unquote|list_arg }}.map { (v) -> [String: Any?] in return dictionaryOf({{ type_name|var_name|unquote }}: v) }
}

{%- endif %}

static func as{{ type_name }}List(arr: [Any]) throws -> [{{ type_name }}] {
    var list = [{{ type_name }}]()
    for value in arr {
{%- if e.is_flat() %}
        if let val = value as? String {
{%- else %}
        if let val = value as? [String: Any?] {
 {%- endif %}
            var {{ type_name|var_name|unquote }} = try as{{ type_name }}({{ type_name|var_name|unquote }}: val)
            list.append({{ type_name|var_name|unquote }})
        } else { 
            throw SdkError.Generic(message: errUnexpectedType(typeName: "{{ type_name }}"))
        }
    }
    return list
}
