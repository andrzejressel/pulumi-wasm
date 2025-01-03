#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetSiteLocation {
    /// Address of the location.
    #[builder(into)]
    #[serde(rename = "address")]
    pub r#address: Box<String>,
    /// Latitude of the location.
    #[builder(into)]
    #[serde(rename = "latitude")]
    pub r#latitude: Box<String>,
    /// Longitude of the location.
    #[builder(into)]
    #[serde(rename = "longitude")]
    pub r#longitude: Box<String>,
}
