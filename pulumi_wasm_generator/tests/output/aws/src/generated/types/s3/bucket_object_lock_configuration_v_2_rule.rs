#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct BucketObjectLockConfigurationV2Rule {
    /// Configuration block for specifying the default Object Lock retention settings for new objects placed in the specified bucket. See below.
    #[builder(into)]
    #[serde(rename = "defaultRetention")]
    pub r#default_retention: Box<super::super::types::s3::BucketObjectLockConfigurationV2RuleDefaultRetention>,
}