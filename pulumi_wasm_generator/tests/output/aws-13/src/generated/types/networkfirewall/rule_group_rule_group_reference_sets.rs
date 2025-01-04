#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RuleGroupRuleGroupReferenceSets {
    #[builder(into, default)]
    #[serde(rename = "ipSetReferences")]
    pub r#ip_set_references: Box<Option<Vec<super::super::types::networkfirewall::RuleGroupRuleGroupReferenceSetsIpSetReference>>>,
}
