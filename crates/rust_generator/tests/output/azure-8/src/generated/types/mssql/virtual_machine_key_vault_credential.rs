#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct VirtualMachineKeyVaultCredential {
    /// The Azure Key Vault url. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "keyVaultUrl")]
    pub r#key_vault_url: Box<String>,
    /// The credential name.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The service principal name to access key vault. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "servicePrincipalName")]
    pub r#service_principal_name: Box<String>,
    /// The service principal name secret to access key vault. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "servicePrincipalSecret")]
    pub r#service_principal_secret: Box<String>,
}
