#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct BackendServiceBackend {
    /// Specifies the balancing mode for this backend.
    /// For global HTTP(S) or TCP/SSL load balancing, the default is
    /// UTILIZATION. Valid values are UTILIZATION, RATE (for HTTP(S))
    /// and CONNECTION (for TCP/SSL).
    /// See the [Backend Services Overview](https://cloud.google.com/load-balancing/docs/backend-service#balancing-mode)
    /// for an explanation of load balancing modes.
    /// Default value is `UTILIZATION`.
    /// Possible values are: `UTILIZATION`, `RATE`, `CONNECTION`.
    #[builder(into, default)]
    #[serde(rename = "balancingMode")]
    pub r#balancing_mode: Box<Option<String>>,
    /// A multiplier applied to the group's maximum servicing capacity
    /// (based on UTILIZATION, RATE or CONNECTION).
    /// Default value is 1, which means the group will serve up to 100%
    /// of its configured capacity (depending on balancingMode). A
    /// setting of 0 means the group is completely drained, offering
    /// 0% of its available Capacity. Valid range is [0.0,1.0].
    #[builder(into, default)]
    #[serde(rename = "capacityScaler")]
    pub r#capacity_scaler: Box<Option<f64>>,
    /// An optional description of this resource.
    /// Provide this property when you create the resource.
    #[builder(into, default)]
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
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
    /// Note that you must specify an Instance Group or Network Endpoint
    /// Group resource using the fully-qualified URL, rather than a
    /// partial URL.
    #[builder(into)]
    #[serde(rename = "group")]
    pub r#group: Box<String>,
    /// The max number of simultaneous connections for the group. Can
    /// be used with either CONNECTION or UTILIZATION balancing modes.
    /// For CONNECTION mode, either maxConnections or one
    /// of maxConnectionsPerInstance or maxConnectionsPerEndpoint,
    /// as appropriate for group type, must be set.
    #[builder(into, default)]
    #[serde(rename = "maxConnections")]
    pub r#max_connections: Box<Option<i32>>,
    /// The max number of simultaneous connections that a single backend
    /// network endpoint can handle. This is used to calculate the
    /// capacity of the group. Can be used in either CONNECTION or
    /// UTILIZATION balancing modes.
    /// For CONNECTION mode, either
    /// maxConnections or maxConnectionsPerEndpoint must be set.
    #[builder(into, default)]
    #[serde(rename = "maxConnectionsPerEndpoint")]
    pub r#max_connections_per_endpoint: Box<Option<i32>>,
    /// The max number of simultaneous connections that a single
    /// backend instance can handle. This is used to calculate the
    /// capacity of the group. Can be used in either CONNECTION or
    /// UTILIZATION balancing modes.
    /// For CONNECTION mode, either maxConnections or
    /// maxConnectionsPerInstance must be set.
    #[builder(into, default)]
    #[serde(rename = "maxConnectionsPerInstance")]
    pub r#max_connections_per_instance: Box<Option<i32>>,
    /// The max requests per second (RPS) of the group.
    /// Can be used with either RATE or UTILIZATION balancing modes,
    /// but required if RATE mode. For RATE mode, either maxRate or one
    /// of maxRatePerInstance or maxRatePerEndpoint, as appropriate for
    /// group type, must be set.
    #[builder(into, default)]
    #[serde(rename = "maxRate")]
    pub r#max_rate: Box<Option<i32>>,
    /// The max requests per second (RPS) that a single backend network
    /// endpoint can handle. This is used to calculate the capacity of
    /// the group. Can be used in either balancing mode. For RATE mode,
    /// either maxRate or maxRatePerEndpoint must be set.
    #[builder(into, default)]
    #[serde(rename = "maxRatePerEndpoint")]
    pub r#max_rate_per_endpoint: Box<Option<f64>>,
    /// The max requests per second (RPS) that a single backend
    /// instance can handle. This is used to calculate the capacity of
    /// the group. Can be used in either balancing mode. For RATE mode,
    /// either maxRate or maxRatePerInstance must be set.
    #[builder(into, default)]
    #[serde(rename = "maxRatePerInstance")]
    pub r#max_rate_per_instance: Box<Option<f64>>,
    /// Used when balancingMode is UTILIZATION. This ratio defines the
    /// CPU utilization target for the group. Valid range is [0.0, 1.0].
    #[builder(into, default)]
    #[serde(rename = "maxUtilization")]
    pub r#max_utilization: Box<Option<f64>>,
}
