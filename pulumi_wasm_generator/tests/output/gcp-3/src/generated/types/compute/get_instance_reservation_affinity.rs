#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetInstanceReservationAffinity {
    /// Specifies the label selector for the reservation to use.
    #[builder(into)]
    #[serde(rename = "specificReservations")]
    pub r#specific_reservations: Box<Vec<super::super::types::compute::GetInstanceReservationAffinitySpecificReservation>>,
    /// The accelerator type resource exposed to this instance. E.g. `nvidia-tesla-k80`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type: Box<String>,
}
