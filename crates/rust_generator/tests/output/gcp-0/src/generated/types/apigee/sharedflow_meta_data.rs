#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct SharedflowMetaData {
    /// Time at which the API proxy was created, in milliseconds since epoch.
    #[builder(into, default)]
    #[serde(rename = "createdAt")]
    pub r#created_at: Box<Option<String>>,
    /// Time at which the API proxy was most recently modified, in milliseconds since epoch.
    #[builder(into, default)]
    #[serde(rename = "lastModifiedAt")]
    pub r#last_modified_at: Box<Option<String>>,
    /// The type of entity described
    #[builder(into, default)]
    #[serde(rename = "subType")]
    pub r#sub_type: Box<Option<String>>,
}
