#[cfg(test)]
mod tests {

    use crate::tests::{with_env_file_variables, with_env_variables};

    use env_settings_derive::EnvSettings;
    use env_settings_utils::{EnvSettingsError, EnvSettingsResult};
    use rstest::rstest;
    use std::collections::HashMap;

    const FILE_PATH: &str = "test.env";

    #[derive(Debug, EnvSettings, PartialEq)]
    #[env_settings(case_insensitive, delay, file_path = "test.env", prefix = "TEST_")]
    struct TestEnvSettings {
        name: String,

        #[env_settings(default = 24)]
        age: u8,
    }

    #[rstest]
    #[case(
		FILE_PATH,
        HashMap::from([]),
        HashMap::from([]),
        Err(EnvSettingsError::NotExists("test_name"))
    )]
    #[case(
		FILE_PATH,
        HashMap::from([("TEST_name", "lorem")]),
        HashMap::from([]),
        Ok(TestEnvSettings { name: "lorem".to_string(), age: 24 })
    )]
    #[case(
		FILE_PATH,
        HashMap::from([("TEST_age", "42")]),
        HashMap::from([]),
        Err(EnvSettingsError::NotExists("test_name"))
    )]
    #[case(
		FILE_PATH,
        HashMap::from([("TEST_name", "lorem"), ("TEST_age", "42")]),
        HashMap::from([]),
        Ok(TestEnvSettings { name: "lorem".to_string(), age: 42 })
    )]
    #[case(
		FILE_PATH,
        HashMap::from([("TEST_age", "42")]),
        HashMap::from([("TEST_name", "lorem")]),
        Ok(TestEnvSettings { name: "lorem".to_string(), age: 42 })
    )]
    #[case(
		FILE_PATH,
        HashMap::from([("TEST_age", "42")]),
        HashMap::from([("TEST_name", "lorem"), ("TEST_age", "24")]),
        Ok(TestEnvSettings { name: "lorem".to_string(), age: 42 })
    )]
    #[case(
		FILE_PATH,
        HashMap::from([("TEST_name", "lorem"), ("TEST_age", "42")]),
        HashMap::from([("TEST_name", "other")]),
        Ok(TestEnvSettings { name: "lorem".to_string(), age: 42 })
    )]
    #[case(
		FILE_PATH,
        HashMap::from([("TEST_name", "lorem"), ("TEST_age", "other")]),
        HashMap::from([]),
        Err(EnvSettingsError::Convert("age", "other".to_string(), "u8"))
    )]
    fn test_from_env_with_prefix(
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
        HashMap::from([]),
        HashMap::from([]),
        None,
        None,
        Err(EnvSettingsError::NotExists("test_name"))
    )]
    #[case(
		FILE_PATH,
        HashMap::from([("TEST_name", "lorem")]),
        HashMap::from([("TEST_age", "42")]),
        None,
        None,
        Ok(TestEnvSettings { name: "lorem".to_string(), age: 42 })
    )]
    #[case(
		FILE_PATH,
        HashMap::from([]),
        HashMap::from([]),
        None,
        Some(42),
        Err(EnvSettingsError::NotExists("test_name"))
    )]
    #[case(
		FILE_PATH,
        HashMap::from([("TEST_name", "lorem")]),
        HashMap::from([]),
        None,
        Some(42),
        Ok(TestEnvSettings { name: "lorem".to_string(), age: 42 })
    )]
    #[case(
		FILE_PATH,
        HashMap::from([]),
        HashMap::from([]),
        Some("lorem".to_string()),
        None,
        Ok(TestEnvSettings { name: "lorem".to_string(), age: 24 })
    )]
    #[case(
		FILE_PATH,
        HashMap::from([("TEST_age", "42")]),
        HashMap::from([]),
        Some("lorem".to_string()),
        None,
        Ok(TestEnvSettings { name: "lorem".to_string(), age: 42 })
    )]
    #[case(
		FILE_PATH,
        HashMap::from([]),
        HashMap::from([]),
        Some("lorem".to_string()),
        Some(42),
        Ok(TestEnvSettings { name: "lorem".to_string(), age: 42 })
    )]
    #[case(
		FILE_PATH,
        HashMap::from([("TEST_name", "other"), ("TEST_age", "24")]),
        HashMap::from([]),
        Some("lorem".to_string()),
        Some(42),
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
