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
        #[env_settings(skip)]
        names: Vec<String>,

        age: u8,
    }

    #[rstest]
    #[case(
        HashMap::from([]),
		vec!["lorem".to_string()],
        Err(EnvSettingsError::NotExists("age"))
    )]
    #[case(
        HashMap::from([("age", "42")]),
		vec!["lorem".to_string()],
        Ok(TestEnvSettings { names: vec!["lorem".to_string()], age: 42 })
    )]
    #[case(
        HashMap::from([("name", "[other]"), ("age", "42")]),
		vec!["lorem".to_string()],
        Ok(TestEnvSettings { names: vec!["lorem".to_string()], age: 42 })
    )]
    #[case(
        HashMap::from([("age", "other")]),
		vec!["lorem".to_string()],
        Err(EnvSettingsError::Convert("age", "other".to_string(), "u8"))
    )]
    fn test_from_env(
        #[case] env_variables: HashMap<&'static str, &'static str>,
        #[case] names: Vec<String>,
        #[case] expected_result: EnvSettingsResult<TestEnvSettings>,
    ) {
        let _ = with_env_variables(
            &env_variables,
            || TestEnvSettings::from_env(names.clone()),
            &expected_result,
        );
    }

    #[rstest]
    #[case(
        HashMap::from([]),
        vec!["lorem".to_string()],
        None,
        Err(EnvSettingsError::NotExists("age"))
    )]
    #[case(
        HashMap::from([("name", "[other]"), ("age", "42")]),
        vec!["lorem".to_string()],
        None,
        Ok(TestEnvSettings { names: vec!["lorem".to_string()], age: 42 })
    )]
    #[case(
        HashMap::from([]),
        vec!["lorem".to_string()],
        Some(42),
        Ok(TestEnvSettings { names: vec!["lorem".to_string()], age: 42 })
    )]
    #[case(
        HashMap::from([("age", "42")]),
        vec!["lorem".to_string()],
        None,
        Ok(TestEnvSettings { names: vec!["lorem".to_string()], age: 42 })
    )]
    #[case(
        HashMap::from([("age", "24")]),
        vec!["lorem".to_string()],
        Some(42),
        Ok(TestEnvSettings { names: vec!["lorem".to_string()], age: 42 })
    )]
    #[case(
        HashMap::from([("age", "other")]),
        vec!["lorem".to_string()],
        None,
        Err(EnvSettingsError::Convert("age", "other".to_string(), "u8"))
    )]
    fn test_new(
        #[case] env_variables: HashMap<&'static str, &'static str>,
        #[case] names: Vec<String>,
        #[case] age: Option<u8>,
        #[case] expected_result: EnvSettingsResult<TestEnvSettings>,
    ) {
        let _ = with_env_variables(
            &env_variables,
            || TestEnvSettings::new(names.clone(), age),
            &expected_result,
        );
    }
}
