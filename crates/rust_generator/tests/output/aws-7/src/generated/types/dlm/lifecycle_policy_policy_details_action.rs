#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct LifecyclePolicyPolicyDetailsAction {
    /// The rule for copying shared snapshots across Regions. See the `cross_region_copy` configuration block.
    #[builder(into)]
    #[serde(rename = "crossRegionCopies")]
    pub r#cross_region_copies: Box<Vec<super::super::types::dlm::LifecyclePolicyPolicyDetailsActionCrossRegionCopy>>,
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}
