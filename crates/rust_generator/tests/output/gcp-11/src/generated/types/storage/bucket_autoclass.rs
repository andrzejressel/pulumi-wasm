#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct BucketAutoclass {
    /// While set to `true`, autoclass automatically transitions objects in your bucket to appropriate storage classes based on each object's access pattern.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<bool>,
    /// The storage class that objects in the bucket eventually transition to if they are not read for a certain length of time. Supported values include: `NEARLINE`, `ARCHIVE`.
    #[builder(into, default)]
    #[serde(rename = "terminalStorageClass")]
    pub r#terminal_storage_class: Box<Option<String>>,
}
