#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetServiceTemplateSpecContainerStartupProbeGrpc {
    /// Port number to access on the container. Number must be in the range 1 to 65535.
    /// If not specified, defaults to the same value as container.ports[0].containerPort.
    #[builder(into)]
    #[serde(rename = "port")]
    pub r#port: Box<i32>,
    /// The name of the service to place in the gRPC HealthCheckRequest
    /// (see https://github.com/grpc/grpc/blob/master/doc/health-checking.md).
    /// If this is not specified, the default behavior is defined by gRPC.
    #[builder(into)]
    #[serde(rename = "service")]
    pub r#service: Box<String>,
}
