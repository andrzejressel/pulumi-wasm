#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
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
