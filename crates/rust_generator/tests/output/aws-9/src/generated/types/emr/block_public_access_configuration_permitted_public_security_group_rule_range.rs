#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct BlockPublicAccessConfigurationPermittedPublicSecurityGroupRuleRange {
    /// The final port in the range of TCP ports.
    #[builder(into)]
    #[serde(rename = "maxRange")]
    pub r#max_range: Box<i32>,
    /// The first port in the range of TCP ports.
    #[builder(into)]
    #[serde(rename = "minRange")]
    pub r#min_range: Box<i32>,
}
