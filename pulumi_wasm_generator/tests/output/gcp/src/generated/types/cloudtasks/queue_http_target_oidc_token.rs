#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct QueueHttpTargetOidcToken {
    /// Audience to be used when generating OIDC token. If not specified, the URI specified in target will be used.
    #[builder(into, default)]
    #[serde(rename = "audience")]
    pub r#audience: Box<Option<String>>,
    /// Service account email to be used for generating OIDC token.
    /// The service account must be within the same project as the queue.
    /// The caller must have iam.serviceAccounts.actAs permission for the service account.
    #[builder(into)]
    #[serde(rename = "serviceAccountEmail")]
    pub r#service_account_email: Box<String>,
}
