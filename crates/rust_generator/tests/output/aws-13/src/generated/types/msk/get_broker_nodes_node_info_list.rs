#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetBrokerNodesNodeInfoList {
    /// Attached elastic network interface of the broker
    #[builder(into)]
    #[serde(rename = "attachedEniId")]
    pub r#attached_eni_id: Box<String>,
    /// ID of the broker
    #[builder(into)]
    #[serde(rename = "brokerId")]
    pub r#broker_id: Box<f64>,
    /// Client subnet to which this broker node belongs
    #[builder(into)]
    #[serde(rename = "clientSubnet")]
    pub r#client_subnet: Box<String>,
    /// The client virtual private cloud (VPC) IP address
    #[builder(into)]
    #[serde(rename = "clientVpcIpAddress")]
    pub r#client_vpc_ip_address: Box<String>,
    /// Set of endpoints for accessing the broker. This does not include ports
    #[builder(into)]
    #[serde(rename = "endpoints")]
    pub r#endpoints: Box<Vec<String>>,
    /// ARN of the node
    #[builder(into)]
    #[serde(rename = "nodeArn")]
    pub r#node_arn: Box<String>,
}
