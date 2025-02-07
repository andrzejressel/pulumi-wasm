#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct CertificateCertificatePolicyKeyProperties {
    /// Specifies the curve to use when creating an `EC` key. Possible values are `P-256`, `P-256K`, `P-384`, and `P-521`. This field will be required in a future release if `key_type` is `EC` or `EC-HSM`.
    #[builder(into, default)]
    #[serde(rename = "curve")]
    pub r#curve: Box<Option<String>>,
    /// Is this certificate exportable?
    #[builder(into)]
    #[serde(rename = "exportable")]
    pub r#exportable: Box<bool>,
    /// The size of the key used in the certificate. Possible values include `2048`, `3072`, and `4096` for `RSA` keys, or `256`, `384`, and `521` for `EC` keys. This property is required when using RSA keys.
    #[builder(into, default)]
    #[serde(rename = "keySize")]
    pub r#key_size: Box<Option<i32>>,
    /// Specifies the type of key. Possible values are `EC`, `EC-HSM`, `RSA`, `RSA-HSM` and `oct`.
    #[builder(into)]
    #[serde(rename = "keyType")]
    pub r#key_type: Box<String>,
    /// Is the key reusable?
    #[builder(into)]
    #[serde(rename = "reuseKey")]
    pub r#reuse_key: Box<bool>,
}
