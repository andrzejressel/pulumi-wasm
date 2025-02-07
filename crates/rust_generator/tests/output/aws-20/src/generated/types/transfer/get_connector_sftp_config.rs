#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetConnectorSftpConfig {
    /// List of the public portions of the host keys that are used to identify the servers the connector is connected to.
    #[builder(into)]
    #[serde(rename = "trustedHostKeys")]
    pub r#trusted_host_keys: Box<Vec<String>>,
    /// Identifer for the secret in AWS Secrets Manager that contains the SFTP user's private key, and/or password.
    #[builder(into)]
    #[serde(rename = "userSecretId")]
    pub r#user_secret_id: Box<String>,
}
