pub mod constant;
pub mod output;

pub extern crate bon;
#[cfg(target_arch = "wasm32")]
pub extern crate pulumi_gestalt_rust_adapter_wasm;
#[cfg(not(target_arch = "wasm32"))]
pub extern crate pulumi_gestalt_rust_adapter_native;
pub extern crate serde;
