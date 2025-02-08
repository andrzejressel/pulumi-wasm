#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RegionBackendServiceFailoverPolicy {
    /// On failover or failback, this field indicates whether connection drain
    /// will be honored. Setting this to true has the following effect: connections
    /// to the old active pool are not drained. Connections to the new active pool
    /// use the timeout of 10 min (currently fixed). Setting to false has the
    /// following effect: both old and new connections will have a drain timeout
    /// of 10 min.
    /// This can be set to true only if the protocol is TCP.
    /// The default is false.
    #[builder(into, default)]
    #[serde(rename = "disableConnectionDrainOnFailover")]
    pub r#disable_connection_drain_on_failover: Box<Option<bool>>,
    /// This option is used only when no healthy VMs are detected in the primary
    /// and backup instance groups. When set to true, traffic is dropped. When
    /// set to false, new connections are sent across all VMs in the primary group.
    /// The default is false.
    #[builder(into, default)]
    #[serde(rename = "dropTrafficIfUnhealthy")]
    pub r#drop_traffic_if_unhealthy: Box<Option<bool>>,
    /// The value of the field must be in [0, 1]. If the ratio of the healthy
    /// VMs in the primary backend is at or below this number, traffic arriving
    /// at the load-balanced IP will be directed to the failover backend.
    /// In case where 'failoverRatio' is not set or all the VMs in the backup
    /// backend are unhealthy, the traffic will be directed back to the primary
    /// backend in the "force" mode, where traffic will be spread to the healthy
    /// VMs with the best effort, or to all VMs when no VM is healthy.
    /// This field is only used with l4 load balancing.
    #[builder(into, default)]
    #[serde(rename = "failoverRatio")]
    pub r#failover_ratio: Box<Option<f64>>,
}
