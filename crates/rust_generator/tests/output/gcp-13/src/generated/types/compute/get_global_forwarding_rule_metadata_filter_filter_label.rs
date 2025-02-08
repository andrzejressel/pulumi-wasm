#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetGlobalForwardingRuleMetadataFilterFilterLabel {
    /// The name of the global forwarding rule.
    /// 
    /// - - -
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The value that the label must match. The value has a maximum
    /// length of 1024 characters.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Box<String>,
}
