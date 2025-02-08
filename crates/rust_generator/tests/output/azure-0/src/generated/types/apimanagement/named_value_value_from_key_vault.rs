#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct NamedValueValueFromKeyVault {
    /// The client ID of User Assigned Identity, for the API Management Service, which will be used to access the key vault secret. The System Assigned Identity will be used in absence.
    #[builder(into, default)]
    #[serde(rename = "identityClientId")]
    pub r#identity_client_id: Box<Option<String>>,
    /// The resource ID of the Key Vault Secret.
    #[builder(into)]
    #[serde(rename = "secretId")]
    pub r#secret_id: Box<String>,
}
