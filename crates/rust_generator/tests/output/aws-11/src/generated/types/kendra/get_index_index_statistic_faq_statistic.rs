#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetIndexIndexStatisticFaqStatistic {
    /// The total number of FAQ questions and answers contained in the index.
    #[builder(into)]
    #[serde(rename = "indexedQuestionAnswersCount")]
    pub r#indexed_question_answers_count: Box<i32>,
}
