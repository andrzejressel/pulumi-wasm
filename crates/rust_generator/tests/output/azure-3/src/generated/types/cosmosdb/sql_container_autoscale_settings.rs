#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct SqlContainerAutoscaleSettings {
    /// The maximum throughput of the SQL container (RU/s). Must be between `1,000` and `1,000,000`. Must be set in increments of `1,000`. Conflicts with `throughput`.
    #[builder(into, default)]
    #[serde(rename = "maxThroughput")]
    pub r#max_throughput: Box<Option<i32>>,
}
