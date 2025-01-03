#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ThingGroupProperties {
    /// The Thing Group attributes. Defined below.
    #[builder(into, default)]
    #[serde(rename = "attributePayload")]
    pub r#attribute_payload: Box<Option<super::super::types::iot::ThingGroupPropertiesAttributePayload>>,
    /// A description of the Thing Group.
    #[builder(into, default)]
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
}
