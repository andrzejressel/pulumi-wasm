#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetIamPolicyBinding {
    /// An [IAM Condition](https://cloud.google.com/iam/docs/conditions-overview) for a given binding. Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "condition")]
    pub r#condition: Box<Option<super::super::types::organizations::GetIamPolicyBindingCondition>>,
    /// An array of identities that will be granted the privilege in the `role`. For more details on format and restrictions see https://cloud.google.com/billing/reference/rest/v1/Policy#Binding
    /// Each entry can have one of the following values:
    /// * **allUsers**: A special identifier that represents anyone who is on the internet; with or without a Google account. Some resources **don't** support this identity.
    /// * **allAuthenticatedUsers**: A special identifier that represents anyone who is authenticated with a Google account or a service account. Some resources **don't** support this identity.
    /// * **user:{emailid}**: An email address that represents a specific Google account. For example, alice@gmail.com.
    /// * **serviceAccount:{emailid}**: An email address that represents a service account. For example, my-other-app@appspot.gserviceaccount.com.
    /// * **group:{emailid}**: An email address that represents a Google group. For example, admins@example.com.
    /// * **domain:{domain}**: A G Suite domain (primary, instead of alias) name that represents all the users of that domain. For example, google.com or example.com.
    #[builder(into)]
    #[serde(rename = "members")]
    pub r#members: Box<Vec<String>>,
    /// The role/permission that will be granted to the members.
    /// See the [IAM Roles](https://cloud.google.com/compute/docs/access/iam) documentation for a complete list of roles.
    /// Note that custom roles must be of the format `[projects|organizations]/{parent-name}/roles/{role-name}`.
    #[builder(into)]
    #[serde(rename = "role")]
    pub r#role: Box<String>,
}
