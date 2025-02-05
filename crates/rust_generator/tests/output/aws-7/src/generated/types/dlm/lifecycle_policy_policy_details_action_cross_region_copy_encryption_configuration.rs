#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct LifecyclePolicyPolicyDetailsActionCrossRegionCopyEncryptionConfiguration {
    #[builder(into, default)]
    #[serde(rename = "cmkArn")]
    pub r#cmk_arn: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "encrypted")]
    pub r#encrypted: Box<Option<bool>>,
}
