#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DeploymentGroupEc2TagSet {
    /// Tag filters associated with the deployment group. See the AWS docs for details.
    #[builder(into, default)]
    #[serde(rename = "ec2TagFilters")]
    pub r#ec_2_tag_filters: Box<Option<Vec<super::super::types::codedeploy::DeploymentGroupEc2TagSetEc2TagFilter>>>,
}
