pub mod deeply {
    pub mod nested {
        pub mod module {
            include!("resources/deeply/nested/module/resource.rs");
        }
    }
}
pub mod functions {}
pub mod types {}
#[doc(hidden)]
pub mod constants {}
#[link_section = "pulumi_gestalt_provider::foo-bar"]
#[no_mangle]
#[cfg(target_arch = "wasm32")]
static PULUMI_WASM_PROVIDER_FOO_BAR: [u8; 44] = *b"{\"version\":\"0.0.1\",\"pluginDownloadURL\":null}";
pub(crate) fn get_version() -> String {
    "0.0.1".to_string()
}
