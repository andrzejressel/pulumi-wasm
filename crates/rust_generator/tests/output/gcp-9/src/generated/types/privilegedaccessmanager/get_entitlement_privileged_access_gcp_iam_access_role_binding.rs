#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetEntitlementPrivilegedAccessGcpIamAccessRoleBinding {
    /// The expression field of the IAM condition to be associated with the role. If specified, a user with an active grant for this entitlement would be able to access the resource only if this condition evaluates to true for their request.
    /// https://cloud.google.com/iam/docs/conditions-overview#attributes.
    #[builder(into)]
    #[serde(rename = "conditionExpression")]
    pub r#condition_expression: Box<String>,
    /// IAM role to be granted. https://cloud.google.com/iam/docs/roles-overview.
    #[builder(into)]
    #[serde(rename = "role")]
    pub r#role: Box<String>,
}
