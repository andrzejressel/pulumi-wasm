#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GremlinGraphIndexPolicyCompositeIndex {
    /// One or more `index` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "indices")]
    pub r#indices: Box<Vec<super::super::types::cosmosdb::GremlinGraphIndexPolicyCompositeIndexIndex>>,
}
