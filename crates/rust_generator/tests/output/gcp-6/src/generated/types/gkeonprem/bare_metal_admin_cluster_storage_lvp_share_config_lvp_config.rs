#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct BareMetalAdminClusterStorageLvpShareConfigLvpConfig {
    /// The host machine path.
    #[builder(into)]
    #[serde(rename = "path")]
    pub r#path: Box<String>,
    /// The StorageClass name that PVs will be created with.
    #[builder(into)]
    #[serde(rename = "storageClass")]
    pub r#storage_class: Box<String>,
}
