#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DashboardDashboardPublishOptionsDataPointDrillUpDownOption {
    /// Availability status. Possibles values: ENABLED, DISABLED.
    #[builder(into, default)]
    #[serde(rename = "availabilityStatus")]
    pub r#availability_status: Box<Option<String>>,
}
