pub mod tree {
    pub mod v1 {
        include!("resources/tree/v1/nursery.rs");
        include!("resources/tree/v1/rubber_tree.rs");
    }
}
pub mod functions {}
pub mod types {
    pub mod tree {
        pub mod v1 {
            include!("types/tree/v1/diameter.rs");
            include!("types/tree/v1/farm.rs");
            include!("types/tree/v1/rubber_tree_variety.rs");
            include!("types/tree/v1/tree_size.rs");
        }
    }
    include!("types/cloud_audit_options_log_name.rs");
    include!("types/container.rs");
    include!("types/container_brightness.rs");
    include!("types/container_color.rs");
    include!("types/container_size.rs");
}
#[doc(hidden)]
pub mod constants {}
#[link_section = "pulumi_gestalt_provider::plant"]
#[no_mangle]
#[cfg(target_arch = "wasm32")]
static PULUMI_WASM_PROVIDER_PLANT: [u8; 44] = *b"{\"version\":\"0.0.1\",\"pluginDownloadURL\":null}";
pub(crate) fn get_version() -> String {
    "0.0.1".to_string()
}
