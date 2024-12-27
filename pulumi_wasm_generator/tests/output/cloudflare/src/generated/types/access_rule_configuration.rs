#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AccessRuleConfiguration {
    /// The request property to target. Available values: `ip`, `ip6`, `ip_range`, `asn`, `country`. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    #[serde(rename = "target")]
    pub r#target: Box<String>,
    /// The value to target. Depends on target's type. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Box<String>,
}
