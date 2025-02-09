mod engine;

#[cfg(not(target_arch = "wasm32"))]
pub(crate) mod native_engine;

#[cfg(not(target_arch = "wasm32"))]
type OutputId = ();

#[cfg(target_arch = "wasm32")]
pub(crate) mod wasm_engine;

#[cfg(target_arch = "wasm32")]
type Id = u32;
#[cfg(target_arch = "wasm32")]
type Engine = wasm_engine::WasmEngine;
#[cfg(target_arch = "wasm32")]
type Output<T> = wasm_engine::WasmOutput<T>;
#[cfg(target_arch = "wasm32")]
type OutputId = pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::output_interface::Output;

#[cfg(not(target_arch = "wasm32"))]
type Id = uuid::Uuid;

#[doc(hidden)]
pub struct RegisterResourceRequest<'a> {
    pub type_: String,
    pub name: String,
    pub version: String,
    pub props: &'a[RegisterResourceRequestObjectField<'a>],
}

#[doc(hidden)]
pub struct RegisterResourceRequestObjectField<'a> {
    pub name: String,
    pub value: &'a OutputId
}