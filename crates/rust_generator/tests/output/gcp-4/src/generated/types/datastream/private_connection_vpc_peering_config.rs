#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct PrivateConnectionVpcPeeringConfig {
    /// A free subnet for peering. (CIDR of /29)
    /// 
    /// - - -
    #[builder(into)]
    #[serde(rename = "subnet")]
    pub r#subnet: Box<String>,
    /// Fully qualified name of the VPC that Datastream will peer to.
    /// Format: projects/{project}/global/{networks}/{name}
    #[builder(into)]
    #[serde(rename = "vpc")]
    pub r#vpc: Box<String>,
}
