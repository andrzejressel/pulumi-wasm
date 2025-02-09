mod engine;

#[cfg(not(target_arch = "wasm32"))]
pub(crate) mod native_engine;

#[cfg(target_arch = "wasm32")]
pub(crate) mod wasm_engine;

#[cfg(target_arch = "wasm32")]
type Id = u32;
#[cfg(target_arch = "wasm32")]
type Engine = wasm_engine::WasmEngine;

#[cfg(not(target_arch = "wasm32"))]
type Id = uuid::Uuid;
