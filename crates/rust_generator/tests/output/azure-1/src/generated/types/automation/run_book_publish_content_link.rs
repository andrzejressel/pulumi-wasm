#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RunBookPublishContentLink {
    /// A `hash` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "hash")]
    pub r#hash: Box<Option<super::super::types::automation::RunBookPublishContentLinkHash>>,
    /// The URI of the runbook content.
    #[builder(into)]
    #[serde(rename = "uri")]
    pub r#uri: Box<String>,
    /// Specifies the version of the content
    #[builder(into, default)]
    #[serde(rename = "version")]
    pub r#version: Box<Option<String>>,
}
