use proc_macro2::TokenTree;
use std::collections::HashMap;
use syn::{
    parse, Attribute, Data, DeriveInput, Error, Fields, Ident, Meta, MetaList, Result, Type,
    TypePath,
};

/// The field info needed to the `EnvSettings` derive
#[derive(Debug)]
pub(crate) struct EnvSettingsField {
    /// The name of the field
    pub(crate) name: Ident,

    /// The type of the field
    pub(crate) type_: TypePath,

    /// The default value of the field
    pub(crate) default: Option<String>,

    /// The environment variable name
    pub(crate) variable: Option<String>,
}

/// The outer parameters of `EnvSettings` derive
#[derive(Debug, Default)]
pub(crate) struct EnvSettingsOuterParams {
    /// Whether the environment variables matching should be case insensitive
    pub(crate) case_insensitive: bool,

    /// Whether to delay the lookup for environment variables from compilation time to run time
    pub(crate) delay: bool,

    /// The path of the file to load
    pub(crate) file_path: Option<String>,

    /// The prefix to add the name of the struct fields to match the environment variables
    pub(crate) prefix: Option<String>,
}

/// The inner parameters of `EnvSettings` derive
#[derive(Debug, Default)]
pub(crate) struct EnvSettingsInnerParams {
    /// The default value to use if the environment variable is not set
    pub(crate) default: Option<String>,

    /// The environment variable name
    pub(crate) variable: Option<String>,
}

/// The `EnvSettings` macro input
#[derive(Debug)]
pub(crate) struct EnvSettingsInput {
    /// The parameters of `EnvSettings` derive
    pub(crate) params: EnvSettingsOuterParams,

    /// The identifier of the struct
    pub(crate) name: Ident,

    /// The fields of the struct
    pub(crate) fields: Vec<EnvSettingsField>,
}

impl EnvSettingsInput {
    /// Parse the attributes of the input
    fn parse_attributes(attributes: &[Attribute]) -> Result<HashMap<String, Option<String>>> {
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

    fn parse_inner_attributes(attributes: &[Attribute]) -> Result<EnvSettingsInnerParams> {
        let params = Self::parse_attributes(attributes)?;
        let mut env_settings_inner_params = EnvSettingsInnerParams::default();
        if let Some(default) = params.get("default") {
            env_settings_inner_params.default = default.to_owned();
        }
        if let Some(variable) = params.get("variable") {
            env_settings_inner_params.variable = variable.to_owned();
        }
        Ok(env_settings_inner_params)
    }

    fn parse_outer_attributes(attributes: &[Attribute]) -> Result<EnvSettingsOuterParams> {
        let params = Self::parse_attributes(attributes)?;
        let mut env_settings_outer_params = EnvSettingsOuterParams::default();
        if params.contains_key("case_insensitive") {
            env_settings_outer_params.case_insensitive = true;
        }
        if params.contains_key("delay") {
            env_settings_outer_params.delay = true;
        }
        if let Some(file_path) = params.get("file_path") {
            env_settings_outer_params.file_path = file_path.to_owned();
        }
        if let Some(prefix) = params.get("prefix") {
            env_settings_outer_params.prefix = prefix.to_owned();
        };
        Ok(env_settings_outer_params)
    }

    /// Parse the data of the input
    fn parse_data(data: &Data) -> Result<Vec<EnvSettingsField>> {
        match data {
            Data::Struct(_struct) => {
                if let Fields::Named(names_fields) = &_struct.fields {
                    let mut fields = Vec::new();
                    for field in &names_fields.named {
                        let params = Self::parse_inner_attributes(&field.attrs)?;
                        if let (Some(field_name), Type::Path(field_type)) =
                            (&field.ident, &field.ty)
                        {
                            let field = EnvSettingsField {
                                name: field_name.to_owned(),
                                type_: field_type.to_owned(),
                                default: params.default,
                                variable: params.variable,
                            };
                            fields.push(field);
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
        let params = EnvSettingsInput::parse_outer_attributes(&ast.attrs)?;
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
