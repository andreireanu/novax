use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use rust_format::{Formatter, RustFmt};
use crate::abi::result::Abi;
use crate::errors::build_error::BuildError;
use crate::generator::generated_file::GeneratedFile;
use crate::generator::generator_error::GeneratorError;
use crate::generator::impl_abi_types::impl_abi_types_mod;
use crate::generator::impl_contract::impl_contract;

pub fn generate_from_abi(abi: &Abi) -> Result<GeneratedFile, BuildError> {
    let Ok(formatted_content) = RustFmt::default().format_str(impl_mod(abi)?.to_string()) else {
        return Err(GeneratorError::UnableToFormatRustCode.into())
    };

    Ok(
        GeneratedFile {
            file_name: abi.get_mod_name() + ".rs",
            mod_name: abi.get_mod_name(),
            file_content: formatted_content
        }
    )
}

fn impl_mod(abi: &Abi) -> Result<TokenStream, BuildError> {
    let mod_name = abi.get_mod_name();
    let mod_name_ident = format_ident!("{}", mod_name);
    let imports = get_mod_imports();
    let types = impl_abi_types_mod(&abi.types)?;
    let contract = impl_contract(&mod_name, abi)?;

    Ok(
        quote! {
            /// This module, `#mod_name_ident`, encapsulates the logic, types, and implementations
            /// generated from the ABI (Application Binary Interface) for interacting with a specific
            /// smart contract on the blockchain. It provides a convenient and type-safe way to
            /// deploy, call, and query the smart contract.
            ///
            /// The module contains several key components:
            ///
            /// - **Contract Struct**: A struct representing the smart contract. It has methods for deploying
            ///   the contract, making calls to the contract, and querying the contract's state.
            /// - **Call and Query Structs**: Structs that encapsulate the logic for making calls to, and
            ///   querying the state of, the smart contract. They provide methods for configuring parameters
            ///   like gas limit, EGLD value, and ESDT transfers.
            /// - **Type Definitions**: Structs and enums representing the types defined in the smart contract's
            ///   ABI, with conversions between native Rust types and the managed types used in the smart contract.
            /// - **Generated Functions for Calls and Queries**: Functions generated from the ABI for each
            ///   endpoint and view of the smart contract, providing a type-safe interface for interacting with
            ///   the contract.
            ///
            /// This module is generated by a macro based on the smart contract's ABI, ensuring that the
            /// Rust interface stays in sync with the contract's actual interface.
            ///
            /// # Usage
            ///
            /// Typically, you would import this module and use the provided structs and functions to interact
            /// with the smart contract from your Rust code.
            ///
            /// # Note
            ///
            /// The `#![allow(clippy::all)]` directive at the beginning of the module suppresses linting warnings
            /// from Clippy for the generated code, as the code's structure is determined by the ABI and may not
            /// always conform to common Rust idioms.
            pub mod #mod_name_ident {
                #![allow(clippy::all)]

                #imports
                #contract
                #types
            }
        }
    )
}

fn get_mod_imports() -> TokenStream {
   quote! {
       #![allow(unused_imports)]

        multiversx_sc::imports!();
        multiversx_sc::derive_imports!();
        use novax_data::NativeConvertible;
        use multiversx_sc_scenario::api::StaticApi;
        use multiversx_sdk::data::vm::VmValueRequest;
        use serde::{Deserialize, Serialize};
        use crate::errors::NovaXError;
        use crate::caching::CachingStrategy;
        use crate::caching::CachingNone;
        use std::collections::hash_map::DefaultHasher;
        use std::hash::Hash;
        use std::hash::Hasher;
        use multiversx_sdk::wallet::Wallet;
        use multiversx_sc_codec::Empty;
        use multiversx_sc_scenario::ContractInfo;
        use multiversx_sc_snippets::Interactor;
        use crate::transaction::TokenTransfer;
        use multiversx_sc_scenario::scenario_model::ScCallStep;
        use multiversx_sc_scenario::scenario_model::TypedScCall;
        use multiversx_sc_scenario::scenario_model::TxExpect;
        use multiversx_sc_scenario::scenario_model::AddressKey;
        use multiversx_sc::api::VMApi;
        use multiversx_sc_scenario::DebugApi;
        use crate::transaction::CallResult;
        use std::sync::Arc;
        use tokio::sync::Mutex;
        use multiversx_sc_scenario::scenario_model::TxResponse;
        use multiversx_sc_scenario::scenario_model::ScQueryStep;
        use novax_executor::TransactionExecutor;
        use novax_executor::QueryExecutor;
        use novax_data::ManagedConvertible;
        use multiversx_sc_scenario::scenario_model::ScDeployStep;
        use crate::code::AsBytesValue;
        use novax_executor::DeployExecutor;
        use crate::code::DeployData;
        use std::ops::Deref;
        use multiversx_sc_scenario::scenario_model::AddressValue;
        use novax_data::Address;
   }
}