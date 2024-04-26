use paste::paste;
use uniffi_bindgen::backend::{CodeOracle, CodeType, Literal};

macro_rules! impl_code_type_for_miscellany {
    ($T:ty, $canonical_name:literal) => {
        paste! {
            pub struct $T;

            impl CodeType for $T  {
                fn type_label(&self, _oracle: &dyn CodeOracle) -> String {
                    format!("{}", $canonical_name)
                }

                fn canonical_name(&self, _oracle: &dyn CodeOracle) -> String {
                    format!("{}", $canonical_name)
                }

                fn literal(&self, _oracle: &dyn CodeOracle, _literal: &Literal) -> String {
                    unreachable!()
                }

                fn coerce(&self, _oracle: &dyn CodeOracle, nm: &str) -> String {
                    nm.to_string()
                }
            }
        }
    };
}

impl_code_type_for_miscellany!(TimestampCodeType, "Date");
