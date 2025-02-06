#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetCertificateCertificatePolicyKeyProperty {
    #[builder(into)]
    #[serde(rename = "curve")]
    pub r#curve: Box<String>,
    /// Is this Certificate Exportable?
    #[builder(into)]
    #[serde(rename = "exportable")]
    pub r#exportable: Box<bool>,
    /// The size of the Key used in the Certificate.
    #[builder(into)]
    #[serde(rename = "keySize")]
    pub r#key_size: Box<i32>,
    /// Specifies the Type of Key, for example `RSA`.
    #[builder(into)]
    #[serde(rename = "keyType")]
    pub r#key_type: Box<String>,
    /// Is the key reusable?
    #[builder(into)]
    #[serde(rename = "reuseKey")]
    pub r#reuse_key: Box<bool>,
}
