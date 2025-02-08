#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ClusterMaintenancePolicyWindow {
    /// Represents an arbitrary window of time that recurs.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "recurringWindow")]
    pub r#recurring_window: Box<super::super::types::edgecontainer::ClusterMaintenancePolicyWindowRecurringWindow>,
}
