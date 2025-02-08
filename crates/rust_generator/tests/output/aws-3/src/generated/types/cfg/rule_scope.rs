#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct RuleScope {
    /// The IDs of the only AWS resource that you want to trigger an evaluation for the rule. If you specify a resource ID, you must specify one resource type for `compliance_resource_types`.
    #[builder(into, default)]
    #[serde(rename = "complianceResourceId")]
    pub r#compliance_resource_id: Box<Option<String>>,
    /// A list of resource types of only those AWS resources that you want to trigger an evaluation for the ruleE.g., `AWS::EC2::Instance`. You can only specify one type if you also specify a resource ID for `compliance_resource_id`. See [relevant part of AWS Docs](http://docs.aws.amazon.com/config/latest/APIReference/API_ResourceIdentifier.html#config-Type-ResourceIdentifier-resourceType) for available types.
    #[builder(into, default)]
    #[serde(rename = "complianceResourceTypes")]
    pub r#compliance_resource_types: Box<Option<Vec<String>>>,
    /// The tag key that is applied to only those AWS resources that you want you want to trigger an evaluation for the rule.
    #[builder(into, default)]
    #[serde(rename = "tagKey")]
    pub r#tag_key: Box<Option<String>>,
    /// The tag value applied to only those AWS resources that you want to trigger an evaluation for the rule.
    #[builder(into, default)]
    #[serde(rename = "tagValue")]
    pub r#tag_value: Box<Option<String>>,
}
