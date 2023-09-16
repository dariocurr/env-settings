use crate::utils::{attributes, field};

use proc_macro2::TokenTree;
use std::collections::HashMap;
use syn::{parse, Attribute, DeriveInput, Error, Ident, Meta, MetaList, Result};

/// The `EnvSettings` macro input
#[derive(Debug)]
pub(crate) struct EnvSettingsInput {
    /// The parameters of `EnvSettings` derive
    pub(crate) params: attributes::outer::EnvSettingsOuterParams,

    /// The identifier of the struct
    pub(crate) name: Ident,

    /// The fields of the struct
    pub(crate) fields: Vec<field::EnvSettingsField>,
}

impl EnvSettingsInput {
    /// Parse the attributes of the input
    pub(crate) fn parse_attributes(
        attributes: &[Attribute],
    ) -> Result<HashMap<String, Option<String>>> {
        let mut params = HashMap::new();
        for attribute in attributes {
            if attribute.meta.path().is_ident("env_settings") {
                if let Meta::List(MetaList { tokens, .. }) = &attribute.meta {
                    let mut tokens_iterator = tokens.clone().into_iter();
                    while let Some(token) = tokens_iterator.next() {
                        match token {
                            TokenTree::Ident(ident) => {
                                if let Some(TokenTree::Punct(punct)) = tokens_iterator.next() {
                                    match punct.as_char() {
                                        '=' => {
                                            if let Some(TokenTree::Literal(literal)) =
                                                tokens_iterator.next()
                                            {
                                                let value = literal.to_string().replace('\"', "");
                                                params.insert(ident.to_string(), Some(value));
                                            } else {
                                                return Err(Error::new(
                                                    punct.span(),
                                                    "literal value expected",
                                                ));
                                            }
                                        }
                                        ',' => {
                                            params.insert(ident.to_string(), None);
                                        }
                                        _ => {
                                            let error_message =
                                                format!("punct value `{}` unexpected", punct);
                                            return Err(Error::new(punct.span(), error_message));
                                        }
                                    }
                                } else {
                                    params.insert(ident.to_string(), None);
                                }
                            }
                            TokenTree::Punct(punct) => {
                                if punct.as_char() != ',' {
                                    let error_message =
                                        format!("punct value `{}` unexpected", punct);
                                    return Err(Error::new(punct.span(), error_message));
                                }
                            }
                            _ => {
                                let error_message = format!("token value `{}` unexpected", token);
                                return Err(Error::new(token.span(), error_message));
                            }
                        };
                    }
                }
            }
        }
        Ok(params)
    }
}

/// Implement the parse method for `EnvSettingsInput`
impl parse::Parse for EnvSettingsInput {
    fn parse(input: parse::ParseStream) -> Result<Self> {
        let ast = DeriveInput::parse(input)?;
        let params = attributes::outer::EnvSettingsOuterParams::parse_attributes(&ast.attrs)?;
        let name = ast.ident;
        let fields = field::EnvSettingsField::parse_fields(&ast.data)?;
        let env_settings_input = EnvSettingsInput {
            params,
            name,
            fields,
        };
        Ok(env_settings_input)
    }
}
