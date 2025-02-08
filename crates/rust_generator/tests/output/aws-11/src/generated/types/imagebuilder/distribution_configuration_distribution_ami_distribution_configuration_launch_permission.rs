#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct DistributionConfigurationDistributionAmiDistributionConfigurationLaunchPermission {
    /// Set of AWS Organization ARNs to assign.
    #[builder(into, default)]
    #[serde(rename = "organizationArns")]
    pub r#organization_arns: Box<Option<Vec<String>>>,
    /// Set of AWS Organizational Unit ARNs to assign.
    #[builder(into, default)]
    #[serde(rename = "organizationalUnitArns")]
    pub r#organizational_unit_arns: Box<Option<Vec<String>>>,
    /// Set of EC2 launch permission user groups to assign. Use `all` to distribute a public AMI.
    #[builder(into, default)]
    #[serde(rename = "userGroups")]
    pub r#user_groups: Box<Option<Vec<String>>>,
    /// Set of AWS Account identifiers to assign.
    #[builder(into, default)]
    #[serde(rename = "userIds")]
    pub r#user_ids: Box<Option<Vec<String>>>,
}
