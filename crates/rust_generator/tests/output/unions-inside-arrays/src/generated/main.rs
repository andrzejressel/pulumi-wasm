include!("resources/example_server.rs");
pub mod functions {}
pub mod types {
    include!("types/server_properties_for_replica.rs");
    include!("types/server_properties_for_restore.rs");
}
#[doc(hidden)]
pub mod constants {
    pulumi_gestalt_rust::__private::constant::generate_string_const!(
        ConstStringPointInTimeRestore, "PointInTimeRestore"
    );
    pulumi_gestalt_rust::__private::constant::generate_string_const!(
        ConstStringReplica, "Replica"
    );
}
#[unsafe(link_section = "pulumi_gestalt_provider::example")]
#[unsafe(no_mangle)]
#[cfg(target_arch = "wasm32")]
static PULUMI_WASM_PROVIDER_EXAMPLE: [u8; 44] = *b"{\"version\":\"1.0.0\",\"pluginDownloadURL\":null}";
pub(crate) fn get_version() -> String {
    "1.0.0".to_string()
}
