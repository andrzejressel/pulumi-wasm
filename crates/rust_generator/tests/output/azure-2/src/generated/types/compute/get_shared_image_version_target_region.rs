#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetSharedImageVersionTargetRegion {
    /// The name of the Image Version.
    /// 
    /// > **Note:** You may specify `latest` to obtain the latest version or `recent` to obtain the most recently updated version.
    /// 
    /// > **Note:** In 3.0, `latest` may return an image version with `exclude_from_latest` set to `true`. Starting from 4.0 onwards `latest` will not return image versions with `exlude_from_latest` set to `true`.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The number of replicas of the Image Version to be created per region.
    #[builder(into)]
    #[serde(rename = "regionalReplicaCount")]
    pub r#regional_replica_count: Box<i32>,
    /// The storage account type for the image version.
    #[builder(into)]
    #[serde(rename = "storageAccountType")]
    pub r#storage_account_type: Box<String>,
}
