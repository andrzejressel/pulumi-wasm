#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RuleGroupRuleGroupReferenceSetsIpSetReferenceIpSetReference {
    /// Set of Managed Prefix IP ARN(s)
    #[builder(into)]
    #[serde(rename = "referenceArn")]
    pub r#reference_arn: Box<String>,
}
