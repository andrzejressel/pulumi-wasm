include!("resources/random_bytes.rs");
include!("resources/random_id.rs");
include!("resources/random_integer.rs");
include!("resources/random_password.rs");
include!("resources/random_pet.rs");
include!("resources/random_shuffle.rs");
include!("resources/random_string.rs");
include!("resources/random_uuid.rs");
pub mod functions {}
pub mod types {}
#[doc(hidden)]
pub mod constants {}
#[unsafe(link_section = "pulumi_gestalt_provider::random")]
#[unsafe(no_mangle)]
#[cfg(target_arch = "wasm32")]
static PULUMI_WASM_PROVIDER_RANDOM: [u8; 45] = *b"{\"version\":\"4.15.1\",\"pluginDownloadURL\":null}";
pub(crate) fn get_version() -> String {
    "4.15.1".to_string()
}
