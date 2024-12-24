#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct BillingMeterDetailsResponse {
    /// Frequency of recurrence
    #[builder(into)]
    #[serde(rename = "frequency")]
    pub r#frequency: Box<String>,
    /// Represents MeterDetails
    #[builder(into)]
    #[serde(rename = "meterDetails")]
    pub r#meter_details: Box<pulumi_wasm_provider_common::OneOf2<super::types::Pav2MeterDetailsResponse, super::types::PurchaseMeterDetailsResponse>>,
    /// Represents Metering type (eg one-time or recurrent)
    #[builder(into)]
    #[serde(rename = "meteringType")]
    pub r#metering_type: Box<String>,
    /// Represents Billing type name
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}
