include!("resources/pulumi_terraform_bridge_2801.rs");
pub mod functions {}
pub mod types {}
#[doc(hidden)]
pub mod constants {}
#[unsafe(link_section = "pulumi_gestalt_provider::example")]
#[unsafe(no_mangle)]
#[cfg(target_arch = "wasm32")]
static PULUMI_WASM_PROVIDER_EXAMPLE: [u8; 44] = *b"{\"version\":\"1.0.0\",\"pluginDownloadURL\":null}";
pub(crate) fn get_version() -> String {
    "1.0.0".to_string()
}
