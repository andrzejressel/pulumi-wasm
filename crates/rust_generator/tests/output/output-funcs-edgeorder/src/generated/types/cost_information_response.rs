#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct CostInformationResponse {
    /// Default url to display billing information
    #[builder(into)]
    #[serde(rename = "billingInfoUrl")]
    pub r#billing_info_url: Box<String>,
    /// Details on the various billing aspects for the product system.
    #[builder(into)]
    #[serde(rename = "billingMeterDetails")]
    pub r#billing_meter_details: Box<Vec<super::types::BillingMeterDetailsResponse>>,
}
