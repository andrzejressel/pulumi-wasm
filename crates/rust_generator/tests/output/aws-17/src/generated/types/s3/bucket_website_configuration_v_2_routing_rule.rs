#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct BucketWebsiteConfigurationV2RoutingRule {
    /// Configuration block for describing a condition that must be met for the specified redirect to apply. See below.
    #[builder(into, default)]
    #[serde(rename = "condition")]
    pub r#condition: Box<Option<super::super::types::s3::BucketWebsiteConfigurationV2RoutingRuleCondition>>,
    /// Configuration block for redirect information. See below.
    #[builder(into)]
    #[serde(rename = "redirect")]
    pub r#redirect: Box<super::super::types::s3::BucketWebsiteConfigurationV2RoutingRuleRedirect>,
}
