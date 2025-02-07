#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct BitbucketServerConfigSecrets {
    /// The resource name for the admin access token's secret version.
    #[builder(into)]
    #[serde(rename = "adminAccessTokenVersionName")]
    pub r#admin_access_token_version_name: Box<String>,
    /// The resource name for the read access token's secret version.
    #[builder(into)]
    #[serde(rename = "readAccessTokenVersionName")]
    pub r#read_access_token_version_name: Box<String>,
    /// Immutable. The resource name for the webhook secret's secret version. Once this field has been set, it cannot be changed.
    /// Changing this field will result in deleting/ recreating the resource.
    /// 
    /// - - -
    #[builder(into)]
    #[serde(rename = "webhookSecretVersionName")]
    pub r#webhook_secret_version_name: Box<String>,
}
