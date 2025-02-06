#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct CxTestCaseTestConfig {
    /// Flow name to start the test case with.
    /// Format: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/flows/<Flow ID>.
    /// Only one of flow and page should be set to indicate the starting point of the test case. If neither is set, the test case will start with start page on the default start flow.
    #[builder(into, default)]
    #[serde(rename = "flow")]
    pub r#flow: Box<Option<String>>,
    /// The page to start the test case with.
    /// Format: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/flows/<Flow ID>/pages/<Page ID>.
    /// Only one of flow and page should be set to indicate the starting point of the test case. If neither is set, the test case will start with start page on the default start flow.
    #[builder(into, default)]
    #[serde(rename = "page")]
    pub r#page: Box<Option<String>>,
    /// Session parameters to be compared when calculating differences.
    #[builder(into, default)]
    #[serde(rename = "trackingParameters")]
    pub r#tracking_parameters: Box<Option<Vec<String>>>,
}
