#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct SiteLocation {
    /// Address of the location.
    #[builder(into, default)]
    #[serde(rename = "address")]
    pub r#address: Box<Option<String>>,
    /// Latitude of the location.
    #[builder(into, default)]
    #[serde(rename = "latitude")]
    pub r#latitude: Box<Option<String>>,
    /// Longitude of the location.
    #[builder(into, default)]
    #[serde(rename = "longitude")]
    pub r#longitude: Box<Option<String>>,
}