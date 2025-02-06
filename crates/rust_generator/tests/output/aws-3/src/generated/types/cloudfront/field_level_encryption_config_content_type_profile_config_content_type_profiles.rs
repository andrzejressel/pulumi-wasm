#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FieldLevelEncryptionConfigContentTypeProfileConfigContentTypeProfiles {
    #[builder(into)]
    #[serde(rename = "items")]
    pub r#items: Box<Vec<super::super::types::cloudfront::FieldLevelEncryptionConfigContentTypeProfileConfigContentTypeProfilesItem>>,
}
