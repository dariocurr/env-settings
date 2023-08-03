use proc_macro2::TokenTree;
use std::collections::HashMap;
use syn::{
    parse, Attribute, Data, DeriveInput, Error, Fields, Ident, Meta, MetaList, Result, Type,
};

/// The parameters of `EnvSettings` derive
#[derive(Debug, Default)]
pub(crate) struct EnvSettingsParams {
    /// Whether the environment variables matching should be case insensitive
    pub(crate) case_insensitive: bool,

    /// Whether to delay the lookup for environment variables from compilation time to run time
    pub(crate) delay: bool,

    /// The path of the file to load
    pub(crate) file_path: Option<String>,

    /// The prefix to add the name of the struct fields to match the environment variables
    pub(crate) prefix: String,
}

/// The `EnvSettings` macro input
#[derive(Debug)]
pub(crate) struct EnvSettingsInput {
    /// The parameters of `EnvSettings` derive
    pub(crate) params: EnvSettingsParams,

    /// The identifier of the struct
    pub(crate) name: Ident,

    /// The fields of the struct: name and type
    pub(crate) fields: Vec<(Ident, Ident)>,
}

impl EnvSettingsInput {
    /// Parse the attributes of the input
    fn parse_attributes(attributes: &[Attribute]) -> Result<EnvSettingsParams> {
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
        let mut env_settings_params = EnvSettingsParams::default();
        if params.contains_key("case_insensitive") {
            env_settings_params.case_insensitive = true;
        }
        if params.contains_key("delay") {
            env_settings_params.delay = true;
        }
        if let Some(file_path) = params.get("file_path") {
            env_settings_params.file_path = file_path.to_owned();
        }
        if let Some(Some(prefix)) = params.get("prefix") {
            env_settings_params.prefix = prefix.to_owned();
        };
        Ok(env_settings_params)
    }

    /// Parse the data of the input
    fn parse_data(data: &Data) -> Result<Vec<(Ident, Ident)>> {
        match data {
            Data::Struct(_struct) => {
                if let Fields::Named(names_fields) = &_struct.fields {
                    let mut fields = Vec::new();
                    for field in &names_fields.named {
                        if let (Some(field_name), Type::Path(field_type)) =
                            (&field.ident, &field.ty)
                        {
                            fields.push((
                                field_name.to_owned(),
                                field_type.path.segments[0].ident.to_owned(),
                            ));
                        }
                    }
                    Ok(fields)
                } else {
                    Err(Error::new(
                        _struct.struct_token.span,
                        "struct fields must be named",
                    ))
                }
            }
            Data::Enum(_enum) => Err(Error::new(_enum.enum_token.span, "enum not supported")),
            Data::Union(_union) => Err(Error::new(_union.union_token.span, "union not supported")),
        }
    }
}

/// Implement the parse method for `EnvSettingsInput`
impl parse::Parse for EnvSettingsInput {
    fn parse(input: parse::ParseStream) -> Result<Self> {
        let ast = DeriveInput::parse(input)?;
        let params = EnvSettingsInput::parse_attributes(&ast.attrs)?;
        let name = ast.ident;
        let fields = EnvSettingsInput::parse_data(&ast.data)?;
        let env_settings_input = EnvSettingsInput {
            params,
            name,
            fields,
        };
        Ok(env_settings_input)
    }
}
