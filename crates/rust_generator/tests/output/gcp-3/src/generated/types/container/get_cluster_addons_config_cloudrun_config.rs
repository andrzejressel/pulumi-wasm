#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetClusterAddonsConfigCloudrunConfig {
    #[builder(into)]
    #[serde(rename = "disabled")]
    pub r#disabled: Box<bool>,
    #[builder(into)]
    #[serde(rename = "loadBalancerType")]
    pub r#load_balancer_type: Box<String>,
}
