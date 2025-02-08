#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GdcSparkApplicationSparkSqlApplicationConfigQueryList {
    /// The queries to run.
    #[builder(into)]
    #[serde(rename = "queries")]
    pub r#queries: Box<Vec<String>>,
}
