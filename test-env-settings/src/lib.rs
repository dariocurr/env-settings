#![deny(missing_docs)]
#![doc(issue_tracker_base_url = "https://github.com/dariocurr/env-settings/issues")]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/dariocurr/env-settings/main/docs/logo.svg",
    html_favicon_url = "https://raw.githubusercontent.com/dariocurr/env-settings/main/docs/logo.ico"
)]

//! # Env Settings Test Library

#[cfg(test)]
mod tests {

    use env_settings_derive::EnvSettings;
    use env_settings_utils::EnvSettingsError;
    use rstest::rstest;
    use std::io::prelude::Write;
    use std::{collections, env, fmt, fs};

    const ENV_FILE_PATH: &str = "test.env";
    const ENV_PREFIX: &str = "TEST_";
    const TEMP_FILE_ERROR: &str =
        "Error occurs while managing the test environment variables file!";

    #[derive(Debug, EnvSettings, PartialEq)]
    #[env_settings(case_insensitive, delay, file_path = "test.env")]
    struct TestEnvSettings {
        name: String,
        age: u8,
    }

    #[derive(Debug, EnvSettings, PartialEq)]
    #[env_settings(delay, file_path = "test.env", prefix = "TEST_")]
    struct TestEnvSettingsWithPrefix {
        name: String,
        age: u8,
    }

    fn assert_result<T: fmt::Debug + PartialEq>(
        actual_result: Result<T, EnvSettingsError>,
        expected_result: Result<T, EnvSettingsError>,
    ) {
        if expected_result.is_err() {
            assert_eq!(
                actual_result.unwrap_err().to_string(),
                expected_result.unwrap_err().to_string()
            );
        } else {
            assert_eq!(actual_result.unwrap(), expected_result.unwrap());
        }
    }

