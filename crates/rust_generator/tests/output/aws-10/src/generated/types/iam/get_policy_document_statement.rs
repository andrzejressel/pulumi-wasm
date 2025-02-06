#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetPolicyDocumentStatement {
    /// List of actions that this statement either allows or denies. For example, `["ec2:RunInstances", "s3:*"]`.
    #[builder(into, default)]
    #[serde(rename = "actions")]
    pub r#actions: Box<Option<Vec<String>>>,
    /// Configuration block for a condition. Detailed below.
    #[builder(into, default)]
    #[serde(rename = "conditions")]
    pub r#conditions: Box<Option<Vec<super::super::types::iam::GetPolicyDocumentStatementCondition>>>,
    /// Whether this statement allows or denies the given actions. Valid values are `Allow` and `Deny`. Defaults to `Allow`.
    #[builder(into, default)]
    #[serde(rename = "effect")]
    pub r#effect: Box<Option<String>>,
    /// List of actions that this statement does *not* apply to. Use to apply a policy statement to all actions *except* those listed.
    #[builder(into, default)]
    #[serde(rename = "notActions")]
    pub r#not_actions: Box<Option<Vec<String>>>,
    /// Like `principals` except these are principals that the statement does *not* apply to.
    #[builder(into, default)]
    #[serde(rename = "notPrincipals")]
    pub r#not_principals: Box<Option<Vec<super::super::types::iam::GetPolicyDocumentStatementNotPrincipal>>>,
    /// List of resource ARNs that this statement does *not* apply to. Use to apply a policy statement to all resources *except* those listed. Conflicts with `resources`.
    #[builder(into, default)]
    #[serde(rename = "notResources")]
    pub r#not_resources: Box<Option<Vec<String>>>,
    /// Configuration block for principals. Detailed below.
    #[builder(into, default)]
    #[serde(rename = "principals")]
    pub r#principals: Box<Option<Vec<super::super::types::iam::GetPolicyDocumentStatementPrincipal>>>,
    /// List of resource ARNs that this statement applies to. This is required by AWS if used for an IAM policy. Conflicts with `not_resources`.
    #[builder(into, default)]
    #[serde(rename = "resources")]
    pub r#resources: Box<Option<Vec<String>>>,
    /// Sid (statement ID) is an identifier for a policy statement.
    #[builder(into, default)]
    #[serde(rename = "sid")]
    pub r#sid: Box<Option<String>>,
}
