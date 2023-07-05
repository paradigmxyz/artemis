use anyhow::{Error, Result};
use clap::{Parser, ValueHint};
use convert_case::{Case, Casing};
use quote::__private::TokenStream;
use std::fs::{create_dir, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::init::{generate_constants, generate_lib, generate_strategy, generate_types};

const SRC_DIR: &str = "src";
const LIB_FILE: &str = "lib.rs";
const CONSTANTS_FILE: &str = "constants.rs";
const STRATEGY_FILE: &str = "strategy.rs";
const TYPES_FILE: &str = "types.rs";
const CARGO_FILE: &str = "Cargo.toml";

#[derive(Debug, Clone, Parser)]
pub struct StrategyParser {
    /// The root directory of the new project.
    #[clap(value_hint = ValueHint::DirPath, default_value = "crates/strategies/")]
    root: PathBuf,

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

fn generate_crate<P: AsRef<Path>>(
    crate_name: &str,
    root: P,
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

    let path: PathBuf = {
        let mut p: PathBuf = root.as_ref().to_path_buf();
        p.push(crate_name);
        p
    };

    // Create crate directory
    create_dir(&path)?;

    // Create src directory
    let src_dir = path.join(SRC_DIR);
    create_dir(&src_dir)?;

    // Generate lib.rs file
    let lib_file = src_dir.join(LIB_FILE);
    let lib_code = lib.to_string();
    write_to_file(lib_file, &lib_code)?;

    // Generate constants.rs file
    let constants_file = src_dir.join(CONSTANTS_FILE);
    let constants_code = constants.to_string();
    write_to_file(constants_file, &constants_code)?;

    // Generate strategy.rs file
    let strategy_file = src_dir.join(STRATEGY_FILE);
    let strategy_code = strategy.to_string();
    write_to_file(strategy_file, &strategy_code)?;

    // Generate types.rs file
    let types_file = src_dir.join(TYPES_FILE);
    let types_code = types.to_string();
    write_to_file(types_file, &types_code)?;

    // Generate Cargo.toml file
    let cargo_toml_file = path.join(CARGO_FILE);
    let cargo_toml_code = generate_cargo_toml_code(crate_name, &dependencies);
    write_to_file(cargo_toml_file, &cargo_toml_code)?;

    // Format the generated code using rustfmt
    format_code(path)?;

    Ok(())
}

fn write_to_file<P: AsRef<Path>>(file_path: P, content: &str) -> std::io::Result<()> {
    let mut file = File::create(file_path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

fn format_code<P: AsRef<Path>>(crate_name: P) -> Result<(), Error> {
    let output = Command::new("cargo")
        .arg("fmt")
        .current_dir(crate_name.as_ref().as_os_str())
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
