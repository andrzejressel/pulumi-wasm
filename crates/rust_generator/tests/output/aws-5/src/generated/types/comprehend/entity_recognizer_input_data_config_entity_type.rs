#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct EntityRecognizerInputDataConfigEntityType {
    /// An entity type to be matched by the Entity Recognizer.
    /// Cannot contain a newline (`\n`), carriage return (`\r`), or tab (`\t`).
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
