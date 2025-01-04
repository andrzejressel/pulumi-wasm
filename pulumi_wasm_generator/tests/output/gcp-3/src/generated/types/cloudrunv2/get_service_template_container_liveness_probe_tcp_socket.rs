#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetServiceTemplateContainerLivenessProbeTcpSocket {
    /// Port number to access on the container. Must be in the range 1 to 65535.
    /// If not specified, defaults to the exposed port of the container, which
    /// is the value of container.ports[0].containerPort.
    #[builder(into)]
    #[serde(rename = "port")]
    pub r#port: Box<i32>,
}
