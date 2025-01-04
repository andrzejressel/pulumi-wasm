#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct IndexIndexStatisticFaqStatistic {
    /// The total number of FAQ questions and answers contained in the index.
    #[builder(into, default)]
    #[serde(rename = "indexedQuestionAnswersCount")]
    pub r#indexed_question_answers_count: Box<Option<i32>>,
}
