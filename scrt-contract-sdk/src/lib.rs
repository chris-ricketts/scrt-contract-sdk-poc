pub mod contract;
pub mod error;
pub mod handle;
pub mod storage;

pub mod prelude {
    pub use crate::contract::Contract;
    pub use crate::error::StdErrorExt;
    pub use crate::handle::HandleResultExt;
    pub use crate::hookup_contract;
    pub use crate::storage::{
        address_store::{
            ExecuteLoadWithStorage, ExecuteSaveWithStorage, LoadForAddress, SaveForAddress,
        },
        DynamicKey, DynamicLoad, DynamicSave, StaticKey, StaticLoad, StaticSave, StaticUpdate,
        Store,
    };
    pub use cosmwasm_std::{
        Api, Env, Extern, HandleResult, InitResponse, Querier, QueryResult, StdError, StdResult,
        Storage,
    };
    pub use schemars::{self, JsonSchema};
    pub use serde::{self, Deserialize, Serialize};
}

pub use cosmwasm_std;
pub use secret_toolkit as toolkit;

#[macro_export]
macro_rules! hookup_contract {
    ($contract:ident) => {
        #[cfg(target_arch = "wasm32")]
        mod wasm {
            use super::$contract;
            use $crate::contract::Contract;
            use $crate::cosmwasm_std::{
                do_handle, do_init, do_query, ExternalApi, ExternalQuerier, ExternalStorage,
            };

            fn hookup_contract_init<C: Contract>(env_ptr: u32, msg_ptr: u32) -> u32 {
                do_init(
                    &C::init::<ExternalStorage, ExternalApi, ExternalQuerier>,
                    env_ptr,
                    msg_ptr,
                )
            }

            fn hookup_contract_handle<C: Contract>(env_ptr: u32, msg_ptr: u32) -> u32 {
                do_handle(
                    &C::handle::<ExternalStorage, ExternalApi, ExternalQuerier>,
                    env_ptr,
                    msg_ptr,
                )
            }

            fn hookup_contract_query<C: Contract>(msg_ptr: u32) -> u32 {
                do_query(
                    &C::_query::<ExternalStorage, ExternalApi, ExternalQuerier>,
                    msg_ptr,
                )
            }

            #[no_mangle]
            extern "C" fn init(env_ptr: u32, msg_ptr: u32) -> u32 {
                hookup_contract_init::<$contract>(env_ptr, msg_ptr)
            }

            #[no_mangle]
            extern "C" fn handle(env_ptr: u32, msg_ptr: u32) -> u32 {
                hookup_contract_handle::<$contract>(env_ptr, msg_ptr)
            }

            #[no_mangle]
            extern "C" fn query(msg_ptr: u32) -> u32 {
                hookup_contract_query::<$contract>(msg_ptr)
            }
        }
    };
}
