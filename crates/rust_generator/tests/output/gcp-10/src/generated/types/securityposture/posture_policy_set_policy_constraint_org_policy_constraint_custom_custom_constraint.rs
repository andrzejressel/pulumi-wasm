#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PosturePolicySetPolicyConstraintOrgPolicyConstraintCustomCustomConstraint {
    /// The action to take if the condition is met.
    /// Possible values are: `ALLOW`, `DENY`.
    #[builder(into)]
    #[serde(rename = "actionType")]
    pub r#action_type: Box<String>,
    /// A CEL condition that refers to a supported service resource, for example `resource.management.autoUpgrade == false`. For details about CEL usage, see [Common Expression Language](https://cloud.google.com/resource-manager/docs/organization-policy/creating-managing-custom-constraints#common_expression_language).
    #[builder(into)]
    #[serde(rename = "condition")]
    pub r#condition: Box<String>,
    /// A human-friendly description of the constraint to display as an error message when the policy is violated.
    #[builder(into, default)]
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    /// A human-friendly name for the constraint.
    #[builder(into, default)]
    #[serde(rename = "displayName")]
    pub r#display_name: Box<Option<String>>,
    /// A list of RESTful methods for which to enforce the constraint. Can be `CREATE`, `UPDATE`, or both. Not all Google Cloud services support both methods. To see supported methods for each service, find the service in [Supported services](https://cloud.google.com/resource-manager/docs/organization-policy/custom-constraint-supported-services).
    #[builder(into)]
    #[serde(rename = "methodTypes")]
    pub r#method_types: Box<Vec<String>>,
    /// Immutable. The name of the custom constraint. This is unique within the organization.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Immutable. The fully qualified name of the Google Cloud REST resource containing the object and field you want to restrict. For example, `container.googleapis.com/NodePool`.
    #[builder(into)]
    #[serde(rename = "resourceTypes")]
    pub r#resource_types: Box<Vec<String>>,
}
