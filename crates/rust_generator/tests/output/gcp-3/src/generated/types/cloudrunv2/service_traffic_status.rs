#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ServiceTrafficStatus {
    /// (Output)
    /// Specifies percent of the traffic to this Revision.
    #[builder(into, default)]
    #[serde(rename = "percent")]
    pub r#percent: Box<Option<i32>>,
    /// (Output)
    /// Revision to which this traffic is sent.
    #[builder(into, default)]
    #[serde(rename = "revision")]
    pub r#revision: Box<Option<String>>,
    /// (Output)
    /// Indicates the string used in the URI to exclusively reference this target.
    #[builder(into, default)]
    #[serde(rename = "tag")]
    pub r#tag: Box<Option<String>>,
    /// (Output)
    /// The allocation type for this traffic target.
    #[builder(into, default)]
    #[serde(rename = "type")]
    pub r#type_: Box<Option<String>>,
    /// (Output)
    /// Displays the target URI.
    #[builder(into, default)]
    #[serde(rename = "uri")]
    pub r#uri: Box<Option<String>>,
}
