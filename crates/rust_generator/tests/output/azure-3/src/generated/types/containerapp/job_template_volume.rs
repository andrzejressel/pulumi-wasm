#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct JobTemplateVolume {
    /// The name of the volume.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The name of the storage to use for the volume.
    #[builder(into, default)]
    #[serde(rename = "storageName")]
    pub r#storage_name: Box<Option<String>>,
    /// The type of storage to use for the volume. Possible values are `AzureFile`, `EmptyDir` and `Secret`.
    #[builder(into, default)]
    #[serde(rename = "storageType")]
    pub r#storage_type: Box<Option<String>>,
}
