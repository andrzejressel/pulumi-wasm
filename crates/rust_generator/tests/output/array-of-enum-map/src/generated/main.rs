include!("resources/example_server.rs");
pub mod functions {}
pub mod types {
    include!("types/annotation_store_schema_value_type.rs");
}
#[doc(hidden)]
pub mod constants {}
#[link_section = "pulumi_gestalt_provider::example"]
#[no_mangle]
#[cfg(target_arch = "wasm32")]
static PULUMI_WASM_PROVIDER_EXAMPLE: [u8; 44] = *b"{\"version\":\"1.0.0\",\"pluginDownloadURL\":null}";
pub(crate) fn get_version() -> String {
    "1.0.0".to_string()
}
