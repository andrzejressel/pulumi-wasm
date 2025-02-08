#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetResourcesResourceTagMappingList {
    /// List of objects with information that shows whether a resource is compliant with the effective tag policy, including details on any noncompliant tag keys.
    #[builder(into)]
    #[serde(rename = "complianceDetails")]
    pub r#compliance_details: Box<Vec<super::super::types::resourcegroupstaggingapi::GetResourcesResourceTagMappingListComplianceDetail>>,
    /// ARN of the resource.
    #[builder(into)]
    #[serde(rename = "resourceArn")]
    pub r#resource_arn: Box<String>,
    /// Map of tags assigned to the resource.
    #[builder(into)]
    #[serde(rename = "tags")]
    pub r#tags: Box<std::collections::HashMap<String, String>>,
}
