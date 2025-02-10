pub mod functions {}
pub mod types {
    pub mod config {
        include!("types/config/ignore_tags.rs");
    }
    include!("types/region.rs");
}
#[doc(hidden)]
pub mod constants {}
#[link_section = "pulumi_gestalt_provider::aws-native"]
#[no_mangle]
#[cfg(target_arch = "wasm32")]
static PULUMI_WASM_PROVIDER_AWS_NATIVE: [u8; 44] = *b"{\"version\":\"0.0.1\",\"pluginDownloadURL\":null}";
pub(crate) fn get_version() -> String {
    "0.0.1".to_string()
}
