#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct WebhookAuthenticationConfiguration {
    /// A valid CIDR block for `IP` filtering. Required for `IP`.
    #[builder(into, default)]
    #[serde(rename = "allowedIpRange")]
    pub r#allowed_ip_range: Box<Option<String>>,
    /// The shared secret for the GitHub repository webhook. Set this as `secret` in your `github_repository_webhook`'s `configuration` block. Required for `GITHUB_HMAC`.
    #[builder(into, default)]
    #[serde(rename = "secretToken")]
    pub r#secret_token: Box<Option<String>>,
}
