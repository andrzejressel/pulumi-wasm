#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetServiceTemplateSpecContainerPort {
    /// Port number the container listens on. This must be a valid port number (between 1 and 65535). Defaults to "8080".
    #[builder(into)]
    #[serde(rename = "containerPort")]
    pub r#container_port: Box<i32>,
    /// The name of the Cloud Run Service.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Protocol for port. Must be "TCP". Defaults to "TCP".
    #[builder(into)]
    #[serde(rename = "protocol")]
    pub r#protocol: Box<String>,
}
