#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetFunctionServiceConfig {
    /// Whether 100% of traffic is routed to the latest revision. Defaults to true.
    #[builder(into)]
    #[serde(rename = "allTrafficOnLatestRevision")]
    pub r#all_traffic_on_latest_revision: Box<bool>,
    /// The number of CPUs used in a single container instance. Default value is calculated from available memory.
    #[builder(into)]
    #[serde(rename = "availableCpu")]
    pub r#available_cpu: Box<String>,
    /// The amount of memory available for a function.
    /// Defaults to 256M. Supported units are k, M, G, Mi, Gi. If no unit is
    /// supplied the value is interpreted as bytes.
    #[builder(into)]
    #[serde(rename = "availableMemory")]
    pub r#available_memory: Box<String>,
    /// Environment variables that shall be available during function execution.
    #[builder(into)]
    #[serde(rename = "environmentVariables")]
    pub r#environment_variables: Box<std::collections::HashMap<String, String>>,
    /// URIs of the Service deployed
    #[builder(into)]
    #[serde(rename = "gcfUri")]
    pub r#gcf_uri: Box<String>,
    /// Available ingress settings. Defaults to "ALLOW_ALL" if unspecified. Default value: "ALLOW_ALL" Possible values: ["ALLOW_ALL", "ALLOW_INTERNAL_ONLY", "ALLOW_INTERNAL_AND_GCLB"]
    #[builder(into)]
    #[serde(rename = "ingressSettings")]
    pub r#ingress_settings: Box<String>,
    /// The limit on the maximum number of function instances that may coexist at a
    /// given time.
    #[builder(into)]
    #[serde(rename = "maxInstanceCount")]
    pub r#max_instance_count: Box<i32>,
    /// Sets the maximum number of concurrent requests that each instance can receive. Defaults to 1.
    #[builder(into)]
    #[serde(rename = "maxInstanceRequestConcurrency")]
    pub r#max_instance_request_concurrency: Box<i32>,
    /// The limit on the minimum number of function instances that may coexist at a
    /// given time.
    #[builder(into)]
    #[serde(rename = "minInstanceCount")]
    pub r#min_instance_count: Box<i32>,
    /// Secret environment variables configuration.
    #[builder(into)]
    #[serde(rename = "secretEnvironmentVariables")]
    pub r#secret_environment_variables: Box<Vec<super::super::types::cloudfunctionsv2::GetFunctionServiceConfigSecretEnvironmentVariable>>,
    /// Secret volumes configuration.
    #[builder(into)]
    #[serde(rename = "secretVolumes")]
    pub r#secret_volumes: Box<Vec<super::super::types::cloudfunctionsv2::GetFunctionServiceConfigSecretVolume>>,
    /// Name of the service associated with a Function.
    #[builder(into)]
    #[serde(rename = "service")]
    pub r#service: Box<String>,
    /// The email of the service account for this function.
    #[builder(into)]
    #[serde(rename = "serviceAccountEmail")]
    pub r#service_account_email: Box<String>,
    /// The function execution timeout. Execution is considered failed and
    /// can be terminated if the function is not completed at the end of the
    /// timeout period. Defaults to 60 seconds.
    #[builder(into)]
    #[serde(rename = "timeoutSeconds")]
    pub r#timeout_seconds: Box<i32>,
    /// URI of the Service deployed.
    #[builder(into)]
    #[serde(rename = "uri")]
    pub r#uri: Box<String>,
    /// The Serverless VPC Access connector that this cloud function can connect to.
    #[builder(into)]
    #[serde(rename = "vpcConnector")]
    pub r#vpc_connector: Box<String>,
    /// Available egress settings. Possible values: ["VPC_CONNECTOR_EGRESS_SETTINGS_UNSPECIFIED", "PRIVATE_RANGES_ONLY", "ALL_TRAFFIC"]
    #[builder(into)]
    #[serde(rename = "vpcConnectorEgressSettings")]
    pub r#vpc_connector_egress_settings: Box<String>,
}
