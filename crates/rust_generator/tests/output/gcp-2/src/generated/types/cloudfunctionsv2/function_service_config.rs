#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct FunctionServiceConfig {
    /// Whether 100% of traffic is routed to the latest revision. Defaults to true.
    #[builder(into, default)]
    #[serde(rename = "allTrafficOnLatestRevision")]
    pub r#all_traffic_on_latest_revision: Box<Option<bool>>,
    /// The number of CPUs used in a single container instance. Default value is calculated from available memory.
    #[builder(into, default)]
    #[serde(rename = "availableCpu")]
    pub r#available_cpu: Box<Option<String>>,
    /// The amount of memory available for a function.
    /// Defaults to 256M. Supported units are k, M, G, Mi, Gi. If no unit is
    /// supplied the value is interpreted as bytes.
    #[builder(into, default)]
    #[serde(rename = "availableMemory")]
    pub r#available_memory: Box<Option<String>>,
    /// Environment variables that shall be available during function execution.
    #[builder(into, default)]
    #[serde(rename = "environmentVariables")]
    pub r#environment_variables: Box<Option<std::collections::HashMap<String, String>>>,
    /// (Output)
    /// URIs of the Service deployed
    #[builder(into, default)]
    #[serde(rename = "gcfUri")]
    pub r#gcf_uri: Box<Option<String>>,
    /// Available ingress settings. Defaults to "ALLOW_ALL" if unspecified.
    /// Default value is `ALLOW_ALL`.
    /// Possible values are: `ALLOW_ALL`, `ALLOW_INTERNAL_ONLY`, `ALLOW_INTERNAL_AND_GCLB`.
    #[builder(into, default)]
    #[serde(rename = "ingressSettings")]
    pub r#ingress_settings: Box<Option<String>>,
    /// The limit on the maximum number of function instances that may coexist at a
    /// given time.
    #[builder(into, default)]
    #[serde(rename = "maxInstanceCount")]
    pub r#max_instance_count: Box<Option<i32>>,
    /// Sets the maximum number of concurrent requests that each instance can receive. Defaults to 1.
    #[builder(into, default)]
    #[serde(rename = "maxInstanceRequestConcurrency")]
    pub r#max_instance_request_concurrency: Box<Option<i32>>,
    /// The limit on the minimum number of function instances that may coexist at a
    /// given time.
    #[builder(into, default)]
    #[serde(rename = "minInstanceCount")]
    pub r#min_instance_count: Box<Option<i32>>,
    /// Secret environment variables configuration.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "secretEnvironmentVariables")]
    pub r#secret_environment_variables: Box<Option<Vec<super::super::types::cloudfunctionsv2::FunctionServiceConfigSecretEnvironmentVariable>>>,
    /// Secret volumes configuration.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "secretVolumes")]
    pub r#secret_volumes: Box<Option<Vec<super::super::types::cloudfunctionsv2::FunctionServiceConfigSecretVolume>>>,
    /// Name of the service associated with a Function.
    #[builder(into, default)]
    #[serde(rename = "service")]
    pub r#service: Box<Option<String>>,
    /// The email of the service account for this function.
    #[builder(into, default)]
    #[serde(rename = "serviceAccountEmail")]
    pub r#service_account_email: Box<Option<String>>,
    /// The function execution timeout. Execution is considered failed and
    /// can be terminated if the function is not completed at the end of the
    /// timeout period. Defaults to 60 seconds.
    #[builder(into, default)]
    #[serde(rename = "timeoutSeconds")]
    pub r#timeout_seconds: Box<Option<i32>>,
    /// (Output)
    /// URI of the Service deployed.
    #[builder(into, default)]
    #[serde(rename = "uri")]
    pub r#uri: Box<Option<String>>,
    /// The Serverless VPC Access connector that this cloud function can connect to.
    #[builder(into, default)]
    #[serde(rename = "vpcConnector")]
    pub r#vpc_connector: Box<Option<String>>,
    /// Available egress settings.
    /// Possible values are: `VPC_CONNECTOR_EGRESS_SETTINGS_UNSPECIFIED`, `PRIVATE_RANGES_ONLY`, `ALL_TRAFFIC`.
    #[builder(into, default)]
    #[serde(rename = "vpcConnectorEgressSettings")]
    pub r#vpc_connector_egress_settings: Box<Option<String>>,
}
