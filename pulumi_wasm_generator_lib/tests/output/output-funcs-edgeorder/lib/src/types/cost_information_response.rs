//! Cost information for the product system

#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct CostInformationResponse {
    /// Default url to display billing information
    #[builder(into)]
    #[serde(rename = "billingInfoUrl")]
    pub r#billing_info_url: Box<String>,
    /// Details on the various billing aspects for the product system.
    #[builder(into)]
    #[serde(rename = "billingMeterDetails")]
    pub r#billing_meter_details: Box<Vec<crate::types::BillingMeterDetailsResponse>>,
}
