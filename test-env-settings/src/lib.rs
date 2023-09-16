#![deny(missing_docs)]
#![doc(issue_tracker_base_url = "https://github.com/dariocurr/env-settings/issues")]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/dariocurr/env-settings/main/docs/logo.svg",
    html_favicon_url = "https://raw.githubusercontent.com/dariocurr/env-settings/main/docs/logo.ico"
)]

//! # Env Settings Test Library

mod basic;
mod case_insensitive;
mod default;
mod e2e;
mod file_path;
mod option;
mod prefix;
mod skip;
mod variable;

#[cfg(test)]
mod tests {

    use env_settings_utils::EnvSettingsResult;
    use std::io::prelude::Write;
    use std::{collections, env, ffi, fmt, fs};

    const TEMP_FILE_ERROR: &str =
        "Error occurs while managing the test environment variables file!";

    pub(crate) fn with_env_variables<E: fmt::Debug + PartialEq, F: Fn() -> EnvSettingsResult<E>>(
        env_variables: &collections::HashMap<
            &(impl AsRef<ffi::OsStr> + ?Sized),
            &(impl AsRef<ffi::OsStr> + ?Sized),
        >,
        fn_: F,
        expected_result: &EnvSettingsResult<E>,
    ) -> EnvSettingsResult<E> {
        env_variables
            .iter()
            .for_each(|(key, value)| env::set_var(key, value));
        let actual_result = fn_();
        env_variables.keys().for_each(env::remove_var);
        assert_result(&actual_result, expected_result);
        actual_result
    }

    pub(crate) fn with_env_file_variables<
        E: fmt::Debug + PartialEq,
        F: Fn() -> EnvSettingsResult<E>,
    >(
        file_path: &str,
        env_file_variables: &collections::HashMap<
            &(impl fmt::Display + AsRef<ffi::OsStr> + ?Sized),
            &(impl fmt::Display + AsRef<ffi::OsStr> + ?Sized),
        >,
        fn_: F,
        expected_result: &EnvSettingsResult<E>,
    ) -> EnvSettingsResult<E> {
        let mut temp_file = fs::File::create(file_path).expect(TEMP_FILE_ERROR);
        env_file_variables.iter().for_each(|(key, value)| {
            let pair = format!("{}={}\n", key, value);
            temp_file.write_all(pair.as_bytes()).expect(TEMP_FILE_ERROR);
        });
        let actual_result = fn_();
        fs::remove_file(file_path).expect(TEMP_FILE_ERROR);
        env_file_variables.keys().for_each(env::remove_var);
        assert_result(&actual_result, expected_result);
        actual_result
    }

    fn assert_result<T: fmt::Debug + PartialEq>(
        actual_result: &EnvSettingsResult<T>,
        expected_result: &EnvSettingsResult<T>,
    ) {
        if expected_result.is_err() {
            assert_eq!(
                actual_result.as_ref().unwrap_err().to_string(),
                expected_result.as_ref().unwrap_err().to_string()
            );
        } else {
            assert_eq!(
                actual_result.as_ref().unwrap(),
                expected_result.as_ref().unwrap()
            );
        }
    }
}
