#[cfg(test)]
mod tests {

    use crate::tests::with_env_variables;

    use env_settings_derive::EnvSettings;
    use env_settings_utils::EnvSettingsResult;
    use rstest::rstest;
    use std::collections::HashMap;

    #[derive(Debug, EnvSettings, PartialEq)]
    #[env_settings(case_insensitive, delay)]
    struct TestEnvSettings {
        name: String,
        age: u8,
    }

    #[rstest]
    #[case(
        HashMap::from([("NAME", "lorem"), ("age", "42")]),
        Ok(TestEnvSettings { name: "lorem".to_string(), age: 42 })
    )]
    fn test_from_env(
        #[case] env_variables: HashMap<&'static str, &'static str>,
        #[case] expected_result: EnvSettingsResult<TestEnvSettings>,
    ) {
        let _ = with_env_variables(&env_variables, TestEnvSettings::from_env, &expected_result);
    }

    #[rstest]
    #[case(
        HashMap::from([("NAME", "lorem"), ("age", "42")]),
        None,
        Some(42),
        Ok(TestEnvSettings { name: "lorem".to_string(), age: 42 })
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
