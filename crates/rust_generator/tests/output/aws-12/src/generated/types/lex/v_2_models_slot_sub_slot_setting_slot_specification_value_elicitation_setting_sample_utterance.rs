#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct V2ModelsSlotSubSlotSettingSlotSpecificationValueElicitationSettingSampleUtterance {
    /// The sample utterance that Amazon Lex uses to build its machine-learning model to recognize intents.
    #[builder(into)]
    #[serde(rename = "utterance")]
    pub r#utterance: Box<String>,
}
