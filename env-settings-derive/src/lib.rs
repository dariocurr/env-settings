#![deny(missing_docs)]
#![doc(issue_tracker_base_url = "https://github.com/dariocurr/env-settings/issues")]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/dariocurr/env-settings/main/docs/logo.svg",
    html_favicon_url = "https://raw.githubusercontent.com/dariocurr/env-settings/main/docs/logo.ico"
)]

//! # Env Settings Derive

use proc_macro::TokenStream;
use quote::quote;
use std::collections::HashMap;
use syn::parse;
use utils::EnvSettingsField;

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

    for EnvSettingsField {
        name,
        type_,
        default,
    } in &input.fields
    {
        let prefix = input.params.prefix.clone().unwrap_or(String::default());
        let mut env_variable = format!("{}{}", prefix, name);
        if case_insensitive {
            env_variable = env_variable.to_lowercase();
        }
        let name_string = name.to_string();
        let type_string = type_.to_string();
        let default_impl = match default {
            Some(default_value) => quote! {
                match #default_value.parse::<#type_>() {
                    Ok(default_value) => default_value,
                    Err(_) => return Err(env_settings_utils::EnvSettingsError::Convert(
                        #name_string,
                        #default_value.to_owned(),
                        #type_string,
                    ))
                }
            },
            None => {
                quote! {
                   return Err(env_settings_utils::EnvSettingsError::NotExists(#env_variable))
                }
            }
        };
        let env_value_impl = if input.params.delay {
            quote! {
                match env_variables.get(#env_variable) {
                    Some(env_value) => match env_value.parse::<#type_>() {
                        Ok(env_value) => env_value,
                        Err(_) => return Err(env_settings_utils::EnvSettingsError::Convert(
                            #name_string,
                            env_value.to_owned(),
                            #type_string,
                        )),
                    },
                    None => #default_impl,
                }
            }
        } else {
            match env_variables.get(&env_variable) {
                Some(env_value) => {
                    quote! {
                        match #env_value.parse::<#type_>() {
                            Ok(env_value) => env_value,
                            Err(_) => return Err(env_settings_utils::EnvSettingsError::Convert(
                                #name_string,
                                #env_value.to_owned(),
                                #type_string,
                            ))
                        }
                    }
                }
                None => quote! { #default_impl },
            }
        };
        from_env_impls.push(quote! {
            #name: #env_value_impl
        });
        new_impls.push(quote! {
            #name: match #name {
                Some(field_value) => field_value,
                None => #env_value_impl
            }
        });
        new_args.push(quote!(
            #name: Option<#type_>
        ));
    }
    let gen = quote! {

        impl #struct_name {

            fn new(#(#new_args),*) -> env_settings_utils::EnvSettingsResult<Self> {
                #file_path_impls
                #env_variables_impls

                let instance = Self {
                    #(#new_impls),*
                };
                Ok(instance)
            }

            fn from_env() -> env_settings_utils::EnvSettingsResult<Self> {
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
