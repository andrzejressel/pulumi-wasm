#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct JobHttpTargetOidcToken {
    /// Audience to be used when generating OIDC token. If not specified,
    /// the URI specified in target will be used.
    #[builder(into, default)]
    #[serde(rename = "audience")]
    pub r#audience: Box<Option<String>>,
    /// Service account email to be used for generating OAuth token.
    /// The service account must be within the same project as the job.
    #[builder(into)]
    #[serde(rename = "serviceAccountEmail")]
    pub r#service_account_email: Box<String>,
}
