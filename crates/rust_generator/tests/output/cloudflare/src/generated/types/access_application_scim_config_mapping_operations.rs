#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AccessApplicationScimConfigMappingOperations {
    /// Whether or not this mapping applies to create (POST) operations.
    #[builder(into, default)]
    #[serde(rename = "create")]
    pub r#create: Box<Option<bool>>,
    /// Whether or not this mapping applies to DELETE operations.
    #[builder(into, default)]
    #[serde(rename = "delete")]
    pub r#delete: Box<Option<bool>>,
    /// Whether or not this mapping applies to update (PATCH/PUT) operations.
    #[builder(into, default)]
    #[serde(rename = "update")]
    pub r#update: Box<Option<bool>>,
}
