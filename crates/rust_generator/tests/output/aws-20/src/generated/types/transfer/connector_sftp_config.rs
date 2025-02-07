#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ConnectorSftpConfig {
    /// A list of public portion of the host key, or keys, that are used to authenticate the user to the external server to which you are connecting.(https://docs.aws.amazon.com/transfer/latest/userguide/API_SftpConnectorConfig.html)
    #[builder(into, default)]
    #[serde(rename = "trustedHostKeys")]
    pub r#trusted_host_keys: Box<Option<Vec<String>>>,
    /// The identifier for the secret (in AWS Secrets Manager) that contains the SFTP user's private key, password, or both. The identifier can be either the Amazon Resource Name (ARN) or the name of the secret.
    #[builder(into, default)]
    #[serde(rename = "userSecretId")]
    pub r#user_secret_id: Box<Option<String>>,
}
