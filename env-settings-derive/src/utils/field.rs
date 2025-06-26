use crate::utils::attributes::inner::EnvSettingsInnerParams;

use syn::{
    Attribute, Data, Error, Fields, GenericArgument, Ident, PathArguments, PathSegment, Result,
    Type, TypePath, punctuated, token,
};

/// A non parsable field
pub(crate) struct NonParsableField {
    /// The name of the field
    pub(crate) name: Ident,

    /// The type of the field
    pub(crate) type_: Type,
}

/// A parsable field
pub(crate) struct ParsableField {
    /// The name of the field
    pub(crate) name: Ident,

    /// The name label of the field
    pub(crate) name_label: String,

    /// The type of the field
    pub(crate) type_: Type,

    /// The type label of the field
    pub(crate) type_label: String,

    /// The default value of the field
    pub(crate) default: Option<String>,

    /// The type specified in the option
    pub(crate) optional_type: Option<Type>,

    /// The environment variable name
    pub(crate) variable: Option<String>,
}

/// The field info needed to the `EnvSettings` derive
pub(crate) enum EnvSettingsField {
    /// A non parsable field
    NonParsable(Box<NonParsableField>),

    //// A parsable field
    Parsable(Box<ParsableField>),
}

impl EnvSettingsField {
    fn get_field(type_: &Type, name: &Ident, attrs: &[Attribute]) -> Result<Self> {
        let params = EnvSettingsInnerParams::parse_attributes(attrs)?;
        let non_parsable_field = NonParsableField {
            name: name.to_owned(),
            type_: type_.to_owned(),
        };
        let non_parsable_field = EnvSettingsField::NonParsable(Box::new(non_parsable_field));
        let field = if params.skip {
            non_parsable_field
        } else {
            match &type_ {
                Type::Path(type_path) => {
                    Self::get_field_from_type_path(type_, type_path, name, params)?
                }
                _ => non_parsable_field,
            }
        };
        Ok(field)
    }

    fn get_field_from_type_path(
        type_: &Type,
        type_path: &TypePath,
        name: &Ident,
        params: EnvSettingsInnerParams,
    ) -> Result<Self> {
        let mut segments = type_path.path.segments.to_owned();
        let optional_type = Self::get_optional_type(&segments);
        if let Some(Type::Path(optional_type_path)) = &optional_type {
            optional_type_path.path.segments.clone_into(&mut segments);
        }
        let type_label = segments
            .into_iter()
            .map(|segment| segment.ident.to_string())
            .collect::<Vec<String>>()
            .join("::");

        let parsable_field = ParsableField {
            name: name.to_owned(),
            name_label: name.to_string(),
            type_: type_.to_owned(),
            type_label,
            default: params.default,
            optional_type,
            variable: params.variable,
        };
        let parsable_field = EnvSettingsField::Parsable(Box::new(parsable_field));
        Ok(parsable_field)
    }

    fn get_optional_type(
        segments: &punctuated::Punctuated<PathSegment, token::PathSep>,
    ) -> Option<Type> {
        if segments[0].ident == "Option" {
            if let PathArguments::AngleBracketed(arguments) = &segments[0].arguments {
                if let GenericArgument::Type(optional_type) = &arguments.args[0] {
                    return Some(optional_type.to_owned());
                }
            }
        }
        None
    }

    /// Parse the fields of the input
    pub(crate) fn parse_fields(data: &Data) -> Result<Vec<Self>> {
        match data {
            Data::Struct(_struct) => {
                if let Fields::Named(names_fields) = &_struct.fields {
                    let mut fields = Vec::new();
                    for field in &names_fields.named {
                        if let Some(field_name) = &field.ident {
                            let field = Self::get_field(&field.ty, field_name, &field.attrs)?;
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
