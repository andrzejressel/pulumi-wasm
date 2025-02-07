#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct KeyKeyAttributes {
    /// Key algorithm to be use during creation of an AWS Payment Cryptography key.
    #[builder(into)]
    #[serde(rename = "keyAlgorithm")]
    pub r#key_algorithm: Box<String>,
    /// Type of AWS Payment Cryptography key to create.
    #[builder(into)]
    #[serde(rename = "keyClass")]
    pub r#key_class: Box<String>,
    /// List of cryptographic operations that you can perform using the key.
    #[builder(into, default)]
    #[serde(rename = "keyModesOfUse")]
    pub r#key_modes_of_use: Box<Option<super::super::types::paymentcryptography::KeyKeyAttributesKeyModesOfUse>>,
    /// Cryptographic usage of an AWS Payment Cryptography key as defined in section A.5.2 of the TR-31 spec.
    #[builder(into)]
    #[serde(rename = "keyUsage")]
    pub r#key_usage: Box<String>,
}
