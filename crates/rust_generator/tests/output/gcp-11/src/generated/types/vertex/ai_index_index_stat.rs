#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AiIndexIndexStat {
    /// (Output)
    /// The number of shards in the Index.
    #[builder(into, default)]
    #[serde(rename = "shardsCount")]
    pub r#shards_count: Box<Option<i32>>,
    /// (Output)
    /// The number of vectors in the Index.
    #[builder(into, default)]
    #[serde(rename = "vectorsCount")]
    pub r#vectors_count: Box<Option<String>>,
}
