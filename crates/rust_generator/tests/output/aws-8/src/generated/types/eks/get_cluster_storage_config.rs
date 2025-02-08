#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetClusterStorageConfig {
    /// Contains block storage configuration for EKS Auto Mode enabled cluster.
    #[builder(into)]
    #[serde(rename = "blockStorages")]
    pub r#block_storages: Box<Vec<super::super::types::eks::GetClusterStorageConfigBlockStorage>>,
}
