#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ExperimentTemplateActionParameter {
    /// Parameter name.
    #[builder(into)]
    #[serde(rename = "key")]
    pub r#key: Box<String>,
    /// Parameter value.
    /// 
    /// For a list of parameters supported by each action, see [AWS FIS actions reference](https://docs.aws.amazon.com/fis/latest/userguide/fis-actions-reference.html).
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Box<String>,
}
