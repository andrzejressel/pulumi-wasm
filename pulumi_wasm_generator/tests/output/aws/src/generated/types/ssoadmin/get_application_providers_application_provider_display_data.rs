#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetApplicationProvidersApplicationProviderDisplayData {
    /// Description of the application provider.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Box<String>,
    /// Name of the application provider.
    #[builder(into)]
    #[serde(rename = "displayName")]
    pub r#display_name: Box<String>,
    /// URL that points to an icon that represents the application provider.
    #[builder(into)]
    #[serde(rename = "iconUrl")]
    pub r#icon_url: Box<String>,
}