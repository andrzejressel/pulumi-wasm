#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AppDapr {
    /// The Dapr Application Identifier.
    #[builder(into)]
    #[serde(rename = "appId")]
    pub r#app_id: Box<String>,
    /// The port which the application is listening on. This is the same as the `ingress` port.
    #[builder(into, default)]
    #[serde(rename = "appPort")]
    pub r#app_port: Box<Option<i32>>,
    /// The protocol for the app. Possible values include `http` and `grpc`. Defaults to `http`.
    #[builder(into, default)]
    #[serde(rename = "appProtocol")]
    pub r#app_protocol: Box<Option<String>>,
}
