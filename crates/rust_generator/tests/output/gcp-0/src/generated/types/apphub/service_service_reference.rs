#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ServiceServiceReference {
    /// (Output)
    /// Output only. The underlying resource URI (For example, URI of Forwarding Rule, URL Map,
    /// and Backend Service).
    #[builder(into, default)]
    #[serde(rename = "uri")]
    pub r#uri: Box<Option<String>>,
}
