#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FieldTtlConfig {
    /// (Output)
    /// The state of TTL (time-to-live) configuration for documents that have this Field set.
    #[builder(into, default)]
    #[serde(rename = "state")]
    pub r#state: Box<Option<String>>,
}
