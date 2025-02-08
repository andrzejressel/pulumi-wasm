#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct RegionBackendServiceBackend {
    /// Specifies the balancing mode for this backend.
    /// See the [Backend Services Overview](https://cloud.google.com/load-balancing/docs/backend-service#balancing-mode)
    /// for an explanation of load balancing modes.
    /// Default value is `UTILIZATION`.
    /// Possible values are: `UTILIZATION`, `RATE`, `CONNECTION`.
    #[builder(into, default)]
    #[serde(rename = "balancingMode")]
    pub r#balancing_mode: Box<Option<String>>,
    /// A multiplier applied to the group's maximum servicing capacity
    /// (based on UTILIZATION, RATE or CONNECTION).
    /// ~>**NOTE**: This field cannot be set for
    /// INTERNAL region backend services (default loadBalancingScheme),
    /// but is required for non-INTERNAL backend service. The total
    /// capacity_scaler for all backends must be non-zero.
    /// A setting of 0 means the group is completely drained, offering
    /// 0% of its available Capacity. Valid range is [0.0,1.0].
    #[builder(into, default)]
    #[serde(rename = "capacityScaler")]
    pub r#capacity_scaler: Box<Option<f64>>,
    /// An optional description of this resource.
    /// Provide this property when you create the resource.
    #[builder(into, default)]
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    /// This field designates whether this is a failover backend. More
    /// than one failover backend can be configured for a given RegionBackendService.
    #[builder(into, default)]
    #[serde(rename = "failover")]
    pub r#failover: Box<Option<bool>>,
    /// The fully-qualified URL of an Instance Group or Network Endpoint
    /// Group resource. In case of instance group this defines the list
    /// of instances that serve traffic. Member virtual machine
    /// instances from each instance group must live in the same zone as
    /// the instance group itself. No two backends in a backend service
    /// are allowed to use same Instance Group resource.
    /// For Network Endpoint Groups this defines list of endpoints. All
    /// endpoints of Network Endpoint Group must be hosted on instances
    /// located in the same zone as the Network Endpoint Group.
    /// Backend services cannot mix Instance Group and
    /// Network Endpoint Group backends.
    /// When the `load_balancing_scheme` is INTERNAL, only instance groups
    /// are supported.
    /// Note that you must specify an Instance Group or Network Endpoint
    /// Group resource using the fully-qualified URL, rather than a
    /// partial URL.
    #[builder(into)]
    #[serde(rename = "group")]
    pub r#group: Box<String>,
    /// The max number of simultaneous connections for the group. Can
    /// be used with either CONNECTION or UTILIZATION balancing modes.
    /// Cannot be set for INTERNAL backend services.
    /// For CONNECTION mode, either maxConnections or one
    /// of maxConnectionsPerInstance or maxConnectionsPerEndpoint,
    /// as appropriate for group type, must be set.
    #[builder(into, default)]
    #[serde(rename = "maxConnections")]
    pub r#max_connections: Box<Option<i32>>,
    /// The max number of simultaneous connections that a single backend
    /// network endpoint can handle. Cannot be set
    /// for INTERNAL backend services.
    /// This is used to calculate the capacity of the group. Can be
    /// used in either CONNECTION or UTILIZATION balancing modes. For
    /// CONNECTION mode, either maxConnections or
    /// maxConnectionsPerEndpoint must be set.
    #[builder(into, default)]
    #[serde(rename = "maxConnectionsPerEndpoint")]
    pub r#max_connections_per_endpoint: Box<Option<i32>>,
    /// The max number of simultaneous connections that a single
    /// backend instance can handle. Cannot be set for INTERNAL backend
    /// services.
    /// This is used to calculate the capacity of the group.
    /// Can be used in either CONNECTION or UTILIZATION balancing modes.
    /// For CONNECTION mode, either maxConnections or
    /// maxConnectionsPerInstance must be set.
    #[builder(into, default)]
    #[serde(rename = "maxConnectionsPerInstance")]
    pub r#max_connections_per_instance: Box<Option<i32>>,
    /// The max requests per second (RPS) of the group. Cannot be set
    /// for INTERNAL backend services.
    /// Can be used with either RATE or UTILIZATION balancing modes,
    /// but required if RATE mode. Either maxRate or one
    /// of maxRatePerInstance or maxRatePerEndpoint, as appropriate for
    /// group type, must be set.
    #[builder(into, default)]
    #[serde(rename = "maxRate")]
    pub r#max_rate: Box<Option<i32>>,
    /// The max requests per second (RPS) that a single backend network
    /// endpoint can handle. This is used to calculate the capacity of
    /// the group. Can be used in either balancing mode. For RATE mode,
    /// either maxRate or maxRatePerEndpoint must be set. Cannot be set
    /// for INTERNAL backend services.
    #[builder(into, default)]
    #[serde(rename = "maxRatePerEndpoint")]
    pub r#max_rate_per_endpoint: Box<Option<f64>>,
    /// The max requests per second (RPS) that a single backend
    /// instance can handle. This is used to calculate the capacity of
    /// the group. Can be used in either balancing mode. For RATE mode,
    /// either maxRate or maxRatePerInstance must be set. Cannot be set
    /// for INTERNAL backend services.
    #[builder(into, default)]
    #[serde(rename = "maxRatePerInstance")]
    pub r#max_rate_per_instance: Box<Option<f64>>,
    /// Used when balancingMode is UTILIZATION. This ratio defines the
    /// CPU utilization target for the group. Valid range is [0.0, 1.0].
    /// Cannot be set for INTERNAL backend services.
    #[builder(into, default)]
    #[serde(rename = "maxUtilization")]
    pub r#max_utilization: Box<Option<f64>>,
}
