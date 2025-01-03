#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetAppEngineServiceTelemetry {
    /// The full name of the resource that defines this service.
    /// Formatted as described in
    /// https://cloud.google.com/apis/design/resource_names.
    #[builder(into)]
    #[serde(rename = "resourceName")]
    pub r#resource_name: Box<String>,
}
