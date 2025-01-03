#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ConnectionBitbucketDataCenterConfig {
    /// Required. A http access token with the `REPO_ADMIN` scope access.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "authorizerCredential")]
    pub r#authorizer_credential: Box<super::super::types::cloudbuildv2::ConnectionBitbucketDataCenterConfigAuthorizerCredential>,
    /// The URI of the Bitbucket Data Center host this connection is for.
    #[builder(into)]
    #[serde(rename = "hostUri")]
    pub r#host_uri: Box<String>,
    /// Required. A http access token with the `REPO_READ` access.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "readAuthorizerCredential")]
    pub r#read_authorizer_credential: Box<super::super::types::cloudbuildv2::ConnectionBitbucketDataCenterConfigReadAuthorizerCredential>,
    /// (Output)
    /// Output only. Version of the Bitbucket Data Center running on the `host_uri`.
    #[builder(into, default)]
    #[serde(rename = "serverVersion")]
    pub r#server_version: Box<Option<String>>,
    /// Configuration for using Service Directory to privately connect to a Bitbucket Data Center. This should only be set if the Bitbucket Data Center is hosted on-premises and not reachable by public internet. If this field is left empty, calls to the Bitbucket Data Center will be made over the public internet.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "serviceDirectoryConfig")]
    pub r#service_directory_config: Box<Option<super::super::types::cloudbuildv2::ConnectionBitbucketDataCenterConfigServiceDirectoryConfig>>,
    /// SSL certificate to use for requests to the Bitbucket Data Center.
    #[builder(into, default)]
    #[serde(rename = "sslCa")]
    pub r#ssl_ca: Box<Option<String>>,
    /// Required. Immutable. SecretManager resource containing the webhook secret used to verify webhook events, formatted as `projects/*/secrets/*/versions/*`.
    #[builder(into)]
    #[serde(rename = "webhookSecretSecretVersion")]
    pub r#webhook_secret_secret_version: Box<String>,
}
