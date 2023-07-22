# Env Settings

**Env Settings** is a Rust library that helps you to initialize structs using environment variables

> This Rust library took inspiration from [`pydantic's BaseSettings`](https://docs.pydantic.dev/latest/usage/pydantic_settings/) Python class

## Installation

```sh
cargo add env-settings
cargo add env-settings-derive
```

## Usage

When you add the `EnvSettings` derive to a `struct`, two methods are added to it

-   ```rs
    fn from_env() -> env_settings_utils::EnvSettingsResult<Self>
    ```

    It creates a new instance using just the environment variables. If something fails, it returns an `env_settings_utils::EnvSettingsError` error

-   ```rs
    fn new(...) -> env_settings_utils::EnvSettingsResult<Self>
    ```

    It creates a new instance using a combination of environment variables and parameters. More in detail, every field value can be passed as parameter wrapped in an `Option` object. Then if the parameter is `Some`, it is used, otherwise the value is recoved from the environment variables. If something fails, it returns an `env_settings_utils::EnvSettingsError` error

### Basic

```sh
export name=paolo
export age=42
```

```rs
use env_settings_derive::EnvSettings;

#[derive(EnvSettings)]
struct MyStruct {
    name: String,
    age: u8,
}

fn main() {
    let my_struct = MyStruct::from_env().unwrap();
    assert_eq!(my_struct.name, "paolo".to_string());
    assert_eq!(my_struct.age, 42);

    let name = "luca";
    let my_struct = MyStruct::new(Some(name.to_string()), None).unwrap();
    assert_eq!(my_struct.name, name);
    assert_eq!(my_struct.age, 42);
}
```

### Advanced

```sh
export STRUCT_NAME=paolo
echo "STRUCT_AGE=42\n" > .env
```

```rs
use env_settings_derive::EnvSettings;

#[derive(EnvSettings)]
#[env_settings(case_insensitive, file_path = ".env", prefix="STRUCT_")]
struct MyStruct {
    name: String,
    age: u8,
}

fn main() {
    let my_struct = MyStruct::from_env().unwrap();
    assert_eq!(my_struct.name, "paolo".to_string());
    assert_eq!(my_struct.age, 42);

    let name = "luca";
    let my_struct = MyStruct::new(Some(name.to_string()), None).unwrap();
    assert_eq!(my_struct.name, name);
    assert_eq!(my_struct.age, 42);
}
```

### Parameters

The current supported parameters are:

-   `case_insensitive`: add it if the environment variables matching should be case insensitive
-   `delay`: add it to delay the lookup for environment variables from compilation time to run time
-   `file_path`: add it to specify a file path to read to add some environment variables (e.g. `.env`)
-   `prefix`: add it to specify a prefix to add to the name of the struct fields before matching the environment variables

### Variables resolution hierarchy

1. Arguments passed to the `new` method (if using `new`).
2. Environment variables
3. Variables loaded from a file (e.g. `.env`)

## Contribute

Before starting to work on a conribution please read:

-   [Code of Conduct](https://github.com/dariocurr/.github/blob/main/.github/CODE_OF_CONDUCT.md)
-   [Contributing](https://github.com/dariocurr/.github/blob/main/.github/CONTRIBUTING.md)
-   [Goverance](https://github.com/dariocurr/.github/blob/main/.github/GOVERNANCE.md)
-   [Security](https://github.com/dariocurr/.github/blob/main/.github/SECURITY.md)
-   [Support](https://github.com/dariocurr/.github/blob/main/.github/SUPPORT.md)

### Run tests

When testing run:

```sh
cargo test -- --test-threads=1
```

to prevent tests from competitively interacting with the same file

## License

This project is licensed under the terms of the MIT or Apache 2.0 license.
