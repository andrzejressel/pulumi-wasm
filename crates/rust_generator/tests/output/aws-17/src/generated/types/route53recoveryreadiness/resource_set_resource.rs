#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ResourceSetResource {
    #[builder(into, default)]
    #[serde(rename = "componentId")]
    pub r#component_id: Box<Option<String>>,
    /// Component for DNS/Routing Control Readiness Checks.
    #[builder(into, default)]
    #[serde(rename = "dnsTargetResource")]
    pub r#dns_target_resource: Box<Option<super::super::types::route53recoveryreadiness::ResourceSetResourceDnsTargetResource>>,
    /// Recovery group ARN or cell ARN that contains this resource set.
    #[builder(into, default)]
    #[serde(rename = "readinessScopes")]
    pub r#readiness_scopes: Box<Option<Vec<String>>>,
    /// ARN of the resource.
    #[builder(into, default)]
    #[serde(rename = "resourceArn")]
    pub r#resource_arn: Box<Option<String>>,
}
