#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetFrameworkControlScope {
    /// The ID of the only AWS resource that you want your control scope to contain.
    #[builder(into)]
    #[serde(rename = "complianceResourceIds")]
    pub r#compliance_resource_ids: Box<Vec<String>>,
    /// Describes whether the control scope includes one or more types of resources, such as EFS or RDS.
    #[builder(into)]
    #[serde(rename = "complianceResourceTypes")]
    pub r#compliance_resource_types: Box<Vec<String>>,
    /// Tag key-value pair applied to those AWS resources that you want to trigger an evaluation for a rule. A maximum of one key-value pair can be provided.
    #[builder(into)]
    #[serde(rename = "tags")]
    pub r#tags: Box<std::collections::HashMap<String, String>>,
}
