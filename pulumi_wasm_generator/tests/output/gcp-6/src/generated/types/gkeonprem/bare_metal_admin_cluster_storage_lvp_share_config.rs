#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct BareMetalAdminClusterStorageLvpShareConfig {
    /// Defines the machine path and storage class for the LVP Share.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "lvpConfig")]
    pub r#lvp_config: Box<super::super::types::gkeonprem::BareMetalAdminClusterStorageLvpShareConfigLvpConfig>,
    /// The number of subdirectories to create under path.
    #[builder(into, default)]
    #[serde(rename = "sharedPathPvCount")]
    pub r#shared_path_pv_count: Box<Option<i32>>,
}
