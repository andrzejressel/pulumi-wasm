#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AccessOrganizationCustomPage {
    /// The id of the forbidden page.
    #[builder(into, default)]
    #[serde(rename = "forbidden")]
    pub r#forbidden: Box<Option<String>>,
    /// The id of the identity denied page.
    #[builder(into, default)]
    #[serde(rename = "identityDenied")]
    pub r#identity_denied: Box<Option<String>>,
}
