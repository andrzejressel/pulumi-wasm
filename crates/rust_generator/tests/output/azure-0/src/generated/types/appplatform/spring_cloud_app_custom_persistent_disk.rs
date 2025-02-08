#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct SpringCloudAppCustomPersistentDisk {
    /// These are the mount options for a persistent disk.
    #[builder(into, default)]
    #[serde(rename = "mountOptions")]
    pub r#mount_options: Box<Option<Vec<String>>>,
    /// The mount path of the persistent disk.
    #[builder(into)]
    #[serde(rename = "mountPath")]
    pub r#mount_path: Box<String>,
    /// Indicates whether the persistent disk is a readOnly one.
    #[builder(into, default)]
    #[serde(rename = "readOnlyEnabled")]
    pub r#read_only_enabled: Box<Option<bool>>,
    /// The share name of the Azure File share.
    #[builder(into)]
    #[serde(rename = "shareName")]
    pub r#share_name: Box<String>,
    /// The name of the Spring Cloud Storage.
    #[builder(into)]
    #[serde(rename = "storageName")]
    pub r#storage_name: Box<String>,
}
