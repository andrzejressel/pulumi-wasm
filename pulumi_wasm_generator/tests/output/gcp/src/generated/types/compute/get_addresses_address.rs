#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetAddressesAddress {
    /// The IP address (for example `1.2.3.4`).
    #[builder(into)]
    #[serde(rename = "address")]
    pub r#address: Box<String>,
    /// The IP address type, can be `EXTERNAL` or `INTERNAL`.
    #[builder(into)]
    #[serde(rename = "addressType")]
    pub r#address_type: Box<String>,
    /// The IP address description.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Box<String>,
    /// A map containing IP labels.
    #[builder(into)]
    #[serde(rename = "labels")]
    pub r#labels: Box<std::collections::HashMap<String, String>>,
    /// The IP address name.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Region that should be considered to search addresses.
    /// All regions are considered if missing.
    #[builder(into)]
    #[serde(rename = "region")]
    pub r#region: Box<String>,
    /// The URI of the created resource.
    #[builder(into)]
    #[serde(rename = "selfLink")]
    pub r#self_link: Box<String>,
    /// Indicates if the address is used. Possible values are: RESERVED or IN_USE.
    #[builder(into)]
    #[serde(rename = "status")]
    pub r#status: Box<String>,
}
