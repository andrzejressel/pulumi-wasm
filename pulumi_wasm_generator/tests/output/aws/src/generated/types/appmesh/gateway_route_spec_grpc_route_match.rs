#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GatewayRouteSpecGrpcRouteMatch {
    /// The port number to match from the request.
    #[builder(into, default)]
    #[serde(rename = "port")]
    pub r#port: Box<Option<i32>>,
    /// Fully qualified domain name for the service to match from the request.
    #[builder(into)]
    #[serde(rename = "serviceName")]
    pub r#service_name: Box<String>,
}
