#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct LayeredType {
    /// The answer to the question
    #[builder(into, default)]
    #[serde(rename = "answer")]
    pub r#answer: Box<Option<f64>>,
    #[builder(into)]
    #[serde(rename = "other")]
    pub r#other: Box<super::types::HelmReleaseSettings>,
    /// Test how plain types interact
    #[builder(into, default)]
    #[serde(rename = "plainOther")]
    pub r#plain_other: Box<Option<super::types::HelmReleaseSettings>>,
    /// The question already answered
    #[builder(into, default)]
    #[serde(rename = "question")]
    pub r#question: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "recursive")]
    pub r#recursive: Box<Option<super::types::LayeredType>>,
    /// To ask and answer
    #[builder(into)]
    #[serde(rename = "thinker")]
    pub r#thinker: Box<String>,
}
