#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RunCommandSource {
    #[builder(into, default)]
    #[serde(rename = "commandId")]
    pub r#command_id: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "script")]
    pub r#script: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "scriptUri")]
    pub r#script_uri: Box<Option<String>>,
    /// A `script_uri_managed_identity` block as defined above.
    #[builder(into, default)]
    #[serde(rename = "scriptUriManagedIdentity")]
    pub r#script_uri_managed_identity: Box<Option<super::super::types::compute::RunCommandSourceScriptUriManagedIdentity>>,
}
