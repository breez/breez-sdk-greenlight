    
    @objc({%- call swift::extern_arg_list(func) -%})
    func {{ func.name()|fn_name|unquote }}(_ {% call swift::arg_list_decl(func) -%}resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) -> Void {
        do {
{%- for arg in func.arguments() -%}
    {%- match arg.type_() %}
    {%- when Type::Enum(inner) %}
        {%- let e = ci.get_enum_definition(inner).unwrap() %}
        {%- if e.is_flat() %}
            let {{arg.name()|var_name|unquote|temporary}} = try BreezSDKMapper.as{{arg.type_()|type_name}}({{ arg.type_()|type_name|var_name|unquote }}: {{ arg.name()|var_name|unquote }})
        {%- else %}
            let {{arg.name()|var_name|unquote|temporary}} = try BreezSDKMapper.as{{arg.type_()|type_name}}({{ arg.type_()|type_name|var_name|unquote }}: {{ arg.name()|var_name|unquote }})
        {%- endif %}
    {%- when Type::Optional(_) %}
            let {{arg.name()|var_name|unquote|temporary}} = {{ arg.type_()|rn_convert_type(arg.name()|var_name|unquote) -}}
    {%- when Type::Record(_) %}
            let {{arg.type_()|type_name|var_name|unquote}} = try BreezSDKMapper.as{{arg.type_()|type_name}}({{ arg.type_()|type_name|var_name|unquote }}: {{ arg.name()|var_name|unquote }})
    {%- else %}
    {%- endmatch %}
{%- endfor %}
{%- match func.return_type() -%}
{%- when Some with (return_type) %}
            var res = {%- call swift::throws_decl(func) -%}{{ obj_interface }}{{ func.name()|fn_name|unquote }}({%- call swift::arg_list(func) -%})
{%- if func.name() == "default_config" %}
            res.workingDir = RNBreezSDK.breezSdkDirectory.path
{%- endif -%}
    {%- match return_type %}
    {%- when Type::Optional(inner) %}
        {%- let unboxed = inner.as_ref() %}
            if res != nil {
                resolve({{ unboxed|rn_return_type(unboxed|type_name|var_name|unquote, true) }})
            } else {
                resolve(nil)
            }
    {%- else %}
            resolve({{ return_type|rn_return_type(return_type|type_name|var_name|unquote, false) }})
    {%- endmatch %}
{%- when None %}
            try {{ obj_interface }}{{ func.name()|fn_name|unquote }}({%- call swift::arg_list(func) -%})
{%- if func.name() == "disconnect" %}
            breezServices = nil
{%- endif %}
            resolve(["status": "ok"])     
{%- endmatch %}
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }