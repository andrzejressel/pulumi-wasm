#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetRegionInstanceTemplateReservationAffinity {
    /// Specifies the label selector for the reservation to use.
    #[builder(into)]
    #[serde(rename = "specificReservations")]
    pub r#specific_reservations: Box<Vec<super::super::types::compute::GetRegionInstanceTemplateReservationAffinitySpecificReservation>>,
    /// The accelerator type resource to expose to this instance. E.g. `nvidia-tesla-k80`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
