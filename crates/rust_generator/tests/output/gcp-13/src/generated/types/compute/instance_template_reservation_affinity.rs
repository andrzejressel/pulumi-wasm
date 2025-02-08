#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct InstanceTemplateReservationAffinity {
    /// Specifies the label selector for the reservation to use..
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "specificReservation")]
    pub r#specific_reservation: Box<Option<super::super::types::compute::InstanceTemplateReservationAffinitySpecificReservation>>,
    /// The type of reservation from which this instance can consume resources.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
