#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct BucketReplicationConfigRuleFilter {
    /// Configuration block for specifying rule filters. This element is required only if you specify more than one filter. See and below for more details.
    #[builder(into, default)]
    #[serde(rename = "and")]
    pub r#and: Box<Option<super::super::types::s3::BucketReplicationConfigRuleFilterAnd>>,
    /// Object key name prefix that identifies subset of objects to which the rule applies. Must be less than or equal to 1024 characters in length.
    #[builder(into, default)]
    #[serde(rename = "prefix")]
    pub r#prefix: Box<Option<String>>,
    /// Configuration block for specifying a tag key and value. See below.
    #[builder(into, default)]
    #[serde(rename = "tag")]
    pub r#tag: Box<Option<super::super::types::s3::BucketReplicationConfigRuleFilterTag>>,
}
