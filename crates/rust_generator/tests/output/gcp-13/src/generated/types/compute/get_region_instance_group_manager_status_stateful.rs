#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetRegionInstanceGroupManagerStatusStateful {
    /// A bit indicating whether the managed instance group has stateful configuration, that is, if you have configured any items in a stateful policy or in per-instance configs. The group might report that it has no stateful config even when there is still some preserved state on a managed instance, for example, if you have deleted all PICs but not yet applied those deletions.
    #[builder(into)]
    #[serde(rename = "hasStatefulConfig")]
    pub r#has_stateful_config: Box<bool>,
    /// Status of per-instance configs on the instances.
    #[builder(into)]
    #[serde(rename = "perInstanceConfigs")]
    pub r#per_instance_configs: Box<Vec<super::super::types::compute::GetRegionInstanceGroupManagerStatusStatefulPerInstanceConfig>>,
}
