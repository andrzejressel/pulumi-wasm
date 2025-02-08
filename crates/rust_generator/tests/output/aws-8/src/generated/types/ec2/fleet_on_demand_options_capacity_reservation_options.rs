#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FleetOnDemandOptionsCapacityReservationOptions {
    /// Indicates whether to use unused Capacity Reservations for fulfilling On-Demand capacity. Valid values: `use-capacity-reservations-first`.
    #[builder(into, default)]
    #[serde(rename = "usageStrategy")]
    pub r#usage_strategy: Box<Option<String>>,
}
