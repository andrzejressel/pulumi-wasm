#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct BucketLifecycleRule {
    /// The Lifecycle Rule's action configuration. A single block of this type is supported. Structure is documented below.
    #[builder(into)]
    #[serde(rename = "action")]
    pub r#action: Box<super::super::types::storage::BucketLifecycleRuleAction>,
    /// The Lifecycle Rule's condition configuration. A single block of this type is supported. Structure is documented below.
    #[builder(into)]
    #[serde(rename = "condition")]
    pub r#condition: Box<super::super::types::storage::BucketLifecycleRuleCondition>,
}
