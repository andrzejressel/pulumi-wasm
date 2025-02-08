#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct VirtualMachineOsProfileSecretVaultCertificate {
    /// (Required, on windows machines) Specifies the certificate store on the Virtual Machine where the certificate should be added to, such as `My`.
    #[builder(into, default)]
    #[serde(rename = "certificateStore")]
    pub r#certificate_store: Box<Option<String>>,
    /// The ID of the Key Vault Secret. Stored secret is the Base64 encoding of a JSON Object that which is encoded in UTF-8 of which the contents need to be:
    /// 
    /// ```json
    /// {
    /// "data":"<Base64-encoded-certificate>",
    /// "dataType":"pfx",
    /// "password":"<pfx-file-password>"
    /// }
    /// ```
    /// 
    /// > **NOTE:** If your certificate is stored in Azure Key Vault - this can be sourced from the `secret_id` property on the `azure.keyvault.Certificate` resource.
    #[builder(into)]
    #[serde(rename = "certificateUrl")]
    pub r#certificate_url: Box<String>,
}
