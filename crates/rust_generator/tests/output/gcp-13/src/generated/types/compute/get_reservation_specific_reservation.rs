#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetReservationSpecificReservation {
    /// The number of resources that are allocated.
    #[builder(into)]
    #[serde(rename = "count")]
    pub r#count: Box<i32>,
    /// How many instances are in use.
    #[builder(into)]
    #[serde(rename = "inUseCount")]
    pub r#in_use_count: Box<i32>,
    /// The instance properties for the reservation.
    #[builder(into)]
    #[serde(rename = "instanceProperties")]
    pub r#instance_properties: Box<Vec<super::super::types::compute::GetReservationSpecificReservationInstanceProperty>>,
}
