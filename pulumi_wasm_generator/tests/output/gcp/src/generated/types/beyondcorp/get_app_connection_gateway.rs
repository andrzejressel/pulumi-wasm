#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetAppConnectionGateway {
    /// AppGateway name in following format: projects/{project_id}/locations/{locationId}/appgateways/{gateway_id}.
    #[builder(into)]
    #[serde(rename = "appGateway")]
    pub r#app_gateway: Box<String>,
    /// Ingress port reserved on the gateways for this AppConnection, if not specified or zero, the default port is 19443.
    #[builder(into)]
    #[serde(rename = "ingressPort")]
    pub r#ingress_port: Box<i32>,
    /// The type of hosting used by the gateway. Refer to
    /// https://cloud.google.com/beyondcorp/docs/reference/rest/v1/projects.locations.appConnections#Type_1
    /// for a list of possible values.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type: Box<String>,
    /// Server-defined URI for this resource.
    #[builder(into)]
    #[serde(rename = "uri")]
    pub r#uri: Box<String>,
}
