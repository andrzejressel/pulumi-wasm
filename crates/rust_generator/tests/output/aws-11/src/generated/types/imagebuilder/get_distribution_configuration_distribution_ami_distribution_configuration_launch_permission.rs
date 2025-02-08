#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetDistributionConfigurationDistributionAmiDistributionConfigurationLaunchPermission {
    /// Set of AWS Organization ARNs.
    #[builder(into)]
    #[serde(rename = "organizationArns")]
    pub r#organization_arns: Box<Vec<String>>,
    /// Set of AWS Organizational Unit ARNs.
    #[builder(into)]
    #[serde(rename = "organizationalUnitArns")]
    pub r#organizational_unit_arns: Box<Vec<String>>,
    /// Set of EC2 launch permission user groups.
    #[builder(into)]
    #[serde(rename = "userGroups")]
    pub r#user_groups: Box<Vec<String>>,
    /// Set of AWS Account identifiers.
    #[builder(into)]
    #[serde(rename = "userIds")]
    pub r#user_ids: Box<Vec<String>>,
}
