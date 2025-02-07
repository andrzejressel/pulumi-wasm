#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RegionInstanceGroupManagerInstanceFlexibilityPolicy {
    /// Named instance selections configuring properties that the group will use when creating new VMs.
    #[builder(into, default)]
    #[serde(rename = "instanceSelections")]
    pub r#instance_selections: Box<Option<Vec<super::super::types::compute::RegionInstanceGroupManagerInstanceFlexibilityPolicyInstanceSelection>>>,
}
