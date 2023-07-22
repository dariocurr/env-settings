#![deny(missing_docs)]
#![doc(issue_tracker_base_url = "https://github.com/dariocurr/env-settings/issues")]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/dariocurr/env-settings/main/docs/logo.svg",
    html_favicon_url = "https://raw.githubusercontent.com/dariocurr/env-settings/main/docs/logo.ico"
)]

//! # **Env Settinsg Utilss**

use std::{collections, env, error, fmt};

/// The result type provided by `EnvSettings`
pub type EnvSettingsResult<T> = Result<T, EnvSettingsError>;

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
pub fn load_env_file_path(file_path: &str) -> EnvSettingsResult<()> {
    if let Err(err) = dotenvy::from_path(file_path) {
        Err(EnvSettingsError::File(file_path.to_string(), err.into()))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    use rstest::rstest;
    use std::io::prelude::Write;
    use std::{fs, path};

    #[rstest]
    #[case("KEY", "value", true, "key", Some("value"))]
    #[case("KEY", "value", true, "KEY", None)]
    #[case("KEY", "value", false, "key", None)]
    #[case("KEY", "value", false, "KEY", Some("value"))]
    fn test_get_env_variables(
        #[case] key: &str,
        #[case] value: &str,
        #[case] case_insensitive: bool,
        #[case] recover_key: &str,
        #[case] expected_result: Option<&str>,
    ) {
        env::set_var(key, value);
        let env_variables = get_env_variables(case_insensitive);
        let actual_result = env_variables.get(recover_key).map(|value| value.as_str());
        assert_eq!(actual_result, expected_result);
    }

    #[rstest]
    #[case("KEY", "value", Some("file_path"))]
    #[case("KEY", "value", None)]
    fn test_load_env_file_path(
        #[case] key: &str,
        #[case] value: &str,
        #[case] file_path: Option<&str>,
    ) {
        let (file_path, is_successful) = if let Some(file_path) = file_path {
            (file_path.to_string(), false)
        } else {
            let temp_dir: path::PathBuf = assert_fs::TempDir::new()
                .expect("Error occurs while creating the test temp directory!")
                .to_path_buf();
            fs::create_dir_all(&temp_dir)
                .expect("Error occurs while creating the test temp directory!");
            let temp_file_path = temp_dir.join("test_file");
            let pair = format!("{}={}\n", key, value);
            let mut temp_file = fs::File::create(&temp_file_path)
                .expect("Error occurs while creating the test temp file!");
            temp_file
                .write_all(pair.as_bytes())
                .expect("Error occurs while writing the test temp file!");
            (temp_file_path.to_string_lossy().to_string(), true)
        };
        let actual_result = load_env_file_path(&file_path);
        if is_successful {
            let actual_value = env::var(key).expect("Test environment variable not set!");
            assert_eq!(actual_value, value);
        } else {
            assert!(actual_result.is_err())
        }
    }
}
