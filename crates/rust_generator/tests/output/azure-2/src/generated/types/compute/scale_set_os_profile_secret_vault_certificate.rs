#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ScaleSetOsProfileSecretVaultCertificate {
    /// (Required, on windows machines) Specifies the certificate store on the Virtual Machine where the certificate should be added to.
    #[builder(into, default)]
    #[serde(rename = "certificateStore")]
    pub r#certificate_store: Box<Option<String>>,
    /// It is the Base64 encoding of a JSON Object that which is encoded in UTF-8 of which the contents need to be `data`, `dataType` and `password`.
    #[builder(into)]
    #[serde(rename = "certificateUrl")]
    pub r#certificate_url: Box<String>,
}
