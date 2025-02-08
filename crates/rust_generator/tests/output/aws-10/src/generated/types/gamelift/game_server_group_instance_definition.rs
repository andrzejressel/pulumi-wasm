#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GameServerGroupInstanceDefinition {
    /// An EC2 instance type.
    #[builder(into)]
    #[serde(rename = "instanceType")]
    pub r#instance_type: Box<String>,
    /// Instance weighting that indicates how much this instance type contributes
    /// to the total capacity of a game server group.
    /// Instance weights are used by GameLift FleetIQ to calculate the instance type's cost per unit hour and better identify
    /// the most cost-effective options.
    #[builder(into, default)]
    #[serde(rename = "weightedCapacity")]
    pub r#weighted_capacity: Box<Option<String>>,
}
