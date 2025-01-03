#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GeoMatchSetGeoMatchConstraint {
    /// The type of geographical area you want AWS WAF to search for. Currently Country is the only valid value.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
    /// The country that you want AWS WAF to search for.
    /// This is the two-letter country code, e.g., `US`, `CA`, `RU`, `CN`, etc.
    /// See [docs](https://docs.aws.amazon.com/waf/latest/APIReference/API_GeoMatchConstraint.html) for all supported values.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Box<String>,
}
