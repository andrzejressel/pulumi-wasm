#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct TaskSparkInfrastructureSpecVpcNetwork {
    /// The Cloud VPC network in which the job is run. By default, the Cloud VPC network named Default within the project is used.
    #[builder(into, default)]
    #[serde(rename = "network")]
    pub r#network: Box<Option<String>>,
    /// List of network tags to apply to the job.
    #[builder(into, default)]
    #[serde(rename = "networkTags")]
    pub r#network_tags: Box<Option<Vec<String>>>,
    /// The Cloud VPC sub-network in which the job is run.
    #[builder(into, default)]
    #[serde(rename = "subNetwork")]
    pub r#sub_network: Box<Option<String>>,
}
