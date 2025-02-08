#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AccessBoundaryPolicyRuleAccessBoundaryRule {
    /// The availability condition further constrains the access allowed by the access boundary rule.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "availabilityCondition")]
    pub r#availability_condition: Box<Option<super::super::types::iam::AccessBoundaryPolicyRuleAccessBoundaryRuleAvailabilityCondition>>,
    /// A list of permissions that may be allowed for use on the specified resource.
    #[builder(into, default)]
    #[serde(rename = "availablePermissions")]
    pub r#available_permissions: Box<Option<Vec<String>>>,
    /// The full resource name of a Google Cloud resource entity.
    #[builder(into, default)]
    #[serde(rename = "availableResource")]
    pub r#available_resource: Box<Option<String>>,
}
