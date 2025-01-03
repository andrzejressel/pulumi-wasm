#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetServiceTraffic {
    /// Specifies percent of the traffic to this Revision. This defaults to zero if unspecified.
    #[builder(into)]
    #[serde(rename = "percent")]
    pub r#percent: Box<i32>,
    /// Revision to which to send this portion of traffic, if traffic allocation is by revision.
    #[builder(into)]
    #[serde(rename = "revision")]
    pub r#revision: Box<String>,
    /// Indicates a string to be part of the URI to exclusively reference this target.
    #[builder(into)]
    #[serde(rename = "tag")]
    pub r#tag: Box<String>,
    /// The allocation type for this traffic target. Possible values: ["TRAFFIC_TARGET_ALLOCATION_TYPE_LATEST", "TRAFFIC_TARGET_ALLOCATION_TYPE_REVISION"]
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type: Box<String>,
}
