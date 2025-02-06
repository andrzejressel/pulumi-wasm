#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ApplicationInteractiveConfiguration {
    /// Enables an Apache Livy endpoint that you can connect to and run interactive jobs.
    #[builder(into, default)]
    #[serde(rename = "livyEndpointEnabled")]
    pub r#livy_endpoint_enabled: Box<Option<bool>>,
    /// Enables you to connect an application to Amazon EMR Studio to run interactive workloads in a notebook.
    #[builder(into, default)]
    #[serde(rename = "studioEnabled")]
    pub r#studio_enabled: Box<Option<bool>>,
}
