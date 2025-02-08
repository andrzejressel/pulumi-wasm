#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetPoolStartTaskUserIdentity {
    /// A `auto_user` block that describes the user identity under which the start task runs.
    #[builder(into)]
    #[serde(rename = "autoUsers")]
    pub r#auto_users: Box<Vec<super::super::types::batch::GetPoolStartTaskUserIdentityAutoUser>>,
    /// The user to use for authentication against the CIFS file system.
    #[builder(into)]
    #[serde(rename = "userName")]
    pub r#user_name: Box<String>,
}
