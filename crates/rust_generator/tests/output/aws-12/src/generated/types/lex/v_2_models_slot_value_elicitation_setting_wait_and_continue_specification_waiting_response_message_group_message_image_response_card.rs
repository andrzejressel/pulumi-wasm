#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct V2ModelsSlotValueElicitationSettingWaitAndContinueSpecificationWaitingResponseMessageGroupMessageImageResponseCard {
    #[builder(into, default)]
    #[serde(rename = "buttons")]
    pub r#buttons: Box<Option<Vec<super::super::types::lex::V2ModelsSlotValueElicitationSettingWaitAndContinueSpecificationWaitingResponseMessageGroupMessageImageResponseCardButton>>>,
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
