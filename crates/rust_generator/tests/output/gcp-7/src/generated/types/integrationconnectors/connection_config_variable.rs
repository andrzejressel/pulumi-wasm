#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ConnectionConfigVariable {
    /// Boolean Value of configVariable
    #[builder(into, default)]
    #[serde(rename = "booleanValue")]
    pub r#boolean_value: Box<Option<bool>>,
    /// Encryption key value of configVariable.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "encryptionKeyValue")]
    pub r#encryption_key_value: Box<Option<super::super::types::integrationconnectors::ConnectionConfigVariableEncryptionKeyValue>>,
    /// Integer Value of configVariable
    #[builder(into, default)]
    #[serde(rename = "integerValue")]
    pub r#integer_value: Box<Option<i32>>,
    /// Key for the configVariable
    #[builder(into)]
    #[serde(rename = "key")]
    pub r#key: Box<String>,
    /// Secret value of configVariable.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "secretValue")]
    pub r#secret_value: Box<Option<super::super::types::integrationconnectors::ConnectionConfigVariableSecretValue>>,
    /// String Value of configVariabley
    #[builder(into, default)]
    #[serde(rename = "stringValue")]
    pub r#string_value: Box<Option<String>>,
}
