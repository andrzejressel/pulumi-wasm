#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GatewayLocationData {
    /// The city or locality where the resource is located.
    #[builder(into, default)]
    #[serde(rename = "city")]
    pub r#city: Box<Option<String>>,
    /// The district, state, or province where the resource is located.
    #[builder(into, default)]
    #[serde(rename = "district")]
    pub r#district: Box<Option<String>>,
    /// A canonical name for the geographic or physical location.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The country or region where the resource is located.
    #[builder(into, default)]
    #[serde(rename = "region")]
    pub r#region: Box<Option<String>>,
}
