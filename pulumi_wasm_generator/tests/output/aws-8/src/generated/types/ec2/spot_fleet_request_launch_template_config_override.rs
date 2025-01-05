#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct SpotFleetRequestLaunchTemplateConfigOverride {
    /// The availability zone in which to place the request.
    #[builder(into, default)]
    #[serde(rename = "availabilityZone")]
    pub r#availability_zone: Box<Option<String>>,
    /// The instance requirements. See below.
    #[builder(into, default)]
    #[serde(rename = "instanceRequirements")]
    pub r#instance_requirements: Box<Option<super::super::types::ec2::SpotFleetRequestLaunchTemplateConfigOverrideInstanceRequirements>>,
    /// The type of instance to request.
    #[builder(into, default)]
    #[serde(rename = "instanceType")]
    pub r#instance_type: Box<Option<String>>,
    /// The priority for the launch template override. The lower the number, the higher the priority. If no number is set, the launch template override has the lowest priority.
    #[builder(into, default)]
    #[serde(rename = "priority")]
    pub r#priority: Box<Option<f64>>,
    /// The maximum spot bid for this override request.
    #[builder(into, default)]
    #[serde(rename = "spotPrice")]
    pub r#spot_price: Box<Option<String>>,
    /// The subnet in which to launch the requested instance.
    #[builder(into, default)]
    #[serde(rename = "subnetId")]
    pub r#subnet_id: Box<Option<String>>,
    /// The capacity added to the fleet by a fulfilled request.
    #[builder(into, default)]
    #[serde(rename = "weightedCapacity")]
    pub r#weighted_capacity: Box<Option<f64>>,
}
