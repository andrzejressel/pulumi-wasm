#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FieldLevelEncryptionConfigContentTypeProfileConfigContentTypeProfilesItem {
    /// he content type for a field-level encryption content type-profile mapping. Valid value is `application/x-www-form-urlencoded`.
    #[builder(into)]
    #[serde(rename = "contentType")]
    pub r#content_type: Box<String>,
    /// The format for a field-level encryption content type-profile mapping. Valid value is `URLEncoded`.
    #[builder(into)]
    #[serde(rename = "format")]
    pub r#format: Box<String>,
    #[builder(into, default)]
    #[serde(rename = "profileId")]
    pub r#profile_id: Box<Option<String>>,
}
