#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetLaunchTemplatePlacement {
    #[builder(into)]
    #[serde(rename = "affinity")]
    pub r#affinity: Box<String>,
    #[builder(into)]
    #[serde(rename = "availabilityZone")]
    pub r#availability_zone: Box<String>,
    #[builder(into)]
    #[serde(rename = "groupName")]
    pub r#group_name: Box<String>,
    #[builder(into)]
    #[serde(rename = "hostId")]
    pub r#host_id: Box<String>,
    #[builder(into)]
    #[serde(rename = "hostResourceGroupArn")]
    pub r#host_resource_group_arn: Box<String>,
    #[builder(into)]
    #[serde(rename = "partitionNumber")]
    pub r#partition_number: Box<i32>,
    #[builder(into)]
    #[serde(rename = "spreadDomain")]
    pub r#spread_domain: Box<String>,
    #[builder(into)]
    #[serde(rename = "tenancy")]
    pub r#tenancy: Box<String>,
}
