#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DomainMappingStatusCondition {
    /// (Output)
    /// Human readable message indicating details about the current status.
    #[builder(into, default)]
    #[serde(rename = "message")]
    pub r#message: Box<Option<String>>,
    /// (Output)
    /// One-word CamelCase reason for the condition's current status.
    #[builder(into, default)]
    #[serde(rename = "reason")]
    pub r#reason: Box<Option<String>>,
    /// (Output)
    /// Status of the condition, one of True, False, Unknown.
    #[builder(into, default)]
    #[serde(rename = "status")]
    pub r#status: Box<Option<String>>,
    /// Resource record type. Example: `AAAA`.
    /// Possible values are: `A`, `AAAA`, `CNAME`.
    #[builder(into, default)]
    #[serde(rename = "type")]
    pub r#type_: Box<Option<String>>,
}
