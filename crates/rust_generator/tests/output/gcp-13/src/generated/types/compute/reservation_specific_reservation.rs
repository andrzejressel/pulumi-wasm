#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ReservationSpecificReservation {
    /// The number of resources that are allocated.
    #[builder(into)]
    #[serde(rename = "count")]
    pub r#count: Box<i32>,
    /// (Output)
    /// How many instances are in use.
    #[builder(into, default)]
    #[serde(rename = "inUseCount")]
    pub r#in_use_count: Box<Option<i32>>,
    /// The instance properties for the reservation.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "instanceProperties")]
    pub r#instance_properties: Box<super::super::types::compute::ReservationSpecificReservationInstanceProperties>,
}
