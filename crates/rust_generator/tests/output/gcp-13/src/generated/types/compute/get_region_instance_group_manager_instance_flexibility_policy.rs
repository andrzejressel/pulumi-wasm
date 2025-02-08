#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetRegionInstanceGroupManagerInstanceFlexibilityPolicy {
    /// Named instance selections configuring properties that the group will use when creating new VMs.
    #[builder(into)]
    #[serde(rename = "instanceSelections")]
    pub r#instance_selections: Box<Vec<super::super::types::compute::GetRegionInstanceGroupManagerInstanceFlexibilityPolicyInstanceSelection>>,
}
