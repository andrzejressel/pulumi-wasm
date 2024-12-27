#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PagesProjectDeploymentConfigsProductionServiceBinding {
    /// The name of the Worker environment to bind to.
    #[builder(into, default)]
    #[serde(rename = "environment")]
    pub r#environment: Box<Option<String>>,
    /// The global variable for the binding in your Worker code.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The name of the Worker to bind to.
    #[builder(into)]
    #[serde(rename = "service")]
    pub r#service: Box<String>,
}
