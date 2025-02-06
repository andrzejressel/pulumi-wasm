#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AiFeatureGroupBigQuery {
    /// The BigQuery source URI that points to either a BigQuery Table or View.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "bigQuerySource")]
    pub r#big_query_source: Box<super::super::types::vertex::AiFeatureGroupBigQueryBigQuerySource>,
    /// Columns to construct entityId / row keys. If not provided defaults to entityId.
    #[builder(into, default)]
    #[serde(rename = "entityIdColumns")]
    pub r#entity_id_columns: Box<Option<Vec<String>>>,
}
