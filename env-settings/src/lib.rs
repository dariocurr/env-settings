#![deny(missing_docs)]
#![doc(issue_tracker_base_url = "https://github.com/dariocurr/env-settings/issues")]

//! # Env settings
//!
//! **Env Settings** is a Rust library that helps you to initialize structs using environment variables
//!
//! > This Rust library took inspiration from [`pydantic.BaseSettings`](https://docs.pydantic.dev/latest/usage/pydantic_settings/) Python class
//!
//! ## Installation
//!
//! ```ignore
//! cargo add env-settings
//! ```
//!
//! ## Usage
//!
//! When you add the `EnvSettings` derive to a `struct`, two methods are added to it
//!
//! -   ```ignore
//!     fn from_env() -> Result<Self, env_settings_utils::EnvSettingsError>
//!     ```
//!
//!     It creates a new instance using just the environment variables. If something fails, it returns an `env_settings_utils::EnvSettingsError` error
//!
//! -   ```ignore
//!     fn new(...) -> Result<Self, env_settings_utils::EnvSettingsError>
//!     ```
//!
//!     It creates a new instance using a combination of environment variables and parameters. More in detail, every field value can be passed as parameter wrapped in an `Option` object. Then if the parameter is `Some`, it is used, otherwise the value is recoved from the environment variables. If something fails, it returns an `env_settings_utils::EnvSettingsError` error
//!
//! ### Basic
//!
//! ```
//! // `export name=paolo` in shell or
//! std::env::set_var("name", "paolo");
//! // `export age=42` in shell or
//! std::env::set_var("age", "42");
//!
//!
//! use env_settings_derive::EnvSettings;
//!
//! // `delay` is necessary because environment variables are set at run time
//! #[derive(EnvSettings)]
//! #[env_settings(delay)]
//! struct MyStruct {
//!     name: String,
//!     age: u8,
//! }
//!
//! let my_struct = MyStruct::from_env().unwrap();
//! assert_eq!(my_struct.name, "paolo".to_string());
//! assert_eq!(my_struct.age, 42);
//!
//! let name = "luca";
//! let my_struct = MyStruct::new(Some(name.to_string()), None).unwrap();
//! assert_eq!(my_struct.name, name);
//! assert_eq!(my_struct.age, 42);
//! ```
//!
//! ### Advanced
//!
//! ```
//! use std::io::prelude::Write;
//!
//! // `export MY_STRUCT_NAME=paolo` in shell or
//! std::env::set_var("MY_STRUCT_NAME", "paolo");
//! // `echo "MY_STRUCT_AGE=42\n" > .env` in shell or
//! let mut env_file = std::fs::File::create(".env").unwrap();
//! env_file.write_all("MY_STRUCT_AGE=42\n".as_bytes()).unwrap();
//!
//!
//! use env_settings_derive::EnvSettings;
//!
//! // `delay` is necessary because environment variables are set at run time
//! #[derive(EnvSettings)]
//! #[env_settings(case_insensitive, delay, file_path = ".env", prefix="MY_STRUCT_")]
//! struct MyStruct {
//!     name: String,
//!     age: u8,
//! }
//!
//! let my_struct = MyStruct::from_env().unwrap();
//! assert_eq!(my_struct.name, "paolo".to_string());
//! assert_eq!(my_struct.age, 42);
//!
//! let name = "luca";
//! let my_struct = MyStruct::new(Some(name.to_string()), None).unwrap();
//! assert_eq!(my_struct.name, name);
//! assert_eq!(my_struct.age, 42);
//! ```
//!
//! ### Parameters
//!
//! The current supported parameters are:
//!
//! -   `case_insensitive`: add it if the environment variables matching should be case insensitive
//! -   `delay`: add it to delay the lookup for environment variables from compilation time to run time
//! -   `file_path`: add it to specify a file path to read to add some environment variables (e.g. `.env`)
//! -   `prefix`: add it to specify a prefix to add to the name of the struct fields before matching the environment variables
//!
//! ### Variables resolution hierarchy
//!
//! 1. Arguments passed to the `new` method (if using `new`).
//! 2. Environment variables
//! 3. Variables loaded from a file (e.g. `.env`)

/// The trait to add to the derive
pub trait EnvSettings {}
