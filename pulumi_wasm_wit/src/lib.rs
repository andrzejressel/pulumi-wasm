#[allow(clippy::all)]
#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_unsafe)]
#[cfg(feature = "pulumi-wasm-rust")]
pub mod bindings {
    wit_bindgen::generate!({
        world: "pulumi-wasm-rust",
    });
}

#[allow(clippy::all)]
#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_unsafe)]
#[cfg(feature = "logger")]
pub mod bindings_logger {
    wit_bindgen::generate!({
        world: "logger",
    });
}

#[allow(clippy::all)]
#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_unsafe)]
#[cfg(feature = "server")]
pub mod bindings_server {
    wasmtime::component::bindgen!({
        world: "main",
        async: true,
        trappable_imports: true,
    });
}
