#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ServiceRegionVpcSettings {
    /// The identifiers of the subnets for the directory servers.
    #[builder(into)]
    #[serde(rename = "subnetIds")]
    pub r#subnet_ids: Box<Vec<String>>,
    /// The identifier of the VPC in which to create the directory.
    #[builder(into)]
    #[serde(rename = "vpcId")]
    pub r#vpc_id: Box<String>,
}
