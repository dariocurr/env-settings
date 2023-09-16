use crate::utils::input::EnvSettingsInput;

use syn::{Attribute, Result};

/// The inner parameters of `EnvSettings` derive
#[derive(Debug, Default)]
pub(crate) struct EnvSettingsInnerParams {
    /// The default value to use if the environment variable is not set
    pub(crate) default: Option<String>,

    /// The environment variable name
    pub(crate) variable: Option<String>,

    /// Whether to skip the parsing
    pub(crate) skip: bool,
}

impl EnvSettingsInnerParams {
    pub(crate) fn parse_attributes(attributes: &[Attribute]) -> Result<Self> {
        let params = EnvSettingsInput::parse_attributes(attributes)?;
        let mut env_settings_inner_params = EnvSettingsInnerParams::default();
        if let Some(default) = params.get("default") {
            env_settings_inner_params.default = default.to_owned();
        }
        if let Some(variable) = params.get("variable") {
            env_settings_inner_params.variable = variable.to_owned();
        }
        if params.contains_key("skip") {
            env_settings_inner_params.skip = true;
        }
        Ok(env_settings_inner_params)
    }
}
