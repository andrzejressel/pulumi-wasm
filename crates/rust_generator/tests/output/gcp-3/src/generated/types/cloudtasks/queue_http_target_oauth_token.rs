#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct QueueHttpTargetOauthToken {
    /// OAuth scope to be used for generating OAuth access token.
    /// If not specified, "https://www.googleapis.com/auth/cloud-platform" will be used.
    #[builder(into, default)]
    #[serde(rename = "scope")]
    pub r#scope: Box<Option<String>>,
    /// Service account email to be used for generating OAuth token.
    /// The service account must be within the same project as the queue.
    /// The caller must have iam.serviceAccounts.actAs permission for the service account.
    #[builder(into)]
    #[serde(rename = "serviceAccountEmail")]
    pub r#service_account_email: Box<String>,
}
