#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct EndpointGeoFilter {
    /// The Action of the Geo Filter. Possible values include `Allow` and `Block`.
    #[builder(into)]
    #[serde(rename = "action")]
    pub r#action: Box<String>,
    /// A List of two letter country codes (e.g. `US`, `GB`) to be associated with this Geo Filter.
    #[builder(into)]
    #[serde(rename = "countryCodes")]
    pub r#country_codes: Box<Vec<String>>,
    /// The relative path applicable to geo filter.
    #[builder(into)]
    #[serde(rename = "relativePath")]
    pub r#relative_path: Box<String>,
}