    
    @ReactMethod
    fun {{ func.name()|fn_name|unquote }}({%- call kt::arg_list_decl(func) -%}promise: Promise) {
        executor.execute {
            try {
{%- for arg in func.arguments() -%}
    {%- match arg.type_() %}
    {%- when Type::Enum(inner) %}
        {%- let e = ci.get_enum_definition(inner).unwrap() %}
        {%- if e.is_flat() %}
                val {{arg.name()|var_name|unquote|temporary}} = as{{arg.type_()|type_name}}({{ arg.name()|var_name|unquote }})
        {%- else %}
                val {{arg.name()|var_name|unquote|temporary}} = as{{arg.type_()|type_name}}({{ arg.name()|var_name|unquote }}) ?: run { throw SdkException.Generic(errMissingMandatoryField("{{arg.name()|var_name|unquote}}", "{{ arg.type_()|type_name }}")) }
        {%- endif %}
    {%- when Type::Optional(_) %}
                val {{arg.name()|var_name|unquote|temporary}} = {{arg.name()|var_name|unquote}}{{ arg.type_()|rn_convert_type(ci) -}}
    {%- when Type::Record(_) %}
                val {{arg.type_()|type_name|var_name|unquote}} = as{{arg.type_()|type_name}}({{ arg.name()|var_name|unquote }}) ?: run { throw SdkException.Generic(errMissingMandatoryField("{{arg.name()|var_name|unquote}}", "{{ arg.type_()|type_name }}")) }
    {%- else %}
    {%- endmatch %}
{%- endfor %}
{%- match func.return_type() -%}
{%- when Some with (return_type) %}
                val res = {{ obj_interface }}{{ func.name()|fn_name|unquote }}({%- call kt::arg_list(func) -%})
{%- if func.name() == "default_config" %}
                val workingDir = File(reactApplicationContext.filesDir.toString() + "/breezSdk")

                res.workingDir = workingDir.absolutePath
{%- endif -%}               
    {%- match return_type %}
    {%- when Type::Optional(inner) %}
        {%- let unboxed = inner.as_ref() %}
                promise.resolve(res?.let { {% call kt::return_value(unboxed) %} })
    {%- else %}
                promise.resolve({% call kt::return_value(return_type) %})
    {%- endmatch %}
{%- when None %}
                {{ obj_interface }}{{ func.name()|fn_name|unquote }}({%- call kt::arg_list(func) -%})
{%- if func.name() == "disconnect" %}
                breezServices = null
{%- endif %}               
                promise.resolve(readableMapOf("status" to "ok"))
{%- endmatch %}
            } catch (e: Exception) {
                promise.reject(e.javaClass.simpleName.replace("Exception", "Error"), e.message, e)
            }
        }
    }