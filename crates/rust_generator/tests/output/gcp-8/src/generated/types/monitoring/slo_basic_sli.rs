#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct SloBasicSli {
    /// Availability based SLI, dervied from count of requests made to this service that return successfully.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "availability")]
    pub r#availability: Box<Option<super::super::types::monitoring::SloBasicSliAvailability>>,
    /// Parameters for a latency threshold SLI.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "latency")]
    pub r#latency: Box<Option<super::super::types::monitoring::SloBasicSliLatency>>,
    /// An optional set of locations to which this SLI is relevant.
    /// Telemetry from other locations will not be used to calculate
    /// performance for this SLI. If omitted, this SLI applies to all
    /// locations in which the Service has activity. For service types
    /// that don't support breaking down by location, setting this
    /// field will result in an error.
    #[builder(into, default)]
    #[serde(rename = "locations")]
    pub r#locations: Box<Option<Vec<String>>>,
    /// An optional set of RPCs to which this SLI is relevant.
    /// Telemetry from other methods will not be used to calculate
    /// performance for this SLI. If omitted, this SLI applies to all
    /// the Service's methods. For service types that don't support
    /// breaking down by method, setting this field will result in an
    /// error.
    #[builder(into, default)]
    #[serde(rename = "methods")]
    pub r#methods: Box<Option<Vec<String>>>,
    /// The set of API versions to which this SLI is relevant.
    /// Telemetry from other API versions will not be used to
    /// calculate performance for this SLI. If omitted,
    /// this SLI applies to all API versions. For service types
    /// that don't support breaking down by version, setting this
    /// field will result in an error.
    #[builder(into, default)]
    #[serde(rename = "versions")]
    pub r#versions: Box<Option<Vec<String>>>,
}
