#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FrameworkControlScope {
    /// The ID of the only AWS resource that you want your control scope to contain. Minimum number of 1 item. Maximum number of 100 items.
    #[builder(into, default)]
    #[serde(rename = "complianceResourceIds")]
    pub r#compliance_resource_ids: Box<Option<Vec<String>>>,
    /// Describes whether the control scope includes one or more types of resources, such as EFS or RDS.
    #[builder(into, default)]
    #[serde(rename = "complianceResourceTypes")]
    pub r#compliance_resource_types: Box<Option<Vec<String>>>,
    /// The tag key-value pair applied to those AWS resources that you want to trigger an evaluation for a rule. A maximum of one key-value pair can be provided.
    #[builder(into, default)]
    #[serde(rename = "tags")]
    pub r#tags: Box<Option<std::collections::HashMap<String, String>>>,
}
