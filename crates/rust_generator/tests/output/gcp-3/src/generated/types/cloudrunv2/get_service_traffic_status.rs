#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetServiceTrafficStatus {
    /// Specifies percent of the traffic to this Revision.
    #[builder(into)]
    #[serde(rename = "percent")]
    pub r#percent: Box<i32>,
    /// Revision to which this traffic is sent.
    #[builder(into)]
    #[serde(rename = "revision")]
    pub r#revision: Box<String>,
    /// Indicates the string used in the URI to exclusively reference this target.
    #[builder(into)]
    #[serde(rename = "tag")]
    pub r#tag: Box<String>,
    /// The allocation type for this traffic target.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
    /// Displays the target URI.
    #[builder(into)]
    #[serde(rename = "uri")]
    pub r#uri: Box<String>,
}
