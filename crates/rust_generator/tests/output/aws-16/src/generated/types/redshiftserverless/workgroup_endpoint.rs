#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct WorkgroupEndpoint {
    /// The DNS address of the VPC endpoint.
    #[builder(into, default)]
    #[serde(rename = "address")]
    pub r#address: Box<Option<String>>,
    /// The port number on which the cluster accepts incoming connections.
    #[builder(into, default)]
    #[serde(rename = "port")]
    pub r#port: Box<Option<i32>>,
    /// The VPC endpoint or the Redshift Serverless workgroup. See `VPC Endpoint` below.
    #[builder(into, default)]
    #[serde(rename = "vpcEndpoints")]
    pub r#vpc_endpoints: Box<Option<Vec<super::super::types::redshiftserverless::WorkgroupEndpointVpcEndpoint>>>,
}
