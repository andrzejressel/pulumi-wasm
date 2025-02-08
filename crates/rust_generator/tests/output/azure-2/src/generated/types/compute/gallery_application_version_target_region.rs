#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GalleryApplicationVersionTargetRegion {
    /// Specifies whether this Gallery Application Version should be excluded from the `latest` filter. If set to `true`, this Gallery Application Version won't be returned for the `latest` version. Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "excludeFromLatest")]
    pub r#exclude_from_latest: Box<Option<bool>>,
    /// The Azure Region in which the Gallery Application Version exists.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The number of replicas of the Gallery Application Version to be created per region. Possible values are between `1` and `10`.
    #[builder(into)]
    #[serde(rename = "regionalReplicaCount")]
    pub r#regional_replica_count: Box<i32>,
    /// The storage account type for the Gallery Application Version. Possible values are `Standard_LRS`, `Premium_LRS` and `Standard_ZRS`. Defaults to `Standard_LRS`.
    #[builder(into, default)]
    #[serde(rename = "storageAccountType")]
    pub r#storage_account_type: Box<Option<String>>,
}
