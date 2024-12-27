#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AccessApplicationTargetCriteria {
    /// The port that the targets use for the chosen communication protocol. A port cannot be assigned to multiple protocols.
    #[builder(into)]
    #[serde(rename = "port")]
    pub r#port: Box<i32>,
    /// The communication protocol your application secures.
    #[builder(into)]
    #[serde(rename = "protocol")]
    pub r#protocol: Box<String>,
    /// Contains a map of target attribute keys to target attribute values.
    #[builder(into)]
    #[serde(rename = "targetAttributes")]
    pub r#target_attributes: Box<Vec<super::types::AccessApplicationTargetCriteriaTargetAttribute>>,
}
