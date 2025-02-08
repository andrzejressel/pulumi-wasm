#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetSAccount {
    /// The Google service account ID (the part before the `@` sign in the `email`)
    #[builder(into)]
    #[serde(rename = "accountId")]
    pub r#account_id: Box<String>,
    /// Whether a service account is disabled or not.
    #[builder(into)]
    #[serde(rename = "disabled")]
    pub r#disabled: Box<bool>,
    /// The display name for the service account.
    #[builder(into)]
    #[serde(rename = "displayName")]
    pub r#display_name: Box<String>,
    /// The e-mail address of the service account. This value
    /// should be referenced from any `gcp.organizations.getIAMPolicy` data sources
    /// that would grant the service account privileges.
    #[builder(into)]
    #[serde(rename = "email")]
    pub r#email: Box<String>,
    /// The Identity of the service account in the form `serviceAccount:{email}`. This value is often used to refer to the service account in order to grant IAM permissions.
    #[builder(into)]
    #[serde(rename = "member")]
    pub r#member: Box<String>,
    /// The fully-qualified name of the service account.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The unique id of the service account.
    #[builder(into)]
    #[serde(rename = "uniqueId")]
    pub r#unique_id: Box<String>,
}
