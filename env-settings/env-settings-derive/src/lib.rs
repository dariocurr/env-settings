#![deny(missing_docs)]
#![doc(issue_tracker_base_url = "https://github.com/dariocurr/env-settings/issues")]

//! # Env Settings Derive

use proc_macro::TokenStream;
use quote::quote;
use std::collections::HashMap;
use syn::parse;

mod utils;

/// The macro to add the `Derive` functionality
#[proc_macro_derive(EnvSettings, attributes(env_settings))]
pub fn env_settings_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree that we can manipulate
    let input = parse(input).unwrap();

    // Build the trait implementation
    implement(&input)
}

/// Implement the logic of the derive macro
fn implement(input: &utils::EnvSettingsInput) -> TokenStream {
    let struct_name = &input.name;
    let mut new_args = Vec::new();
    let mut new_impls = Vec::new();
    let mut from_env_impls = Vec::new();
    let mut env_variables_impls = quote! {};
    let mut file_path_impls = quote! {};
    if let Some(file_path) = &input.params.file_path {
        if input.params.delay {
            file_path_impls = quote! {
                env_settings_utils::load_env_file_path(#file_path)?;
            }
        } else {
            env_settings_utils::load_env_file_path(file_path).unwrap();
        }
    }
    let case_insensitive = input.params.case_insensitive;
    let env_variables = if input.params.delay {
        env_variables_impls = quote! {
            let env_variables = env_settings_utils::get_env_variables(#case_insensitive);
        };
        HashMap::new()
    } else {
        env_settings_utils::get_env_variables(case_insensitive)
    };

    for (field_name, field_type) in &input.fields {
        let mut env_variable = format!("{}{}", input.params.prefix, field_name);
        if case_insensitive {
            env_variable = env_variable.to_lowercase();
        }
        let field_name_string = field_name.to_string();
        let field_type_string = field_type.to_string();
        let env_value = if input.params.delay {
            quote! {
                match env_variables.get(#env_variable) {
                    Some(env_value) => match env_value.parse::<#field_type>() {
                        Ok(env_value) => env_value,
                        Err(_) => return Err(env_settings_utils::EnvSettingsError::Convert(
                            #field_name_string,
                            env_value.to_owned(),
                            #field_type_string,
                        )),
                    },
                    None => return Err(
                        env_settings_utils::EnvSettingsError::NotExists(#env_variable)
                    ),
                }
            }
        } else {
            match env_variables.get(&env_variable) {
                Some(env_value) => {
                    quote! {
                        match #env_value.parse::<#field_type>() {
                            Ok(env_value) => env_value,
                            Err(_) => return Err(env_settings_utils::EnvSettingsError::Convert(
                                #field_name_string,
                                env_value.to_owned(),
                                #field_type_string,
                            ))
                        }
                    }
                }
                None => {
                    quote! {
                       return Err(env_settings_utils::EnvSettingsError::NotExists(#env_variable))
                    }
                }
            }
        };
        from_env_impls.push(quote! {
            #field_name: #env_value
        });
        new_impls.push(quote! {
            #field_name: match #field_name {
                Some(field_value) => field_value,
                None => #env_value
            }
        });
        new_args.push(quote!(
            #field_name: Option<#field_type>
        ));
    }
    let gen = quote! {

        impl #struct_name {

            fn new(#(#new_args),*) -> Result<Self, env_settings_utils::EnvSettingsError> {
                #file_path_impls
                #env_variables_impls

                let instance = Self {
                    #(#new_impls),*
                };
                Ok(instance)
            }

            fn from_env() -> Result<Self, env_settings_utils::EnvSettingsError> {
                #file_path_impls
                #env_variables_impls
                let instance = Self {
                    #(#from_env_impls),*
                };
                Ok(instance)
            }

        }

    };
    gen.into()
}
