#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct LifecyclePolicyPolicyDetails {
    /// The actions to be performed when the event-based policy is triggered. You can specify only one action per policy. This parameter is required for event-based policies only. If you are creating a snapshot or AMI policy, omit this parameter. See the `action` configuration block.
    #[builder(into, default)]
    #[serde(rename = "action")]
    pub r#action: Box<Option<super::super::types::dlm::LifecyclePolicyPolicyDetailsAction>>,
    /// The event that triggers the event-based policy. This parameter is required for event-based policies only. If you are creating a snapshot or AMI policy, omit this parameter. See the `event_source` configuration block.
    #[builder(into, default)]
    #[serde(rename = "eventSource")]
    pub r#event_source: Box<Option<super::super::types::dlm::LifecyclePolicyPolicyDetailsEventSource>>,
    #[builder(into, default)]
    #[serde(rename = "parameters")]
    pub r#parameters: Box<Option<super::super::types::dlm::LifecyclePolicyPolicyDetailsParameters>>,
    /// The valid target resource types and actions a policy can manage. Specify `EBS_SNAPSHOT_MANAGEMENT` to create a lifecycle policy that manages the lifecycle of Amazon EBS snapshots. Specify `IMAGE_MANAGEMENT` to create a lifecycle policy that manages the lifecycle of EBS-backed AMIs. Specify `EVENT_BASED_POLICY` to create an event-based policy that performs specific actions when a defined event occurs in your AWS account. Default value is `EBS_SNAPSHOT_MANAGEMENT`.
    #[builder(into, default)]
    #[serde(rename = "policyType")]
    pub r#policy_type: Box<Option<String>>,
    /// The location of the resources to backup. If the source resources are located in an AWS Region, specify `CLOUD`. If the source resources are located on an Outpost in your account, specify `OUTPOST`. If you specify `OUTPOST`, Amazon Data Lifecycle Manager backs up all resources of the specified type with matching target tags across all of the Outposts in your account. Valid values are `CLOUD` and `OUTPOST`.
    #[builder(into, default)]
    #[serde(rename = "resourceLocations")]
    pub r#resource_locations: Box<Option<String>>,
    /// A list of resource types that should be targeted by the lifecycle policy. Valid values are `VOLUME` and `INSTANCE`.
    #[builder(into, default)]
    #[serde(rename = "resourceTypes")]
    pub r#resource_types: Box<Option<Vec<String>>>,
    /// See the `schedule` configuration block.
    #[builder(into, default)]
    #[serde(rename = "schedules")]
    pub r#schedules: Box<Option<Vec<super::super::types::dlm::LifecyclePolicyPolicyDetailsSchedule>>>,
    /// A map of tag keys and their values. Any resources that match the `resource_types` and are tagged with _any_ of these tags will be targeted.
    /// 
    /// > Note: You cannot have overlapping lifecycle policies that share the same `target_tags`. Pulumi is unable to detect this at plan time but it will fail during apply.
    #[builder(into, default)]
    #[serde(rename = "targetTags")]
    pub r#target_tags: Box<Option<std::collections::HashMap<String, String>>>,
}
