#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct AuthorityConfigSubjectKeyId {
    /// The value of the KeyId in lowercase hexadecimal.
    /// 
    /// <a name="nested_x509_config"></a>The `x509_config` block supports:
    #[builder(into, default)]
    #[serde(rename = "keyId")]
    pub r#key_id: Box<Option<String>>,
}
