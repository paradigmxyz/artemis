use quote::__private::TokenStream;
use quote::quote;

// Generate struct that implements required traits
pub fn generate_strategy(name: &str) -> TokenStream {
    let struct_name = quote::format_ident!("{}", name);
    let generic_type = quote::format_ident!("M");

    quote! {
        use async_trait::async_trait;
        use std::sync::Arc;

        use anyhow::Result;
        use artemis_core::types::Strategy;
        use ethers::providers::Middleware;

        use super::types::{Action, Config, Event};

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
        }
    }
}

pub fn generate_types() -> TokenStream {
    let event_definition = quote! {
        #[derive(Debug, Clone)]
        pub enum Event {}
    };

    let action_definition = quote! {
        #[derive(Debug, Clone)]
        pub enum Action {}
    };

    let config_definition = quote! {
        #[derive(Debug, Clone)]
        pub struct Config {}
    };

    quote! {
        #event_definition
        #action_definition
        #config_definition
    }
}

// Generate lib file
pub fn generate_lib() -> TokenStream {
    quote! {
        pub mod strategy;
        pub mod types;
        pub mod constants;
    }
}

// Generate constants file
pub fn generate_constants() -> TokenStream {
    quote! {
        //add required constants here
    }
}
