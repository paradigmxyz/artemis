//! init command

use anyhow::Error;
use clap::{Parser, ValueHint};
use quote::__private::TokenStream;
use quote::quote;
use std::{
    fs::{self, File},
    path::PathBuf,
};

use std::process::Command;

use std::io::Write;
/// CLI arguments for `forge init`.
#[derive(Debug, Clone, Parser)]
pub struct InitArgs {
    /// The root directory of the new project.
    #[clap(value_hint = ValueHint::DirPath, default_value = "crates/strategies", value_name = "PATH")]
    root: PathBuf,

    #[clap(long, short)]
    strategy_name: String,

    /// Create the project even if the specified root directory is not empty.
    #[clap(long)]
    force: bool,
}

const IMPORTS: &str = "
use async_trait::async_trait;
use std::sync::Arc;

use anyhow::Result;
use artemis_core::types::Strategy;
use ethers::providers::Middleware;

use super::types::{Action, Config, Event};

";

// Generate struct names using quote
fn generate_strategy_base(name: &str) -> TokenStream {
    let struct_name = quote::format_ident!("{}", name);
    let generic_type = quote::format_ident!("M");

    quote! {
        pub struct #struct_name<#generic_type> {
            client: Arc<#generic_type>,
        }

        impl<#generic_type: Middleware + 'static> #struct_name<#generic_type> {
            pub fn new(client: Arc<#generic_type>, config: Config) -> Self {
                Self { client }
            }
        }

        #[async_trait]
        impl<#generic_type: Middleware + 'static> Strategy<Event, Action> for #struct_name<#generic_type> {
            async fn sync_state(&mut self) -> Result<()> {
                Ok(())
            }

            async fn process_event(&mut self, event: Event) -> Option<Action> {
                match event {}
            }
        }

        impl<#generic_type: Middleware + 'static> #struct_name<#generic_type> {
            // add struct strategy methods here
        }
    }
}

impl InitArgs {
    pub fn run(self) -> anyhow::Result<(), Error> {
        let InitArgs {
            root,
            force,
            strategy_name,
        } = self;

        let lower = strategy_name.to_lowercase();

        // Format the generated code using rustfmt
        let pp = Command::new("cargo init")
            .arg("crates/strategies/".to_owned() + &lower)
            .output()?;

        let ts = generate_strategy_base(&strategy_name);

        let strat = IMPORTS.to_owned() + &ts.to_string();

        let mut file = File::create("src/generated_code.rs").expect("Failed to create file");

        file.write_all(strat.as_bytes())
            .expect("Failed to write to file");

        // Format the generated code using rustfmt
        let output = Command::new("rustfmt")
            .arg("src/generated_code.rs")
            .output()
            .expect("Failed to run rustfmt");

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            eprintln!("rustfmt error:\n{}", stderr);
        }

        println!("{}", ts);

        // create the root dir if it does not exist
        if !root.exists() {
            fs::create_dir_all(&root)?;
        }

        let root = dunce::canonicalize(root)?;

        // if target is not empty
        if root
            .read_dir()
            .map(|mut i| i.next().is_some())
            .unwrap_or(false)
        {
            if !force {
                anyhow::bail!(
                    "Cannot run `init` on a non-empty directory.\n\
                        Run with the `--force` flag to initialize regardless."
                );
            }
        }

        // make the dirs
        let src = root.join("src");
        fs::create_dir_all(&src)?;

        // write the types file
        let types_path = src.join("types.rs");
        fs::write(types_path, include_str!("../../assets/types.rs"))?;

        let strategy_path = src.join("strategy.rs");

        let template = fs::read_to_string("../../assets/strategy.rs")?;

        for mut line in template.lines() {
            let new_line = line.replace("StrategyName", &strategy_name);
            line = &new_line;
            println!("{}", line);
        }

        fs::write(strategy_path, template)?;

        Ok(())
    }
}
