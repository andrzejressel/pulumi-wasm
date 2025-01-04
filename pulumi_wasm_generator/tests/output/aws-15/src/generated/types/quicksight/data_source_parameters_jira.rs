#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DataSourceParametersJira {
    /// The base URL of the Jira instance's site to which to connect.
    #[builder(into)]
    #[serde(rename = "siteBaseUrl")]
    pub r#site_base_url: Box<String>,
}
