#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct V2ModelsSlotValueElicitationSettingWaitAndContinueSpecificationContinueResponseMessageGroupMessageImageResponseCard {
    #[builder(into, default)]
    #[serde(rename = "buttons")]
    pub r#buttons: Box<Option<Vec<super::super::types::lex::V2ModelsSlotValueElicitationSettingWaitAndContinueSpecificationContinueResponseMessageGroupMessageImageResponseCardButton>>>,
    #[builder(into, default)]
    #[serde(rename = "imageUrl")]
    pub r#image_url: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "subtitle")]
    pub r#subtitle: Box<Option<String>>,
    #[builder(into)]
    #[serde(rename = "title")]
    pub r#title: Box<String>,
}