use std::collections::HashSet;

use askama::Template;
use heck::{ToLowerCamelCase, ToShoutySnakeCase, ToUpperCamelCase};
use once_cell::sync::Lazy;
use uniffi_bindgen::backend::{CodeOracle, CodeType, TypeIdentifier};
use uniffi_bindgen::interface::*;

use crate::generator::RNConfig;

mod callback_interface;
mod compounds;
mod custom;
mod enum_;
mod error;
mod external;
mod miscellany;
mod object;
mod primitives;
mod record;

// Keywords to fix
static KEYWORDS: Lazy<HashSet<String>> = Lazy::new(|| {
    let list = vec!["Function", "Number", "Object", "Record", "String", "Symbol"];
    HashSet::from_iter(list.into_iter().map(|s| s.to_string()))
});

static IGNORED_FUNCTIONS: Lazy<HashSet<String>> = Lazy::new(|| {
    let list = vec!["connect", "set_log_stream"];
    HashSet::from_iter(list.into_iter().map(|s| s.to_string()))
});

#[derive(Template)]
#[template(syntax = "rn", escape = "none", path = "module.ts")]
#[allow(dead_code)]
pub struct ModuleGenerator<'a> {
    config: RNConfig,
    ci: &'a ComponentInterface,
}

impl<'a> ModuleGenerator<'a> {
    pub fn new(config: RNConfig, ci: &'a ComponentInterface) -> Self {
        Self { config, ci }
    }
}

fn fixup_keyword(name: String, append: String) -> String {
    if KEYWORDS.contains(&name) {
        format!("{name}{append}")
    } else {
        name
    }
}

#[derive(Clone)]
pub struct TypescriptCodeOracle;

impl TypescriptCodeOracle {
    // Map `Type` instances to a `Box<dyn CodeType>` for that type.
    //
    // There is a companion match in `templates/Types.ts` which performs a similar function for the
    // template code.
    //
    //   - When adding additional types here, make sure to also add a match arm to the `Types.ts` template.
    //   - To keep things managable, let's try to limit ourselves to these 2 mega-matches
    fn create_code_type(&self, type_: TypeIdentifier) -> Box<dyn CodeType> {
        match type_ {
            Type::UInt8 => Box::new(primitives::UInt8CodeType),
            Type::Int8 => Box::new(primitives::Int8CodeType),
            Type::UInt16 => Box::new(primitives::UInt16CodeType),
            Type::Int16 => Box::new(primitives::Int16CodeType),
            Type::UInt32 => Box::new(primitives::UInt32CodeType),
            Type::Int32 => Box::new(primitives::Int32CodeType),
            Type::UInt64 => Box::new(primitives::UInt64CodeType),
            Type::Int64 => Box::new(primitives::Int64CodeType),
            Type::Float32 => Box::new(primitives::Float32CodeType),
            Type::Float64 => Box::new(primitives::Float64CodeType),
            Type::Boolean => Box::new(primitives::BooleanCodeType),
            Type::String => Box::new(primitives::StringCodeType),

            Type::Timestamp => Box::new(miscellany::TimestampCodeType),
            Type::Duration => {
                unimplemented!("Duration is not implemented")
            }

            Type::Enum(id) => Box::new(enum_::EnumCodeType::new(id)),
            Type::Object(id) => Box::new(object::ObjectCodeType::new(id)),
            Type::Record(id) => Box::new(record::RecordCodeType::new(id)),
            Type::Error(id) => Box::new(error::ErrorCodeType::new(id)),
            Type::CallbackInterface(id) => {
                Box::new(callback_interface::CallbackInterfaceCodeType::new(id))
            }
            Type::Optional(inner) => Box::new(compounds::OptionalCodeType::new(*inner)),
            Type::Sequence(inner) => Box::new(compounds::SequenceCodeType::new(*inner)),
            Type::Map(key, value) => Box::new(compounds::MapCodeType::new(*key, *value)),
            Type::External { name, .. } => Box::new(external::ExternalCodeType::new(name)),
            Type::Custom { name, .. } => Box::new(custom::CustomCodeType::new(name)),

            Type::Unresolved { name } => {
                unreachable!("Type `{name}` must be resolved before calling create_code_type")
            }
        }
    }
}

impl CodeOracle for TypescriptCodeOracle {
    fn find(&self, type_: &TypeIdentifier) -> Box<dyn CodeType> {
        self.create_code_type(type_.clone())
    }

