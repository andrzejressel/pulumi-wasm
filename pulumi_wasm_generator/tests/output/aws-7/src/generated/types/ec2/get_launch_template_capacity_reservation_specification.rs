#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetLaunchTemplateCapacityReservationSpecification {
    #[builder(into)]
    #[serde(rename = "capacityReservationPreference")]
    pub r#capacity_reservation_preference: Box<String>,
    #[builder(into)]
    #[serde(rename = "capacityReservationTargets")]
    pub r#capacity_reservation_targets: Box<Vec<super::super::types::ec2::GetLaunchTemplateCapacityReservationSpecificationCapacityReservationTarget>>,
}
