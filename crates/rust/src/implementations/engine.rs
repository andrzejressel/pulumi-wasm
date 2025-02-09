use serde::de::DeserializeOwned;
use serde::Serialize;

pub trait GestaltEngine {
    type Output<T>;

    fn new<T>(&self, value: String, secret: bool) -> Self::Output<T>;
}

pub trait GestaltOutput<T> {
    type Me<A>;
    fn map<B, F>(&self, f: F) -> Self::Me<B>
    where
        F: Fn(T) -> B + Send + 'static,
        T: DeserializeOwned,
        B: Serialize;
}

pub struct GestaltEngineOwned {
    #[cfg(target_arch = "wasm32")]
    pub engine: super::wasm_engine::WasmEngine,
    #[cfg(not(target_arch = "wasm32"))]
    pub engine: super::native_engine::NativeEngine,
}
