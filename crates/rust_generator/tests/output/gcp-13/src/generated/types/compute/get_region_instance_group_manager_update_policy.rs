#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetRegionInstanceGroupManagerUpdatePolicy {
    /// The instance redistribution policy for regional managed instance groups. Valid values are: "PROACTIVE", "NONE". If PROACTIVE (default), the group attempts to maintain an even distribution of VM instances across zones in the region. If NONE, proactive redistribution is disabled.
    #[builder(into)]
    #[serde(rename = "instanceRedistributionType")]
    pub r#instance_redistribution_type: Box<String>,
    /// Specifies a fixed number of VM instances. This must be a positive integer. Conflicts with max_surge_percent. Both cannot be 0
    #[builder(into)]
    #[serde(rename = "maxSurgeFixed")]
    pub r#max_surge_fixed: Box<i32>,
    /// Specifies a percentage of instances between 0 to 100%, inclusive. For example, specify 80 for 80%. Conflicts with max_surge_fixed.
    #[builder(into)]
    #[serde(rename = "maxSurgePercent")]
    pub r#max_surge_percent: Box<i32>,
    /// Specifies a fixed number of VM instances. This must be a positive integer.
    #[builder(into)]
    #[serde(rename = "maxUnavailableFixed")]
    pub r#max_unavailable_fixed: Box<i32>,
    /// Specifies a percentage of instances between 0 to 100%, inclusive. For example, specify 80 for 80%.
    #[builder(into)]
    #[serde(rename = "maxUnavailablePercent")]
    pub r#max_unavailable_percent: Box<i32>,
    /// Minimum number of seconds to wait for after a newly created instance becomes available. This value must be from range [0, 3600].
    #[builder(into)]
    #[serde(rename = "minReadySec")]
    pub r#min_ready_sec: Box<i32>,
    /// Minimal action to be taken on an instance. You can specify either NONE to forbid any actions, REFRESH to update without stopping instances, RESTART to restart existing instances or REPLACE to delete and create new instances from the target template. If you specify a REFRESH, the Updater will attempt to perform that action only. However, if the Updater determines that the minimal action you specify is not enough to perform the update, it might perform a more disruptive action.
    #[builder(into)]
    #[serde(rename = "minimalAction")]
    pub r#minimal_action: Box<String>,
    /// Most disruptive action that is allowed to be taken on an instance. You can specify either NONE to forbid any actions, REFRESH to allow actions that do not need instance restart, RESTART to allow actions that can be applied without instance replacing or REPLACE to allow all possible actions. If the Updater determines that the minimal update action needed is more disruptive than most disruptive allowed action you specify it will not perform the update at all.
    #[builder(into)]
    #[serde(rename = "mostDisruptiveAllowedAction")]
    pub r#most_disruptive_allowed_action: Box<String>,
    /// The instance replacement method for regional managed instance groups. Valid values are: "RECREATE", "SUBSTITUTE". If SUBSTITUTE (default), the group replaces VM instances with new instances that have randomly generated names. If RECREATE, instance names are preserved.  You must also set max_unavailable_fixed or max_unavailable_percent to be greater than 0.
    #[builder(into)]
    #[serde(rename = "replacementMethod")]
    pub r#replacement_method: Box<String>,
    /// The type of update process. You can specify either PROACTIVE so that the instance group manager proactively executes actions in order to bring instances to their target versions or OPPORTUNISTIC so that no action is proactively executed but the update will be performed as part of other actions (for example, resizes or recreateInstances calls).
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
