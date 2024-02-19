#[cfg(test)]
mod tests {

    use crate::tests::{with_env_file_variables, with_env_variables};

    use env_settings_derive::EnvSettings;
    use env_settings_utils::{EnvSettingsError, EnvSettingsResult};
    use rstest::rstest;
    use std::collections::HashMap;
    use std::io;

    const FILE_PATH: &str = "test.env";

    #[derive(Debug, EnvSettings, PartialEq)]
    #[env_settings(delay, file_path = "test.env")]
    struct TestEnvSettings {
        name: String,
        age: u8,
    }

    #[rstest]
    #[case(
        FILE_PATH,
        HashMap::from([("age", "42")]),
        HashMap::from([("name", "lorem")]),
        Ok(TestEnvSettings { name: "lorem".to_string(), age: 42 })
    )]
    #[case(
        FILE_PATH,
        HashMap::from([("age", "42")]),
        HashMap::from([("name", "lorem"), ("age", "24")]),
        Ok(TestEnvSettings { name: "lorem".to_string(), age: 42 })
    )]
    #[case(
        FILE_PATH,
        HashMap::from([("name", "lorem"), ("age", "42")]),
        HashMap::from([("name", "other")]),
        Ok(TestEnvSettings { name: "lorem".to_string(), age: 42 })
    )]
    #[case(
        &format!("_{}", FILE_PATH),
        HashMap::from([]),
        HashMap::from([]),
        Err(
            EnvSettingsError::File(
                FILE_PATH.to_string(),
                dotenvy::Error::Io(io::Error::new(
                    io::ErrorKind::NotFound,
                    "No such file or directory (os error 2)"
                ))
            )
        ),
    )]
    fn test_from_env(
        #[case] file_path: &str,
        #[case] env_variables: HashMap<&'static str, &'static str>,
        #[case] env_file_variables: HashMap<&'static str, &'static str>,
        #[case] expected_result: EnvSettingsResult<TestEnvSettings>,
    ) {
        let _ = with_env_file_variables(
            file_path,
            &env_file_variables,
            || with_env_variables(&env_variables, TestEnvSettings::from_env, &expected_result),
            &expected_result,
        );
    }

    #[rstest]
    #[case(
        FILE_PATH,
        HashMap::from([("age", "24")]),
        HashMap::from([("name", "lorem")]),
        None,
        Some(42),
        Ok(TestEnvSettings { name: "lorem".to_string(), age: 42 })
    )]
    #[case(
        FILE_PATH,
        HashMap::from([("age", "42")]),
        HashMap::from([("name", "other"), ("age", "24")]),
        Some("lorem".to_string()),
        None,
        Ok(TestEnvSettings { name: "lorem".to_string(), age: 42 })
    )]
    #[case(
        FILE_PATH,
        HashMap::from([("name", "lorem")]),
        HashMap::from([("name", "other"), ("age", "42")]),
        None,
        None,
        Ok(TestEnvSettings { name: "lorem".to_string(), age: 42 })
    )]
    fn test_new(
        #[case] file_path: &str,
        #[case] env_variables: HashMap<&'static str, &'static str>,
        #[case] env_file_variables: HashMap<&'static str, &'static str>,
        #[case] name: Option<String>,
        #[case] age: Option<u8>,
        #[case] expected_result: EnvSettingsResult<TestEnvSettings>,
    ) {
        let _ = with_env_file_variables(
            file_path,
            &env_file_variables,
            || {
                with_env_variables(
                    &env_variables,
                    || TestEnvSettings::new(name.clone(), age),
                    &expected_result,
                )
            },
            &expected_result,
        );
    }
}
