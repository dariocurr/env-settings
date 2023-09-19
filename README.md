# Env Settings

<p align="center">
    <img src="./docs/logo.svg" alt="Env Settings" width="128" height="128">
</p>

<div align="center">

<a href="https://crates.io/crates/env-settings">![crates](https://img.shields.io/crates/v/env-settings.svg)</a>
<a href="https://github.com/dariocurr/env-settings/blob/main/Cargo.toml#L29">![license](https://img.shields.io/crates/l/env-settings)</a>
<a href="https://github.com/dariocurr/env-settings/actions/workflows/validate.yml">![validate](https://github.com/dariocurr/env-settings/actions/workflows/validate.yml/badge.svg)</a>

</div>

**Env Settings** is a Rust library that helps you to initialize structs using environment variables

> This Rust library took inspiration from [`pydantic's BaseSettings`](https://docs.pydantic.dev/latest/usage/pydantic_settings/) Python class

## Installation

```shell
cargo add env-settings
cargo add env-settings-derive
```

## Usage

When you add the `EnvSettings` derive to a `struct`, two public methods are added to it

-   ```rust
    fn from_env(...) -> env_settings_utils::EnvSettingsResult<Self>
    ```

    Create a new instance using just the environment variables. Skipped fields must be passed. If something fails, it returns an `env_settings_utils::EnvSettingsError` error

-   ```rust
    fn new(...) -> env_settings_utils::EnvSettingsResult<Self>
    ```

    Create a new instance using a combination of environment variables and parameters. More in detail, every field that can be initialized by the environment variables can be passed as parameter wrapped in an `Option` object. Then if the parameter is `Some`, it is used, otherwise the value is recoved from the environment variables. Skipped fields must be passed. If something fails, it returns an `env_settings_utils::EnvSettingsError` error

### Basic

```shell
export name=paolo
export favourite_number=42
```

```rust
use env_settings_derive::EnvSettings;

#[derive(EnvSettings)]
struct MyStruct {
    name: String,

    favourite_number: u8,
}

fn main() {
    let my_struct = MyStruct::from_env().unwrap();
    assert_eq!(my_struct.name, "paolo".to_string());
    assert_eq!(my_struct.favourite_number, 42);

    let name = "luca";
    let my_struct = MyStruct::new(Some(name.to_string()), None).unwrap();
    assert_eq!(my_struct.name, name);
    assert_eq!(my_struct.favourite_number, 42);
}
```

### Advanced

```shell
echo "MY_STRUCT_FAVOURITE_NUMBER=42\n" > .env
export MY_BIRTH_DATE=01/01/1970
```

```rust
use env_settings_derive::EnvSettings;

#[derive(EnvSettings)]
#[env_settings(case_insensitive, file_path = ".env", prefix="MY_STRUCT_")]
struct MyStruct {
    #[env_settings(default = "paolo")]
    name: String,

    favourite_number: u8,

    #[env_settings(variable = "MY_BIRTH_DATE")]
    birth_date: String,

    birth_place: Option<String>,

    #[env_settings(skip)]
    friends: Vec<String>,
}

fn main() {
    let friends = vec!["luca".to_string()];
    let my_struct = MyStruct::from_env(friends.clone()).unwrap();
    assert_eq!(my_struct.name, "paolo".to_string());
    assert_eq!(my_struct.favourite_number, 42);
    assert_eq!(my_struct.birth_date, "01/01/1970");
    assert_eq!(my_struct.birth_place, None);
    assert_eq!(my_struct.friends, friends);

    let name = "luca";
    let my_struct = MyStruct::new(
        Some(name.to_string()),
        None,
        None,
        Some("london".to_string()),
        friends.clone(),
    ).unwrap();
    assert_eq!(my_struct.name, name);
    assert_eq!(my_struct.favourite_number, 42);
    assert_eq!(my_struct.birth_date, "01/01/1970");
    assert_eq!(my_struct.birth_place, Some("london".to_string()));
    assert_eq!(my_struct.friends, friends);
}
```

### Parameters

#### Struct

The current supported parameters for the structs are:

-   `case_insensitive`: whether the environment variables matching should be case insensitive. By default, matching is case sensitive.
-   `delay`: whether to delay the lookup for environment variables from compilation time to run time. By default the lookup is performed at compilation time
-   `file_path`: the file path to read to add some environment variables (e.g. `.env`). By default, it is not set
-   `prefix`: the prefix to add to the name of the struct fields before matching the environment variables. By default, it is not set

#### Field

The current supported parameters for the fields are:

-   `default`: the default value to use if the environment variable is not found. By default, it is not set
-   `skip`: whether to skip the parsing of the environment variable. It is necessary if the type specified does not implement `std::str::FromStr`.
-   `variable`: the environment variable to use for the lookup. By default, the name of the field

### Variables resolution hierarchy

1. Arguments passed to the `new` method (if using `new`).
2. Environment variables
3. Variables loaded from a file (e.g. `.env`)
4. Default values

## Contribute

Before starting to work on a contribution please read:

-   [Code of Conduct](https://github.com/dariocurr/.github/blob/main/.github/CODE_OF_CONDUCT.md)
-   [Contributing](https://github.com/dariocurr/.github/blob/main/.github/CONTRIBUTING.md)
-   [Goverance](https://github.com/dariocurr/.github/blob/main/.github/GOVERNANCE.md)
-   [Security](https://github.com/dariocurr/.github/blob/main/.github/SECURITY.md)
-   [Support](https://github.com/dariocurr/.github/blob/main/.github/SUPPORT.md)

### Run tests

When testing run:

```shell
cargo test -- --test-threads=1
```

to prevent tests from competitively interacting with the same file

## License

This project is licensed under the terms of the MIT or Apache 2.0 license.
