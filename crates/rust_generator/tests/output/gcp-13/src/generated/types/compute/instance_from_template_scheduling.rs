#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct InstanceFromTemplateScheduling {
    /// Specifies if the instance should be restarted if it was terminated by Compute Engine (not a user).
    #[builder(into, default)]
    #[serde(rename = "automaticRestart")]
    pub r#automatic_restart: Box<Option<bool>>,
    /// Specify the time in seconds for host error detection, the value must be within the range of [90, 330] with the increment of 30, if unset, the default behavior of host error recovery will be used.
    #[builder(into, default)]
    #[serde(rename = "hostErrorTimeoutSeconds")]
    pub r#host_error_timeout_seconds: Box<Option<i32>>,
    /// Specifies the action GCE should take when SPOT VM is preempted.
    #[builder(into, default)]
    #[serde(rename = "instanceTerminationAction")]
    pub r#instance_termination_action: Box<Option<String>>,
    /// Specifies the maximum amount of time a Local Ssd Vm should wait while
    ///   recovery of the Local Ssd state is attempted. Its value should be in
    ///   between 0 and 168 hours with hour granularity and the default value being 1
    ///   hour.
    #[builder(into, default)]
    #[serde(rename = "localSsdRecoveryTimeout")]
    pub r#local_ssd_recovery_timeout: Box<Option<super::super::types::compute::InstanceFromTemplateSchedulingLocalSsdRecoveryTimeout>>,
    /// Specifies the frequency of planned maintenance events. The accepted values are: PERIODIC
    #[builder(into, default)]
    #[serde(rename = "maintenanceInterval")]
    pub r#maintenance_interval: Box<Option<String>>,
    /// The timeout for new network connections to hosts.
    #[builder(into, default)]
    #[serde(rename = "maxRunDuration")]
    pub r#max_run_duration: Box<Option<super::super::types::compute::InstanceFromTemplateSchedulingMaxRunDuration>>,
    #[builder(into, default)]
    #[serde(rename = "minNodeCpus")]
    pub r#min_node_cpus: Box<Option<i32>>,
    /// Specifies node affinities or anti-affinities to determine which sole-tenant nodes your instances and managed instance groups will use as host systems.
    #[builder(into, default)]
    #[serde(rename = "nodeAffinities")]
    pub r#node_affinities: Box<Option<Vec<super::super::types::compute::InstanceFromTemplateSchedulingNodeAffinity>>>,
    /// Describes maintenance behavior for the instance. One of MIGRATE or TERMINATE,
    #[builder(into, default)]
    #[serde(rename = "onHostMaintenance")]
    pub r#on_host_maintenance: Box<Option<String>>,
    /// Defines the behaviour for instances with the instance_termination_action.
    #[builder(into, default)]
    #[serde(rename = "onInstanceStopAction")]
    pub r#on_instance_stop_action: Box<Option<super::super::types::compute::InstanceFromTemplateSchedulingOnInstanceStopAction>>,
    /// Whether the instance is preemptible.
    #[builder(into, default)]
    #[serde(rename = "preemptible")]
    pub r#preemptible: Box<Option<bool>>,
    /// Whether the instance is spot. If this is set as SPOT.
    #[builder(into, default)]
    #[serde(rename = "provisioningModel")]
    pub r#provisioning_model: Box<Option<String>>,
}
