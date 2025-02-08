#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ReservationAutoscale {
    /// (Output)
    /// The slot capacity added to this reservation when autoscale happens. Will be between [0, max_slots].
    #[builder(into, default)]
    #[serde(rename = "currentSlots")]
    pub r#current_slots: Box<Option<i32>>,
    /// Number of slots to be scaled when needed.
    #[builder(into, default)]
    #[serde(rename = "maxSlots")]
    pub r#max_slots: Box<Option<i32>>,
}
