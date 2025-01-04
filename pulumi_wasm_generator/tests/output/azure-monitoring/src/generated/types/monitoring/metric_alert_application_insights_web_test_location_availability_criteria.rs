#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct MetricAlertApplicationInsightsWebTestLocationAvailabilityCriteria {
    /// The ID of the Application Insights Resource.
    #[builder(into)]
    #[serde(rename = "componentId")]
    pub r#component_id: Box<String>,
    /// The number of failed locations.
    #[builder(into)]
    #[serde(rename = "failedLocationCount")]
    pub r#failed_location_count: Box<i32>,
    /// The ID of the Application Insights Web Test.
    #[builder(into)]
    #[serde(rename = "webTestId")]
    pub r#web_test_id: Box<String>,
}
