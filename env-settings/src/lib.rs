#![deny(missing_docs)]
#![doc(issue_tracker_base_url = "https://github.com/dariocurr/env-settings/issues")]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/dariocurr/env-settings/main/docs/logo.svg",
    html_favicon_url = "https://raw.githubusercontent.com/dariocurr/env-settings/main/docs/logo.ico"
)]

//! # Env Settings
//!
//! **Env Settings** is a Rust library that helps you to initialize structs using environment variables
//!
//! > This Rust library took inspiration from [`pydantic's BaseSettings`](https://docs.pydantic.dev/latest/usage/pydantic_settings/) Python class
//!
//! ## Installation
//!
//! ```sh
//! cargo add env-settings
//! cargo add env-settings-derive
//! ```
//!
//! ## Usage
//!
//! When you add the `EnvSettings` derive to a `struct`, two methods are added to it
//!
//! -   ```text
//!     fn from_env() -> env_settings_utils::EnvSettingsResult<Self>
//!     ```
//!
//!     It creates a new instance using just the environment variables. If something fails, it returns an `env_settings_utils::EnvSettingsError` error
//!
//! -   ```text
//!     fn new(...) -> env_settings_utils::EnvSettingsResult<Self>
//!     ```
//!
//!     It creates a new instance using a combination of environment variables and parameters. More in detail, every field value can be passed as parameter wrapped in an `Option` object. Then if the parameter is `Some`, it is used, otherwise the value is recoved from the environment variables. If something fails, it returns an `env_settings_utils::EnvSettingsError` error
//!
//! ### Basic
//!
//! ```rust
//! // `export name=paolo` in shell or
//! std::env::set_var("name", "paolo");
//! // `export favourite_number=42` in shell or
//! std::env::set_var("favourite_number", "42");
//!
//!
//! use env_settings_derive::EnvSettings;
//!
//! // `delay` is necessary because environment variables are set at run time
//! #[derive(EnvSettings)]
//! #[env_settings(delay)]
//! struct MyStruct {
//!     name: String,
//!
//!     favourite_number: u8,
//! }
//!
//! let my_struct = MyStruct::from_env().unwrap();
//! assert_eq!(my_struct.name, "paolo".to_string());
//! assert_eq!(my_struct.favourite_number, 42);
//!
//! let name = "luca";
//! let my_struct = MyStruct::new(Some(name.to_string()), None).unwrap();
//! assert_eq!(my_struct.name, name);
//! assert_eq!(my_struct.favourite_number, 42);
//! ```
//!
//! ### Advanced
//!
//! ```rust
//! use std::io::prelude::Write;
//!
//! // `echo "MY_STRUCT_FAVOURITE_NUMBER=42\n" > .env` in shell or
//! let mut env_file = std::fs::File::create(".env").unwrap();
//! env_file.write_all("MY_STRUCT_FAVOURITE_NUMBER=42\n".as_bytes()).unwrap();
//! // `export MY_BIRTH_DATE=01/01/1970` in shell or
//! std::env::set_var("MY_BIRTH_DATE", "01/01/1970");
//!
//!
//! use env_settings_derive::EnvSettings;
//!
//! // `delay` is necessary because environment variables are set at run time
//! #[derive(EnvSettings)]
//! #[env_settings(case_insensitive, delay, file_path = ".env", prefix="MY_STRUCT_")]
//! struct MyStruct {
//!     #[env_settings(default = "paolo")]
//!     name: String,
//!
//!     favourite_number: u8,
//!
//!     #[env_settings(variable = "MY_BIRTH_DATE")]
//!     birth_date: String,
//!
//!     birth_place: Option<String>,
//!
//!     #[env_settings(skip)]
//!     friends: Vec<String>,
//! }
//!
//! let friends = vec!["luca".to_string()];
//! let my_struct = MyStruct::from_env(friends.clone()).unwrap();
//! assert_eq!(my_struct.name, "paolo".to_string());
//! assert_eq!(my_struct.favourite_number, 42);
//! assert_eq!(my_struct.birth_date, "01/01/1970");
//! assert_eq!(my_struct.birth_place, None);
//! assert_eq!(my_struct.friends, friends);
//!
//! let name = "luca";
//! let my_struct = MyStruct::new(
//!     Some(name.to_string()),
//!     None,
//!     None,
//!     Some("london".to_string()),
//!     friends.clone(),
//! ).unwrap();
//! assert_eq!(my_struct.name, name);
//! assert_eq!(my_struct.favourite_number, 42);
//! assert_eq!(my_struct.birth_date, "01/01/1970");
//! assert_eq!(my_struct.birth_place, Some("london".to_string()));
//! assert_eq!(my_struct.friends, friends);
//! ```
//!
//! ### Parameters
//!
//! #### Struct
//!
//! The current supported parameters for the structs are:
//!
//! -   `case_insensitive`: whether the environment variables matching should be case insensitive. By default, matching is case sensitive.
//! -   `delay`: whether to delay the lookup for environment variables from compilation time to run time. By default the lookup is performed at compilation time
//! -   `file_path`: the file path to read to add some environment variables (e.g. `.env`). By default, it is not set
//! -   `prefix`: the prefix to add to the name of the struct fields before matching the environment variables. By default, it is not set
//!
//! #### Field
//!
//! The current supported parameters for the fields are:
//!
//! -   `default`: the default value to use if the environment variable is not found. By default, it is not set
//! -   `skip`: whether to skip the parsing of the environment variable
//! -   `variable`: the environment variable to use for the lookup. By default, the name of the field
//!
//! ### Variables resolution hierarchy
//!
//! 1. Arguments passed to the `new` method (if using `new`).
//! 2. Environment variables
//! 3. Variables loaded from a file (e.g. `.env`)
//! 4. Default values
//!

/// The trait to add to the derive
pub trait EnvSettings {}
