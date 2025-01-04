#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ClusterServiceConnectDefaults {
    /// ARN of the `aws.servicediscovery.HttpNamespace` that's used when you create a service and don't specify a Service Connect configuration.
    #[builder(into)]
    #[serde(rename = "namespace")]
    pub r#namespace: Box<String>,
}
