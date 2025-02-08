#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct FlexibleAppVersionEndpointsApiService {
    /// Endpoints service configuration ID as specified by the Service Management API. For example "2016-09-19r1".
    /// By default, the rollout strategy for Endpoints is "FIXED". This means that Endpoints starts up with a particular configuration ID.
    /// When a new configuration is rolled out, Endpoints must be given the new configuration ID. The configId field is used to give the configuration ID
    /// and is required in this case.
    /// Endpoints also has a rollout strategy called "MANAGED". When using this, Endpoints fetches the latest configuration and does not need
    /// the configuration ID. In this case, configId must be omitted.
    #[builder(into, default)]
    #[serde(rename = "configId")]
    pub r#config_id: Box<Option<String>>,
    /// Enable or disable trace sampling. By default, this is set to false for enabled.
    #[builder(into, default)]
    #[serde(rename = "disableTraceSampling")]
    pub r#disable_trace_sampling: Box<Option<bool>>,
    /// Endpoints service name which is the name of the "service" resource in the Service Management API.
    /// For example "myapi.endpoints.myproject.cloud.goog"
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Endpoints rollout strategy. If FIXED, configId must be specified. If MANAGED, configId must be omitted.
    /// Default value is `FIXED`.
    /// Possible values are: `FIXED`, `MANAGED`.
    #[builder(into, default)]
    #[serde(rename = "rolloutStrategy")]
    pub r#rollout_strategy: Box<Option<String>>,
}
