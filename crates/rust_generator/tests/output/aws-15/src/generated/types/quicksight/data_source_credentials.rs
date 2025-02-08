#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct DataSourceCredentials {
    /// The Amazon Resource Name (ARN) of a data source that has the credential pair that you want to use.
    /// When the value is not null, the `credential_pair` from the data source in the ARN is used.
    #[builder(into, default)]
    #[serde(rename = "copySourceArn")]
    pub r#copy_source_arn: Box<Option<String>>,
    /// Credential pair. See Credential Pair below for more details.
    #[builder(into, default)]
    #[serde(rename = "credentialPair")]
    pub r#credential_pair: Box<Option<super::super::types::quicksight::DataSourceCredentialsCredentialPair>>,
    /// The Amazon Resource Name (ARN) of the secret associated with the data source in Amazon Secrets Manager.
    #[builder(into, default)]
    #[serde(rename = "secretArn")]
    pub r#secret_arn: Box<Option<String>>,
}
