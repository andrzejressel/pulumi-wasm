#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct InstanceScheduling {
    /// Specifies if the instance should be
    /// restarted if it was terminated by Compute Engine (not a user).
    /// Defaults to true.
    #[builder(into, default)]
    #[serde(rename = "automaticRestart")]
    pub r#automatic_restart: Box<Option<bool>>,
    /// Specifies the time in seconds for host error detection, the value must be within the range of [90, 330] with the increment of 30, if unset, the default behavior of host error recovery will be used.
    #[builder(into, default)]
    #[serde(rename = "hostErrorTimeoutSeconds")]
    pub r#host_error_timeout_seconds: Box<Option<i32>>,
    /// Describe the type of termination action for VM. Can be `STOP` or `DELETE`.  Read more on [here](https://cloud.google.com/compute/docs/instances/create-use-spot)
    #[builder(into, default)]
    #[serde(rename = "instanceTerminationAction")]
    pub r#instance_termination_action: Box<Option<String>>,
    /// Specifies the maximum amount of time a Local Ssd Vm should wait while
    ///   recovery of the Local Ssd state is attempted. Its value should be in
    ///   between 0 and 168 hours with hour granularity and the default value being 1
    ///   hour.
    #[builder(into, default)]
    #[serde(rename = "localSsdRecoveryTimeout")]
    pub r#local_ssd_recovery_timeout: Box<Option<super::super::types::compute::InstanceSchedulingLocalSsdRecoveryTimeout>>,
    /// Specifies the frequency of planned maintenance events. The accepted values are: `PERIODIC`.
    #[builder(into, default)]
    #[serde(rename = "maintenanceInterval")]
    pub r#maintenance_interval: Box<Option<String>>,
    /// The duration of the instance. Instance will run and be terminated after then, the termination action could be defined in `instance_termination_action`. Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "maxRunDuration")]
    pub r#max_run_duration: Box<Option<super::super::types::compute::InstanceSchedulingMaxRunDuration>>,
    /// The minimum number of virtual CPUs this instance will consume when running on a sole-tenant node.
    #[builder(into, default)]
    #[serde(rename = "minNodeCpus")]
    pub r#min_node_cpus: Box<Option<i32>>,
    /// Specifies node affinities or anti-affinities
    /// to determine which sole-tenant nodes your instances and managed instance
    /// groups will use as host systems. Read more on sole-tenant node creation
    /// [here](https://cloud.google.com/compute/docs/nodes/create-nodes).
    /// Structure documented below.
    #[builder(into, default)]
    #[serde(rename = "nodeAffinities")]
    pub r#node_affinities: Box<Option<Vec<super::super::types::compute::InstanceSchedulingNodeAffinity>>>,
    /// Describes maintenance behavior for the
    /// instance. Can be MIGRATE or TERMINATE, for more info, read
    /// [here](https://cloud.google.com/compute/docs/instances/setting-instance-scheduling-options).
    #[builder(into, default)]
    #[serde(rename = "onHostMaintenance")]
    pub r#on_host_maintenance: Box<Option<String>>,
    /// Specifies the action to be performed when the instance is terminated using `max_run_duration` and `STOP` `instance_termination_action`. Only support `true` `discard_local_ssd` at this point. Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "onInstanceStopAction")]
    pub r#on_instance_stop_action: Box<Option<super::super::types::compute::InstanceSchedulingOnInstanceStopAction>>,
    /// Specifies if the instance is preemptible.
    /// If this field is set to true, then `automatic_restart` must be
    /// set to false.  Defaults to false.
    #[builder(into, default)]
    #[serde(rename = "preemptible")]
    pub r#preemptible: Box<Option<bool>>,
    /// Describe the type of preemptible VM. This field accepts the value `STANDARD` or `SPOT`. If the value is `STANDARD`, there will be no discount. If this   is set to `SPOT`,
    /// `preemptible` should be `true` and `automatic_restart` should be
    /// `false`. For more info about
    /// `SPOT`, read [here](https://cloud.google.com/compute/docs/instances/spot)
    #[builder(into, default)]
    #[serde(rename = "provisioningModel")]
    pub r#provisioning_model: Box<Option<String>>,
}
