#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetServiceTemplateVpcAccess {
    /// VPC Access connector name. Format: projects/{project}/locations/{location}/connectors/{connector}, where {project} can be project id or number.
    #[builder(into)]
    #[serde(rename = "connector")]
    pub r#connector: Box<String>,
    /// Traffic VPC egress settings. Possible values: ["ALL_TRAFFIC", "PRIVATE_RANGES_ONLY"]
    #[builder(into)]
    #[serde(rename = "egress")]
    pub r#egress: Box<String>,
    /// Direct VPC egress settings. Currently only single network interface is supported.
    #[builder(into)]
    #[serde(rename = "networkInterfaces")]
    pub r#network_interfaces: Box<Vec<super::super::types::cloudrunv2::GetServiceTemplateVpcAccessNetworkInterface>>,
}
