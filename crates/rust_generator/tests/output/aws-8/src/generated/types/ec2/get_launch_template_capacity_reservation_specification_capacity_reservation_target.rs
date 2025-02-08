#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetLaunchTemplateCapacityReservationSpecificationCapacityReservationTarget {
    #[builder(into)]
    #[serde(rename = "capacityReservationId")]
    pub r#capacity_reservation_id: Box<String>,
    #[builder(into)]
    #[serde(rename = "capacityReservationResourceGroupArn")]
    pub r#capacity_reservation_resource_group_arn: Box<String>,
}
