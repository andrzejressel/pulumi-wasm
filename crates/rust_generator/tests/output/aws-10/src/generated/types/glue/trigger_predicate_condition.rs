#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct TriggerPredicateCondition {
    /// The condition crawl state. Currently, the values supported are `RUNNING`, `SUCCEEDED`, `CANCELLED`, and `FAILED`. If this is specified, `crawler_name` must also be specified. Conflicts with `state`.
    #[builder(into, default)]
    #[serde(rename = "crawlState")]
    pub r#crawl_state: Box<Option<String>>,
    /// The name of the crawler to watch. If this is specified, `crawl_state` must also be specified. Conflicts with `job_name`.
    #[builder(into, default)]
    #[serde(rename = "crawlerName")]
    pub r#crawler_name: Box<Option<String>>,
    /// The name of the job to watch. If this is specified, `state` must also be specified. Conflicts with `crawler_name`.
    #[builder(into, default)]
    #[serde(rename = "jobName")]
    pub r#job_name: Box<Option<String>>,
    /// A logical operator. Defaults to `EQUALS`.
    #[builder(into, default)]
    #[serde(rename = "logicalOperator")]
    pub r#logical_operator: Box<Option<String>>,
    /// The condition job state. Currently, the values supported are `SUCCEEDED`, `STOPPED`, `TIMEOUT` and `FAILED`. If this is specified, `job_name` must also be specified. Conflicts with `crawler_state`.
    #[builder(into, default)]
    #[serde(rename = "state")]
    pub r#state: Box<Option<String>>,
}
