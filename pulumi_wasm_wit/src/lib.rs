#[allow(clippy::all)]
#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_unsafe)]
#[cfg(feature = "client")]
pub mod client_bindings {
    wit_bindgen::generate!({
        world: "client",
        with: {
            "component:pulumi-wasm-external/pulumi-main@0.0.0-STABLE-DEV": generate
        }
    });
}

#[allow(clippy::all)]
#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_unsafe)]
#[cfg(feature = "pulumi_wasm")]
pub mod pulumi_wasm_bindings {
    wit_bindgen::generate!({
        world: "pulumi-wasm",
        with: {
            "component:pulumi-wasm-external/log@0.0.0-STABLE-DEV": generate,
            "component:pulumi-wasm-external/external-world@0.0.0-STABLE-DEV": generate,
            "component:pulumi-wasm-external/pulumi-settings@0.0.0-STABLE-DEV": generate
        }
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
        with: {
            "component:pulumi-wasm-external/log@0.0.0-STABLE-DEV": generate
        }
    });
}

#[allow(clippy::all)]
#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_unsafe)]
#[cfg(feature = "runner")]
pub mod bindings_runner {
    wasmtime::component::bindgen!({
        world: "runner",
        async: true,
        trappable_imports: true,
    });
}
