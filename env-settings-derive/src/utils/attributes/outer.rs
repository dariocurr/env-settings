use crate::utils::input::EnvSettingsInput;

use syn::{Attribute, Result};

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

impl EnvSettingsOuterParams {
    pub(crate) fn parse_attributes(attributes: &[Attribute]) -> Result<Self> {
        let params = EnvSettingsInput::parse_attributes(attributes)?;
        let mut env_settings_outer_params = EnvSettingsOuterParams::default();
        if params.contains_key("case_insensitive") {
            env_settings_outer_params.case_insensitive = true;
        }
        if params.contains_key("delay") {
            env_settings_outer_params.delay = true;
        }
        if let Some(file_path) = params.get("file_path") {
            file_path.clone_into(&mut env_settings_outer_params.file_path);
        }
        if let Some(prefix) = params.get("prefix") {
            prefix.clone_into(&mut env_settings_outer_params.prefix);
        };
        Ok(env_settings_outer_params)
    }
}
