#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ConnectorKafkaClusterApacheKafkaClusterVpc {
    /// The security groups for the connector.
    #[builder(into)]
    #[serde(rename = "securityGroups")]
    pub r#security_groups: Box<Vec<String>>,
    /// The subnets for the connector.
    #[builder(into)]
    #[serde(rename = "subnets")]
    pub r#subnets: Box<Vec<String>>,
}
