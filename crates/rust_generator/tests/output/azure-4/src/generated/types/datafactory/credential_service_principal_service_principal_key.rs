#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct CredentialServicePrincipalServicePrincipalKey {
    /// The name of the Linked Service to use for the Service Principal Key.
    #[builder(into)]
    #[serde(rename = "linkedServiceName")]
    pub r#linked_service_name: Box<String>,
    /// The name of the Secret in the Key Vault.
    #[builder(into)]
    #[serde(rename = "secretName")]
    pub r#secret_name: Box<String>,
    /// The version of the Secret in the Key Vault.
    #[builder(into, default)]
    #[serde(rename = "secretVersion")]
    pub r#secret_version: Box<Option<String>>,
}
