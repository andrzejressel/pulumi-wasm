#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct LifecyclePolicyPolicyDetailAction {
    /// Specifies the resources that the lifecycle policy applies to. Detailed below.
    #[builder(into, default)]
    #[serde(rename = "includeResources")]
    pub r#include_resources: Box<Option<super::super::types::imagebuilder::LifecyclePolicyPolicyDetailActionIncludeResources>>,
    /// Specifies the lifecycle action to take. Valid values: `DELETE`, `DEPRECATE` or `DISABLE`.
    /// 
    /// The following arguments are optional:
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
