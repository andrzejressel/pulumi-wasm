#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AvailabilityInformationResponse {
    /// Current availability stage of the product. Availability stage
    #[builder(into)]
    #[serde(rename = "availabilityStage")]
    pub r#availability_stage: Box<String>,
    /// Reason why the product is disabled.
    #[builder(into)]
    #[serde(rename = "disabledReason")]
    pub r#disabled_reason: Box<String>,
    /// Message for why the product is disabled.
    #[builder(into)]
    #[serde(rename = "disabledReasonMessage")]
    pub r#disabled_reason_message: Box<String>,
}
