#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FulfillmentFeature {
    /// The type of the feature that enabled for fulfillment.
    /// * SMALLTALK: Fulfillment is enabled for SmallTalk.
    /// Possible values are: `SMALLTALK`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type: Box<String>,
}
