//! Make sure that defaults propagate through types

#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct LayeredType {
    /// The answer to the question
    #[builder(into, default)]
    #[serde(rename = "answer")]
    pub r#answer: Box<Option<f64>>,
    #[builder(into)]
    #[serde(rename = "other")]
    pub r#other: Box<crate::types::HelmReleaseSettings>,
    /// Test how plain types interact
    #[builder(into, default)]
    #[serde(rename = "plainOther")]
    pub r#plain_other: Box<Option<crate::types::HelmReleaseSettings>>,
    /// The question already answered
    #[builder(into, default)]
    #[serde(rename = "question")]
    pub r#question: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "recursive")]
    pub r#recursive: Box<Option<crate::types::LayeredType>>,
    /// To ask and answer
    #[builder(into)]
    #[serde(rename = "thinker")]
    pub r#thinker: Box<String>,
}
