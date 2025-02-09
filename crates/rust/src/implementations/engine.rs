pub trait GestaltEngine {
    type Output;

    fn new(&self, value: String, secret: bool) -> Self::Output;
}

pub struct GestaltEngineOwned {
    #[cfg(target_arch = "wasm32")]
    pub engine: super::wasm_engine::WasmEngine,
    #[cfg(not(target_arch = "wasm32"))]
    pub engine: super::native_engine::NativeEngine
}