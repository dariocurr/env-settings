#[cfg(test)]
mod tests {

    use crate::tests::with_env_variables;

    use env_settings_derive::EnvSettings;
    use env_settings_utils::{EnvSettingsError, EnvSettingsResult};
    use rstest::rstest;
    use std::collections::HashMap;

    #[derive(Debug, EnvSettings, PartialEq)]
    #[env_settings(delay)]
    struct TestEnvSettings {
        name: String,
        age: Option<u8>,
    }

    #[rstest]
    #[case(
        HashMap::from([]),
        Err(EnvSettingsError::NotExists("name"))
    )]
    #[case(
        HashMap::from([("name", "lorem")]),
        Ok(TestEnvSettings { name: "lorem".to_string(), age: None })
    )]
    #[case(
        HashMap::from([("age", "42")]),
        Err(EnvSettingsError::NotExists("name"))
    )]
    #[case(
        HashMap::from([("name", "lorem"), ("age", "42")]),
        Ok(TestEnvSettings { name: "lorem".to_string(), age: Some(42) })
    )]
    #[case(
        HashMap::from([("name", "lorem"), ("age", "other")]),
        Err(EnvSettingsError::Convert("age", "other".to_string(), "u8"))
    )]
    fn test_from_env(
        #[case] env_variables: HashMap<&'static str, &'static str>,
        #[case] expected_result: EnvSettingsResult<TestEnvSettings>,
    ) {
        let _ = with_env_variables(&env_variables, TestEnvSettings::from_env, &expected_result);
    }

    #[rstest]
    #[case(
        HashMap::from([]),
        None,
        None,
        Err(EnvSettingsError::NotExists("name"))
    )]
    #[case(
        HashMap::from([("name", "lorem"), ("age", "42")]),
        None,
        None,
        Ok(TestEnvSettings { name: "lorem".to_string(), age: Some(42) })
    )]
    #[case(
        HashMap::from([]),
        None,
        Some(42),
        Err(EnvSettingsError::NotExists("name"))
    )]
    #[case(
        HashMap::from([("name", "lorem")]),
        None,
        Some(42),
        Ok(TestEnvSettings { name: "lorem".to_string(), age: Some(42) })
    )]
    #[case(
        HashMap::from([]),
        Some("lorem".to_string()),
        None,
        Ok(TestEnvSettings { name: "lorem".to_string(), age: None })
    )]
    #[case(
        HashMap::from([("age", "42")]),
        Some("lorem".to_string()),
        None,
        Ok(TestEnvSettings { name: "lorem".to_string(), age: Some(42) })
    )]
    #[case(
        HashMap::from([]),
        Some("lorem".to_string()),
        Some(42),
        Ok(TestEnvSettings { name: "lorem".to_string(), age: Some(42) })
    )]
    #[case(
        HashMap::from([("name", "other"), ("age", "24")]),
        Some("lorem".to_string()),
        Some(42),
        Ok(TestEnvSettings { name: "lorem".to_string(), age: Some(42) })
    )]
    #[case(
        HashMap::from([("name", "lorem"), ("age", "other")]),
        None,
        None,
        Err(EnvSettingsError::Convert("age", "other".to_string(), "u8"))
    )]
    fn test_new(
        #[case] env_variables: HashMap<&'static str, &'static str>,
        #[case] name: Option<String>,
        #[case] age: Option<u8>,
        #[case] expected_result: EnvSettingsResult<TestEnvSettings>,
    ) {
        let _ = with_env_variables(
            &env_variables,
            || TestEnvSettings::new(name.clone(), age),
            &expected_result,
        );
    }
}
