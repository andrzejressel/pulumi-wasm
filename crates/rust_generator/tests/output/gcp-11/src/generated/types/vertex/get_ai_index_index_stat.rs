#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetAiIndexIndexStat {
    /// The number of shards in the Index.
    #[builder(into)]
    #[serde(rename = "shardsCount")]
    pub r#shards_count: Box<i32>,
    /// The number of vectors in the Index.
    #[builder(into)]
    #[serde(rename = "vectorsCount")]
    pub r#vectors_count: Box<String>,
}
