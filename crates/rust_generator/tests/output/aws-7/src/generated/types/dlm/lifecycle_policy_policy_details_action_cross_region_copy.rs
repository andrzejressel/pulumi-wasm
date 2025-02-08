#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct LifecyclePolicyPolicyDetailsActionCrossRegionCopy {
    /// The encryption settings for the copied snapshot. See the `encryption_configuration` block. Max of 1 per action.
    #[builder(into)]
    #[serde(rename = "encryptionConfiguration")]
    pub r#encryption_configuration: Box<super::super::types::dlm::LifecyclePolicyPolicyDetailsActionCrossRegionCopyEncryptionConfiguration>,
    #[builder(into, default)]
    #[serde(rename = "retainRule")]
    pub r#retain_rule: Box<Option<super::super::types::dlm::LifecyclePolicyPolicyDetailsActionCrossRegionCopyRetainRule>>,
    #[builder(into)]
    #[serde(rename = "target")]
    pub r#target: Box<String>,
}
