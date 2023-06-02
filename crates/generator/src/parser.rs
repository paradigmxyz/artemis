use anyhow::{Error, Result};
use clap::{Parser, ValueHint};
use convert_case::{Case, Casing};
use quote::__private::TokenStream;
use std::ffi::OsString;
use std::fs::{create_dir, File};
use std::io::Write;
use std::process::Command;

use crate::init::{generate_constants, generate_lib, generate_strategy, generate_types};

#[derive(Debug, Clone, Parser)]
pub struct StrategyParser {
    /// The root directory of the new project.
    #[clap(value_hint = ValueHint::DirPath, default_value = "crates/strategies/")]
    root: OsString,

    #[clap(long, short)]
    strategy_name: String,
}

impl StrategyParser {
    pub fn generate(&self) -> Result<(), Error> {
        let snake = self.strategy_name.to_case(Case::Snake);
        let pascal = self.strategy_name.to_case(Case::Pascal);

        let strategy_data = generate_strategy(&pascal);
        let constants_data = generate_constants();
        let type_data = generate_types();
        let lib_data = generate_lib();

        let crate_name = snake;

        generate_crate(
            &crate_name,
            &self.root,
            strategy_data,
            constants_data,
            type_data,
            lib_data,
        )?;

        Ok(())
    }
}

fn generate_crate(
    crate_name: &str,
    root: &OsString,
    strategy: TokenStream,
    constants: TokenStream,
    types: TokenStream,
    lib: TokenStream,
) -> Result<(), Error> {
    // Crate name and dependencies
    let dependencies = vec![
        ("anyhow", "1.0"),
        ("ethers", "2"),
        ("async-trait", "0.1.64"),
    ];

    let path = root.as_os_str();

    let path_and_crate = path.to_str().unwrap().to_owned() + crate_name;

    // Create crate directory
    create_dir(&path_and_crate)?;

    // Create src directory
    let src_dir = format!("{}/src", path_and_crate);
    create_dir(src_dir)?;

    // Generate lib.rs file
    let lib_file = format!("{}/src/lib.rs", path_and_crate);
    let lib_code = lib.to_string();
    write_to_file(&lib_file, &lib_code)?;

    // Generate constants.rs file
    let constants_file = format!("{}/src/constants.rs", path_and_crate);
    let constants_code = constants.to_string();
    write_to_file(&constants_file, &constants_code)?;

    // Generate strategy.rs file
    let strategy_file = format!("{}/src/strategy.rs", path_and_crate);
    let strategy_code = strategy.to_string();
    write_to_file(&strategy_file, &strategy_code)?;

    // Generate types.rs file
    let types_file = format!("{}/src/types.rs", path_and_crate);
    let types_code = types.to_string();
    write_to_file(&types_file, &types_code)?;

    // Generate Cargo.toml file
    let cargo_toml_file = format!("{}/Cargo.toml", path_and_crate);
    let cargo_toml_code = generate_cargo_toml_code(crate_name, &dependencies);
    write_to_file(&cargo_toml_file, &cargo_toml_code)?;

    // Format the generated code using rustfmt
    format_code(&path_and_crate)?;

    Ok(())
}

fn write_to_file(file_path: &str, content: &str) -> std::io::Result<()> {
    let mut file = File::create(file_path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

fn format_code(crate_name: &str) -> Result<(), Error> {
    let output = Command::new("cargo")
        .arg("fmt")
        .current_dir(crate_name)
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("rustfmt error:\n{}", stderr);
    }

    Ok(())
}

fn generate_cargo_toml_code(crate_name: &str, dependencies: &[(&str, &str)]) -> String {
    let dependencies_str: String = dependencies
        .iter()
        .map(|(name, version)| format!("{} = \"{}\"", name, version))
        .collect::<Vec<String>>()
        .join("\n");

    format!(
        r#"
[package]
name = "{}"
version = "0.1.0"
edition = "2021"

[dependencies]
{}
{}
"#,
        crate_name, dependencies_str, r#"artemis-core = { path = "../../artemis-core" }"#,
    )
}
