#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ConnectionAuthConfigOauth2ClientCredentialsClientSecret {
    /// The resource name of the secret version in the format,
    /// format as: projects/*/secrets/*/versions/*.
    #[builder(into)]
    #[serde(rename = "secretVersion")]
    pub r#secret_version: Box<String>,
}
