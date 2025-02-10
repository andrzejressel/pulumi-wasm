pub mod functions {}
pub mod types {
    include!("types/acyclic_referent.rs");
    include!("types/acyclic_s.rs");
    include!("types/acyclic_t.rs");
    include!("types/direct_cycle.rs");
    include!("types/indirect_cycle_s.rs");
    include!("types/indirect_cycle_t.rs");
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
