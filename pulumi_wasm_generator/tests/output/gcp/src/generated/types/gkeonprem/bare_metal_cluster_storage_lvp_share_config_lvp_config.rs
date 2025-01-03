#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct BareMetalClusterStorageLvpShareConfigLvpConfig {
    /// The host machine path.
    #[builder(into)]
    #[serde(rename = "path")]
    pub r#path: Box<String>,
    /// The StorageClass name that PVs will be created with.
    #[builder(into)]
    #[serde(rename = "storageClass")]
    pub r#storage_class: Box<String>,
}
