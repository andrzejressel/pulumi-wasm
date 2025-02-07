#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct BlockchainNodesConnectionInfoEndpointInfo {
    /// (Output)
    /// The assigned URL for the node JSON-RPC API endpoint.
    #[builder(into, default)]
    #[serde(rename = "jsonRpcApiEndpoint")]
    pub r#json_rpc_api_endpoint: Box<Option<String>>,
    /// (Output)
    /// The assigned URL for the node WebSockets API endpoint.
    #[builder(into, default)]
    #[serde(rename = "websocketsApiEndpoint")]
    pub r#websockets_api_endpoint: Box<Option<String>>,
}
