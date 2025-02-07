#[allow(clippy::all)]
#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_unsafe)]
#[cfg(feature = "client")]
pub mod client_bindings {
    wit_bindgen::generate!({
        world: "client",
        with: {
            "component:pulumi-gestalt-external/pulumi-main@0.0.0-STABLE-DEV": generate
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
            "component:pulumi-gestalt-external/log@0.0.0-STABLE-DEV": generate
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