    /// Get the idiomatic Typescript rendering of a class name (for enums, records, errors, etc).
    fn class_name(&self, nm: &str) -> String {
        fixup_keyword(nm.to_string().to_upper_camel_case(), "Type".to_string())
    }

    /// Get the idiomatic Typescript rendering of a function name.
    fn fn_name(&self, nm: &str) -> String {
        fixup_keyword(nm.to_string().to_lower_camel_case(), "Fn".to_string())
    }

    /// Get the idiomatic Typescript rendering of a variable name.
    fn var_name(&self, nm: &str) -> String {
        fixup_keyword(nm.to_string().to_lower_camel_case(), "Var".to_string())
    }

    /// Get the idiomatic Typescript rendering of an individual enum variant.
    fn enum_variant_name(&self, nm: &str) -> String {
        fixup_keyword(nm.to_string().to_shouty_snake_case(), "Enum".to_string())
    }

    /// Get the idiomatic Typescript rendering of an exception name
    fn error_name(&self, nm: &str) -> String {
        self.class_name(nm)
    }

    fn ffi_type_label(&self, ffi_type: &FfiType) -> String {
        match ffi_type {
            FfiType::Int8
            | FfiType::UInt8
            | FfiType::Int16
            | FfiType::UInt16
            | FfiType::Int32
            | FfiType::UInt32
            | FfiType::Int64
            | FfiType::UInt64
            | FfiType::Float32
            | FfiType::Float64 => "number".to_string(),
            FfiType::RustArcPtr(name) => format!("{}SafeHandle", name),
            FfiType::RustBuffer(_) => "RustBuffer".to_string(),
            FfiType::ForeignBytes => "ForeignBytes".to_string(),
            FfiType::ForeignCallback => "ForeignCallback".to_string(),
        }
    }
}

pub mod filters {
    use uniffi_bindgen::backend::CodeType;

    use super::*;

    fn oracle() -> &'static TypescriptCodeOracle {
        &TypescriptCodeOracle
    }

    pub fn type_name(codetype: &impl CodeType) -> Result<String, askama::Error> {
        Ok(codetype.type_label(oracle()))
    }

    /// Get the idiomatic Typescript rendering of a function name.
    pub fn fn_name(nm: &str) -> Result<String, askama::Error> {
        Ok(oracle().fn_name(nm))
    }

    /// Get the idiomatic Typescript rendering of a variable name.
    pub fn var_name(nm: &str) -> Result<String, askama::Error> {
        Ok(oracle().var_name(nm))
    }

    /// Get the idiomatic Typescript rendering of an individual enum variant.
    pub fn enum_variant(nm: &str) -> Result<String, askama::Error> {
        Ok(oracle().enum_variant_name(nm))
    }

    pub fn absolute_type_name(t: &TypeIdentifier) -> Result<String, askama::Error> {
        let res: Result<String, askama::Error> = match t {
            Type::Optional(inner) => {
                let unboxed = inner.as_ref();
                type_name(unboxed)
            }
            _ => type_name(t),
        };
        res
    }

    pub fn return_type_name(t: &TypeIdentifier) -> Result<String, askama::Error> {
        let res: Result<String, askama::Error> = match t {
            Type::Optional(inner) => {
                let unboxed = inner.as_ref();
                let name = type_name(unboxed)?;
                Ok(format!("{name} | null"))
            }
            _ => type_name(t),
        };
        res
    }

    pub fn default_value(t: &TypeIdentifier) -> Result<String, askama::Error> {
        let res: Result<String, askama::Error> = match t {
            Type::Optional(inner) => {
                let unboxed = inner.as_ref();
                match unboxed {
                    Type::UInt8
                    | Type::Int8
                    | Type::UInt16
                    | Type::Int16
                    | Type::UInt32
                    | Type::Int32
                    | Type::UInt64
                    | Type::Int64
                    | Type::Float32
                    | Type::Float64 => Ok(" = 0".into()),
                    Type::String => Ok(" = \"\"".into()),
                    Type::Record(_) => Ok(" = {}".into()),
                    Type::Sequence(_) => Ok(" = []".into()),
                    _ => Ok("".into()),
                }
            }
            _ => Ok("".into()),
        };
        res
    }

    pub fn ignored_function(nm: &str) -> Result<bool, askama::Error> {
        Ok(IGNORED_FUNCTIONS.contains(nm))
    }
}
