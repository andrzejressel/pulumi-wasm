pub mod documentdb {
    include!("resources/documentdb/sql_resource_sql_container.rs");
}
pub mod functions {}
pub mod types {
    pub mod documentdb {
        include!("types/documentdb/composite_path_response.rs");
        include!("types/documentdb/indexing_policy_response.rs");
        include!("types/documentdb/sql_container_get_properties_response_resource.rs");
    }
}
#[doc(hidden)]
pub mod constants {}
#[link_section = "pulumi_gestalt_provider::azure-native"]
#[no_mangle]
#[cfg(target_arch = "wasm32")]
static PULUMI_WASM_PROVIDER_AZURE_NATIVE: [u8; 44] = *b"{\"version\":\"0.0.1\",\"pluginDownloadURL\":null}";
pub(crate) fn get_version() -> String {
    "0.0.1".to_string()
}
