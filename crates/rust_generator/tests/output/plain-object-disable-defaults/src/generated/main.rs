include!("resources/foo.rs");
include!("resources/module_test.rs");
pub mod functions {
    include!("functions/func_with_all_optional_inputs.rs");
}
pub mod types {
    pub mod mod1 {
        include!("types/mod1/typ.rs");
    }
    pub mod mod2 {
        include!("types/mod2/typ.rs");
    }
    include!("types/helm_release_settings.rs");
    include!("types/kube_client_settings.rs");
    include!("types/layered_type.rs");
    include!("types/typ.rs");
}
#[doc(hidden)]
pub mod constants {}
#[link_section = "pulumi_gestalt_provider::example"]
#[no_mangle]
#[cfg(target_arch = "wasm32")]
static PULUMI_WASM_PROVIDER_EXAMPLE: [u8; 44] = *b"{\"version\":\"0.0.1\",\"pluginDownloadURL\":null}";
pub(crate) fn get_version() -> String {
    "0.0.1".to_string()
}
