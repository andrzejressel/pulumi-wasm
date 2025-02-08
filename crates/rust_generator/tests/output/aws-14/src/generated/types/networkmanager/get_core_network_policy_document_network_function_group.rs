#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetCoreNetworkPolicyDocumentNetworkFunctionGroup {
    /// Optional description of the network function group.
    #[builder(into, default)]
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    /// This identifies the network function group container.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// This will be either `true`, that attachment acceptance is required, or `false`, that it is not required.
    #[builder(into)]
    #[serde(rename = "requireAttachmentAcceptance")]
    pub r#require_attachment_acceptance: Box<bool>,
}
