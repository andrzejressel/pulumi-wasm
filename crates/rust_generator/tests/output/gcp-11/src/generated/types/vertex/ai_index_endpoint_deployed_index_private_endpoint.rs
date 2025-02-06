#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AiIndexEndpointDeployedIndexPrivateEndpoint {
    /// (Output)
    /// The ip address used to send match gRPC requests.
    #[builder(into, default)]
    #[serde(rename = "matchGrpcAddress")]
    pub r#match_grpc_address: Box<Option<String>>,
    /// (Output)
    /// PscAutomatedEndpoints is populated if private service connect is enabled if PscAutomatedConfig is set.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "pscAutomatedEndpoints")]
    pub r#psc_automated_endpoints: Box<Option<Vec<super::super::types::vertex::AiIndexEndpointDeployedIndexPrivateEndpointPscAutomatedEndpoint>>>,
    /// (Output)
    /// The name of the service attachment resource. Populated if private service connect is enabled.
    #[builder(into, default)]
    #[serde(rename = "serviceAttachment")]
    pub r#service_attachment: Box<Option<String>>,
}