    #[rstest]
    #[case(
        collections::HashMap::from([]),
        collections::HashMap::from([]),
        Err(EnvSettingsError::NotExists("name")))
    ]
    #[case(
        collections::HashMap::from([("name", "lorem")]),
        collections::HashMap::from([]),
        Err(EnvSettingsError::NotExists("age")))
    ]
    #[case(
        collections::HashMap::from([("age", "42")]),
        collections::HashMap::from([]),
        Err(EnvSettingsError::NotExists("name")))
    ]
    #[case(
        collections::HashMap::from([("name", "lorem"), ("age", "42")]),
        collections::HashMap::from([]),
        Ok(TestEnvSettings { name: "lorem".to_string(), age: 42 }))
    ]
    #[case(
        collections::HashMap::from([("NAME", "lorem"), ("age", "42")]),
        collections::HashMap::from([]),
        Ok(TestEnvSettings { name: "lorem".to_string(), age: 42 }))
    ]
    #[case(
        collections::HashMap::from([("age", "42")]),
        collections::HashMap::from([("NAME", "lorem")]),
        Ok(TestEnvSettings { name: "lorem".to_string(), age: 42 }))
    ]
    #[case(
        collections::HashMap::from([("age", "42")]),
        collections::HashMap::from([("NAME", "lorem"), ("age", "24")]),
        Ok(TestEnvSettings { name: "lorem".to_string(), age: 42 }))
    ]
    #[case(
        collections::HashMap::from([("name", "lorem"), ("age", "42")]),
        collections::HashMap::from([("name", "other")]),
        Ok(TestEnvSettings { name: "lorem".to_string(), age: 42 }))
    ]
    #[case(
        collections::HashMap::from([("name", "lorem"), ("age", "other")]),
        collections::HashMap::from([]),
        Err(EnvSettingsError::Convert("age", "other".to_string(), "u8")))
    ]
    fn test_from_env(
        #[case] env_variables: collections::HashMap<&'static str, &'static str>,
        #[case] env_file_variables: collections::HashMap<&'static str, &'static str>,
        #[case] expected_result: Result<TestEnvSettings, EnvSettingsError>,
    ) {
        let mut temp_file = fs::File::create(ENV_FILE_PATH).expect(TEMP_FILE_ERROR);
        env_file_variables.iter().for_each(|(key, value)| {
            let pair = format!("{}={}\n", key, value);
            temp_file.write_all(pair.as_bytes()).expect(TEMP_FILE_ERROR);
        });
        env_variables
            .iter()
            .for_each(|(key, value)| env::set_var(key, value));
        let actual_result = TestEnvSettings::from_env();
        [env_variables, env_file_variables]
            .into_iter()
            .for_each(|variables| variables.keys().for_each(env::remove_var));
        fs::remove_file(ENV_FILE_PATH).expect(TEMP_FILE_ERROR);
        assert_result(actual_result, expected_result);
    }

    #[rstest]
    #[case(
        collections::HashMap::from([]),
        collections::HashMap::from([]),
        Err(EnvSettingsError::NotExists("TEST_name")))
    ]
    #[case(
        collections::HashMap::from([("name", "lorem")]),
        collections::HashMap::from([]),
        Err(EnvSettingsError::NotExists("TEST_age")))
    ]
    #[case(
        collections::HashMap::from([("age", "42")]),
        collections::HashMap::from([]),
        Err(EnvSettingsError::NotExists("TEST_name")))
    ]
    #[case(
        collections::HashMap::from([("name", "lorem"), ("age", "42")]),
        collections::HashMap::from([]),
        Ok(TestEnvSettingsWithPrefix { name: "lorem".to_string(), age: 42 }))
    ]
    #[case(
        collections::HashMap::from([("age", "42")]),
        collections::HashMap::from([("name", "lorem")]),
        Ok(TestEnvSettingsWithPrefix { name: "lorem".to_string(), age: 42 }))
    ]
    #[case(
        collections::HashMap::from([("age", "42")]),
        collections::HashMap::from([("name", "lorem"), ("age", "24")]),
        Ok(TestEnvSettingsWithPrefix { name: "lorem".to_string(), age: 42 }))
    ]
    #[case(
        collections::HashMap::from([("name", "lorem"), ("age", "42")]),
        collections::HashMap::from([("name", "other")]),
        Ok(TestEnvSettingsWithPrefix { name: "lorem".to_string(), age: 42 }))
    ]
    #[case(
        collections::HashMap::from([("name", "lorem"), ("age", "other")]),
        collections::HashMap::from([]),
        Err(EnvSettingsError::Convert("age", "other".to_string(), "u8")))
    ]
    fn test_from_env_with_prefix(
        #[case] env_variables: collections::HashMap<&'static str, &'static str>,
        #[case] env_file_variables: collections::HashMap<&'static str, &'static str>,
        #[case] expected_result: Result<TestEnvSettingsWithPrefix, EnvSettingsError>,
    ) {
        let env_variables: collections::HashMap<String, &'static str> = env_variables
            .into_iter()
            .map(|(key, value)| (format!("{}{}", ENV_PREFIX, key), value))
            .collect();
        let env_file_variables: collections::HashMap<String, &'static str> = env_file_variables
            .into_iter()
            .map(|(key, value)| (format!("{}{}", ENV_PREFIX, key), value))
            .collect();
        let mut temp_file = fs::File::create(ENV_FILE_PATH).expect(TEMP_FILE_ERROR);
        env_file_variables.iter().for_each(|(key, value)| {
            let pair = format!("{}={}\n", key, value);
            temp_file.write_all(pair.as_bytes()).expect(TEMP_FILE_ERROR);
        });
        env_variables.iter().for_each(|(key, value)| {
            env::set_var(key, value);
        });
        let actual_result = TestEnvSettingsWithPrefix::from_env();
        [env_variables, env_file_variables]
            .into_iter()
            .for_each(|variables| variables.keys().for_each(env::remove_var));
        fs::remove_file(ENV_FILE_PATH).expect(TEMP_FILE_ERROR);
        assert_result(actual_result, expected_result);
    }

    #[rstest]
    #[case(
        collections::HashMap::from([]),
        collections::HashMap::from([]),
        None,
        None,
        Err(EnvSettingsError::NotExists("name")))
    ]
    #[case(
        collections::HashMap::from([("name", "lorem"), ("age", "42")]),
        collections::HashMap::from([]),
        None,
        None,
        Ok(TestEnvSettings { name: "lorem".to_string(), age: 42 }))
    ]
    #[case(
        collections::HashMap::from([]),
        collections::HashMap::from([]),
        None,
        Some(42),
        Err(EnvSettingsError::NotExists("name")))
    ]
    #[case(
        collections::HashMap::from([("name", "lorem")]),
        collections::HashMap::from([]),
        None,
        Some(42),
        Ok(TestEnvSettings { name: "lorem".to_string(), age: 42 }))
    ]
    #[case(
        collections::HashMap::from([]),
        collections::HashMap::from([]),
        Some("lorem".to_string()),
        None,
        Err(EnvSettingsError::NotExists("age")))
    ]
    #[case(
        collections::HashMap::from([("age", "42")]),
        collections::HashMap::from([]),
        Some("lorem".to_string()),
        None,
        Ok(TestEnvSettings { name: "lorem".to_string(), age: 42 }))
    ]
    #[case(
        collections::HashMap::from([]),
        collections::HashMap::from([]),
        Some("lorem".to_string()),
        Some(42),
        Ok(TestEnvSettings { name: "lorem".to_string(), age: 42 }))
    ]
    #[case(
        collections::HashMap::from([("name", "other"), ("age", "24")]),
        collections::HashMap::from([]),
        Some("lorem".to_string()),
        Some(42),
        Ok(TestEnvSettings { name: "lorem".to_string(), age: 42 }))
    ]
    fn test_new(
        #[case] env_variables: collections::HashMap<&'static str, &'static str>,
        #[case] env_file_variables: collections::HashMap<&'static str, &'static str>,
        #[case] name: Option<String>,
        #[case] age: Option<u8>,
        #[case] expected_result: Result<TestEnvSettings, EnvSettingsError>,
    ) {
        let mut temp_file = fs::File::create(ENV_FILE_PATH).expect(TEMP_FILE_ERROR);
        env_file_variables.iter().for_each(|(key, value)| {
            let pair = format!("{}={}\n", key, value);
            temp_file.write_all(pair.as_bytes()).expect(TEMP_FILE_ERROR);
        });
        env_variables
            .iter()
            .for_each(|(key, value)| env::set_var(key, value));
        let actual_result = TestEnvSettings::new(name, age);
        [env_variables, env_file_variables]
            .into_iter()
            .for_each(|variables| variables.keys().for_each(env::remove_var));
        fs::remove_file(ENV_FILE_PATH).expect(TEMP_FILE_ERROR);
        assert_result(actual_result, expected_result);
    }
}
