#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetAppIngressTrafficWeight {
    /// The label to apply to the revision as a name prefix for routing traffic.
    #[builder(into)]
    #[serde(rename = "label")]
    pub r#label: Box<String>,
    /// This traffic Weight relates to the latest stable Container Revision.
    #[builder(into)]
    #[serde(rename = "latestRevision")]
    pub r#latest_revision: Box<bool>,
    /// The percentage of traffic which should be sent this revision.
    #[builder(into)]
    #[serde(rename = "percentage")]
    pub r#percentage: Box<i32>,
    /// The suffix string to which this `traffic_weight` applies.
    #[builder(into)]
    #[serde(rename = "revisionSuffix")]
    pub r#revision_suffix: Box<String>,
}
