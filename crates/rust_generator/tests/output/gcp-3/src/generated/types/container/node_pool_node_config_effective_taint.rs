#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct NodePoolNodeConfigEffectiveTaint {
    /// Effect for taint.
    #[builder(into, default)]
    #[serde(rename = "effect")]
    pub r#effect: Box<Option<String>>,
    /// Key for taint.
    #[builder(into, default)]
    #[serde(rename = "key")]
    pub r#key: Box<Option<String>>,
    /// Value for taint.
    #[builder(into, default)]
    #[serde(rename = "value")]
    pub r#value: Box<Option<String>>,
}
