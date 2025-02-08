#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ScheduledActionScalableTargetAction {
    /// Maximum capacity. At least one of `max_capacity` or `min_capacity` must be set.
    #[builder(into, default)]
    #[serde(rename = "maxCapacity")]
    pub r#max_capacity: Box<Option<i32>>,
    /// Minimum capacity. At least one of `min_capacity` or `max_capacity` must be set.
    #[builder(into, default)]
    #[serde(rename = "minCapacity")]
    pub r#min_capacity: Box<Option<i32>>,
}
