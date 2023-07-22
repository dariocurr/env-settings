#![deny(missing_docs)]
#![doc(issue_tracker_base_url = "https://github.com/dariocurr/env-settings/issues")]

//! # **Env Settinsg Utilss**

use std::{collections, env, error, fmt};

/// The error that may occurs during `EnvSettings` resolution
#[derive(Debug)]
pub enum EnvSettingsError {
    /// Error raised when a convertion fails
    Convert(&'static str, String, &'static str),

    /// Error raised when environment variables resolution from a file fails
    File(String, Box<dyn error::Error>),

    /// Error raised when an environment variable not exists
    NotExists(&'static str),
}

impl fmt::Display for EnvSettingsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EnvSettingsError::NotExists(env_variable) => {
                write!(f, "Environment variable named `{}` not found", env_variable)
            }
            EnvSettingsError::Convert(field_name, field_value, field_type) => write!(
                f,
                "Unable to convert the field `{}`: `{}` to `{}`",
                field_name, field_value, field_type
            ),
            EnvSettingsError::File(file_path, err) => write!(
                f,
                "Error occurs while reading `{}` as environment variable file: {}",
                file_path, err
            ),
        }
    }
}

/// Get the environment variables
pub fn get_env_variables(case_insensitive: bool) -> collections::HashMap<String, String> {
    let env_variables = env::vars();
    if case_insensitive {
        env_variables
            .map(|(key, value)| (key.to_lowercase(), value))
            .collect()
    } else {
        env_variables.collect()
    }
}

/// Load the environment variables file path
pub fn load_env_file_path(file_path: &str) -> Result<(), EnvSettingsError> {
    if let Err(err) = dotenvy::from_path(file_path) {
        Err(EnvSettingsError::File(file_path.to_string(), err.into()))
    } else {
        Ok(())
    }
}
