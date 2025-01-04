#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetReceivedLicenseConsumptionConfigurationBorrowConfiguration {
    /// Indicates whether early check-ins are allowed.
    #[builder(into)]
    #[serde(rename = "allowEarlyCheckIn")]
    pub r#allow_early_check_in: Box<bool>,
    /// Maximum time for the provisional configuration, in minutes.
    #[builder(into)]
    #[serde(rename = "maxTimeToLiveInMinutes")]
    pub r#max_time_to_live_in_minutes: Box<i32>,
}
