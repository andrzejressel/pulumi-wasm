#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ResourceSetResourceDnsTargetResource {
    /// DNS Name that acts as the ingress point to a portion of application.
    #[builder(into)]
    #[serde(rename = "domainName")]
    pub r#domain_name: Box<String>,
    /// Hosted Zone ARN that contains the DNS record with the provided name of target resource.
    #[builder(into, default)]
    #[serde(rename = "hostedZoneArn")]
    pub r#hosted_zone_arn: Box<Option<String>>,
    /// Route53 record set id to uniquely identify a record given a `domain_name` and a `record_type`.
    #[builder(into, default)]
    #[serde(rename = "recordSetId")]
    pub r#record_set_id: Box<Option<String>>,
    /// Type of DNS Record of target resource.
    #[builder(into, default)]
    #[serde(rename = "recordType")]
    pub r#record_type: Box<Option<String>>,
    /// Target resource the R53 record specified with the above params points to.
    #[builder(into, default)]
    #[serde(rename = "targetResource")]
    pub r#target_resource: Box<Option<super::super::types::route53recoveryreadiness::ResourceSetResourceDnsTargetResourceTargetResource>>,
}
