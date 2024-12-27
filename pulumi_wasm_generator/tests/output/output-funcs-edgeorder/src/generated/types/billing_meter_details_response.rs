#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct BillingMeterDetailsResponse {
    /// Frequency of recurrence
    #[builder(into)]
    #[serde(rename = "frequency")]
    pub r#frequency: Box<String>,
    /// Represents MeterDetails
    #[builder(into)]
    #[serde(rename = "meterDetails")]
    pub r#meter_details: Box<pulumi_wasm_rust::OneOf2<super::types::Pav2MeterDetailsResponse, super::types::PurchaseMeterDetailsResponse>>,
    /// Represents Metering type (eg one-time or recurrent)
    #[builder(into)]
    #[serde(rename = "meteringType")]
    pub r#metering_type: Box<String>,
    /// Represents Billing type name
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}
