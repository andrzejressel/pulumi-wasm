#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct CaPoolIssuancePolicyAllowedKeyTypeRsa {
    /// The maximum allowed RSA modulus size, in bits. If this is not set, or if set to zero, the
    /// service will not enforce an explicit upper bound on RSA modulus sizes.
    #[builder(into, default)]
    #[serde(rename = "maxModulusSize")]
    pub r#max_modulus_size: Box<Option<String>>,
    /// The minimum allowed RSA modulus size, in bits. If this is not set, or if set to zero, the
    /// service-level min RSA modulus size will continue to apply.
    #[builder(into, default)]
    #[serde(rename = "minModulusSize")]
    pub r#min_modulus_size: Box<Option<String>>,
}
