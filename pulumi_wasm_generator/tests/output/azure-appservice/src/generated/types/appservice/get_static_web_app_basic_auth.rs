#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetStaticWebAppBasicAuth {
    /// The Environment types which are configured to use Basic Auth access.
    #[builder(into)]
    #[serde(rename = "environments")]
    pub r#environments: Box<String>,
}