#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct LaunchTemplateCapacityReservationSpecification {
    /// Indicates the instance's Capacity Reservation preferences. Can be `open` or `none`. (Default `none`).
    #[builder(into, default)]
    #[serde(rename = "capacityReservationPreference")]
    pub r#capacity_reservation_preference: Box<Option<String>>,
    /// Used to target a specific Capacity Reservation:
    #[builder(into, default)]
    #[serde(rename = "capacityReservationTarget")]
    pub r#capacity_reservation_target: Box<Option<super::super::types::ec2::LaunchTemplateCapacityReservationSpecificationCapacityReservationTarget>>,
}
