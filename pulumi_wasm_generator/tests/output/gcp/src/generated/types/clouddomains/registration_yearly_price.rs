#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RegistrationYearlyPrice {
    /// The three-letter currency code defined in ISO 4217.
    #[builder(into, default)]
    #[serde(rename = "currencyCode")]
    pub r#currency_code: Box<Option<String>>,
    /// The whole units of the amount. For example if currencyCode is "USD", then 1 unit is one US dollar.
    #[builder(into, default)]
    #[serde(rename = "units")]
    pub r#units: Box<Option<String>>,
}
