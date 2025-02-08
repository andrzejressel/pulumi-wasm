#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct SparkClusterRolesWorkerNodeAutoscale {
    /// A `capacity` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "capacity")]
    pub r#capacity: Box<Option<super::super::types::hdinsight::SparkClusterRolesWorkerNodeAutoscaleCapacity>>,
    /// A `recurrence` block as defined below.
    /// 
    /// > **NOTE:** Either a `capacity` or `recurrence` block must be specified - but not both.
    #[builder(into, default)]
    #[serde(rename = "recurrence")]
    pub r#recurrence: Box<Option<super::super::types::hdinsight::SparkClusterRolesWorkerNodeAutoscaleRecurrence>>,
}
