#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct BucketLifecycleConfigurationV2RuleFilter {
    /// Configuration block used to apply a logical `AND` to two or more predicates. See below. The Lifecycle Rule will apply to any object matching all the predicates configured inside the `and` block.
    #[builder(into, default)]
    #[serde(rename = "and")]
    pub r#and: Box<Option<super::super::types::s3::BucketLifecycleConfigurationV2RuleFilterAnd>>,
    /// Minimum object size (in bytes) to which the rule applies.
    #[builder(into, default)]
    #[serde(rename = "objectSizeGreaterThan")]
    pub r#object_size_greater_than: Box<Option<String>>,
    /// Maximum object size (in bytes) to which the rule applies.
    #[builder(into, default)]
    #[serde(rename = "objectSizeLessThan")]
    pub r#object_size_less_than: Box<Option<String>>,
    /// Prefix identifying one or more objects to which the rule applies. Defaults to an empty string (`""`) if not specified.
    #[builder(into, default)]
    #[serde(rename = "prefix")]
    pub r#prefix: Box<Option<String>>,
    /// Configuration block for specifying a tag key and value. See below.
    #[builder(into, default)]
    #[serde(rename = "tag")]
    pub r#tag: Box<Option<super::super::types::s3::BucketLifecycleConfigurationV2RuleFilterTag>>,
}
