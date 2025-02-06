#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsInforNexus {
    /// The Access Key portion of the credentials.
    #[builder(into)]
    #[serde(rename = "accessKeyId")]
    pub r#access_key_id: Box<String>,
    /// Encryption keys used to encrypt data.
    #[builder(into)]
    #[serde(rename = "datakey")]
    pub r#datakey: Box<String>,
    /// The secret key used to sign requests.
    #[builder(into)]
    #[serde(rename = "secretAccessKey")]
    pub r#secret_access_key: Box<String>,
    /// Identifier for the user.
    #[builder(into)]
    #[serde(rename = "userId")]
    pub r#user_id: Box<String>,
}
