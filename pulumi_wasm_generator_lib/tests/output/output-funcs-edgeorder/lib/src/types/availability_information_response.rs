//! Availability information of a product system.

#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
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
