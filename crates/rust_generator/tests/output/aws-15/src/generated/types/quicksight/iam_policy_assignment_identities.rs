#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct IamPolicyAssignmentIdentities {
    /// Array of Quicksight group names to assign the policy to.
    #[builder(into, default)]
    #[serde(rename = "groups")]
    pub r#groups: Box<Option<Vec<String>>>,
    /// Array of Quicksight user names to assign the policy to.
    #[builder(into, default)]
    #[serde(rename = "users")]
    pub r#users: Box<Option<Vec<String>>>,
}
