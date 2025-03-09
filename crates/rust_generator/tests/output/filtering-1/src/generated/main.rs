pub mod ns2 {
    include!("resources/ns2/resource_2.rs");
}
pub mod functions {
    pub mod ns2 {
        include!("functions/ns2/function_2.rs");
    }
}
pub mod types {
    pub mod common {
        include!("types/common/common_type.rs");
    }
    pub mod ns2 {
        include!("types/ns2/type_2.rs");
    }
}
#[doc(hidden)]
pub mod constants {}
#[unsafe(link_section = "pulumi_gestalt_provider::example")]
#[unsafe(no_mangle)]
#[cfg(target_arch = "wasm32")]
static PULUMI_WASM_PROVIDER_EXAMPLE: [u8; 44] = *b"{\"version\":\"1.0.0\",\"pluginDownloadURL\":null}";
pub(crate) fn get_version() -> String {
    "1.0.0".to_string()
}
