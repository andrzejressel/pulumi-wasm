#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetLinuxFunctionAppSiteConfig {
    /// If this Linux Web App is Always On enabled.
    #[builder(into)]
    #[serde(rename = "alwaysOn")]
    pub r#always_on: Box<bool>,
    /// The URL of the API definition that describes this Linux Function App.
    #[builder(into)]
    #[serde(rename = "apiDefinitionUrl")]
    pub r#api_definition_url: Box<String>,
    /// The ID of the API Management API for this Linux Function App.
    #[builder(into)]
    #[serde(rename = "apiManagementApiId")]
    pub r#api_management_api_id: Box<String>,
    /// The App command line that is launched.
    #[builder(into)]
    #[serde(rename = "appCommandLine")]
    pub r#app_command_line: Box<String>,
    /// The number of workers this function app can scale out to.
    #[builder(into)]
    #[serde(rename = "appScaleLimit")]
    pub r#app_scale_limit: Box<i32>,
    /// An `app_service_logs` block as defined above.
    #[builder(into)]
    #[serde(rename = "appServiceLogs")]
    pub r#app_service_logs: Box<Vec<super::super::types::appservice::GetLinuxFunctionAppSiteConfigAppServiceLog>>,
    /// The Connection String that links the Linux Function App to Application Insights.
    #[builder(into)]
    #[serde(rename = "applicationInsightsConnectionString")]
    pub r#application_insights_connection_string: Box<String>,
    /// The Instrumentation Key that connects the Linux Function App to Application Insights.
    #[builder(into)]
    #[serde(rename = "applicationInsightsKey")]
    pub r#application_insights_key: Box<String>,
    /// An `application_stack` block as defined above.
    #[builder(into)]
    #[serde(rename = "applicationStacks")]
    pub r#application_stacks: Box<Vec<super::super::types::appservice::GetLinuxFunctionAppSiteConfigApplicationStack>>,
    /// The Client ID of the Managed Service Identity that is used for connections to the Azure Container Registry.
    #[builder(into)]
    #[serde(rename = "containerRegistryManagedIdentityClientId")]
    pub r#container_registry_managed_identity_client_id: Box<String>,
    /// Do connections for Azure Container Registry use Managed Identity?
    #[builder(into)]
    #[serde(rename = "containerRegistryUseManagedIdentity")]
    pub r#container_registry_use_managed_identity: Box<bool>,
    /// A `cors` block as defined above.
    #[builder(into)]
    #[serde(rename = "cors")]
    pub r#cors: Box<Vec<super::super::types::appservice::GetLinuxFunctionAppSiteConfigCor>>,
    /// A list of Default Documents for the Linux Web App.
    #[builder(into)]
    #[serde(rename = "defaultDocuments")]
    pub r#default_documents: Box<Vec<String>>,
    #[builder(into)]
    #[serde(rename = "detailedErrorLoggingEnabled")]
    pub r#detailed_error_logging_enabled: Box<bool>,
    /// The number of minimum instances for this Linux Function App.
    #[builder(into)]
    #[serde(rename = "elasticInstanceMinimum")]
    pub r#elastic_instance_minimum: Box<i32>,
    /// State of FTP / FTPS service for this function app.
    #[builder(into)]
    #[serde(rename = "ftpsState")]
    pub r#ftps_state: Box<String>,
    /// The amount of time in minutes that a node can be unhealthy before being removed from the load balancer.
    #[builder(into)]
    #[serde(rename = "healthCheckEvictionTimeInMin")]
    pub r#health_check_eviction_time_in_min: Box<i32>,
    /// The path that is checked for this function app health.
    #[builder(into)]
    #[serde(rename = "healthCheckPath")]
    pub r#health_check_path: Box<String>,
    /// Is the HTTP2 protocol enabled?
    #[builder(into)]
    #[serde(rename = "http2Enabled")]
    pub r#http_2_enabled: Box<bool>,
    /// The Default action for traffic that does not match any `ip_restriction` rule.
    #[builder(into)]
    #[serde(rename = "ipRestrictionDefaultAction")]
    pub r#ip_restriction_default_action: Box<String>,
    /// One or more `ip_restriction` blocks as defined above.
    #[builder(into)]
    #[serde(rename = "ipRestrictions")]
    pub r#ip_restrictions: Box<Vec<super::super::types::appservice::GetLinuxFunctionAppSiteConfigIpRestriction>>,
    #[builder(into)]
    #[serde(rename = "linuxFxVersion")]
    pub r#linux_fx_version: Box<String>,
    /// The Site load balancing mode.
    #[builder(into)]
    #[serde(rename = "loadBalancingMode")]
    pub r#load_balancing_mode: Box<String>,
    /// Managed pipeline mode.
    #[builder(into)]
    #[serde(rename = "managedPipelineMode")]
    pub r#managed_pipeline_mode: Box<String>,
    /// The minimum version of TLS required for SSL requests.
    #[builder(into)]
    #[serde(rename = "minimumTlsVersion")]
    pub r#minimum_tls_version: Box<String>,
    /// The number of pre-warmed instances for this function app.
    #[builder(into)]
    #[serde(rename = "preWarmedInstanceCount")]
    pub r#pre_warmed_instance_count: Box<i32>,
    /// Is Remote Debugging enabled?
    #[builder(into)]
    #[serde(rename = "remoteDebuggingEnabled")]
    pub r#remote_debugging_enabled: Box<bool>,
    /// The Remote Debugging Version.
    #[builder(into)]
    #[serde(rename = "remoteDebuggingVersion")]
    pub r#remote_debugging_version: Box<String>,
    /// Is Scale Monitoring of the Functions Runtime enabled?
    #[builder(into)]
    #[serde(rename = "runtimeScaleMonitoringEnabled")]
    pub r#runtime_scale_monitoring_enabled: Box<bool>,
    /// The Default action for traffic that does not match any `scm_ip_restriction` rule.
    #[builder(into, default)]
    #[serde(rename = "scmIpRestrictionDefaultAction")]
    pub r#scm_ip_restriction_default_action: Box<Option<String>>,
    /// One or more `scm_ip_restriction` blocks as defined above.
    #[builder(into)]
    #[serde(rename = "scmIpRestrictions")]
    pub r#scm_ip_restrictions: Box<Vec<super::super::types::appservice::GetLinuxFunctionAppSiteConfigScmIpRestriction>>,
    /// The minimum version of TLS for SSL requests to the SCM site.
    #[builder(into)]
    #[serde(rename = "scmMinimumTlsVersion")]
    pub r#scm_minimum_tls_version: Box<String>,
    #[builder(into)]
    #[serde(rename = "scmType")]
    pub r#scm_type: Box<String>,
    /// Is the Linux Function App `ip_restriction` configuration used for the SCM also?
    #[builder(into)]
    #[serde(rename = "scmUseMainIpRestriction")]
    pub r#scm_use_main_ip_restriction: Box<bool>,
    /// Does the Linux Web App use a 32-bit worker process?
    #[builder(into)]
    #[serde(rename = "use32BitWorker")]
    pub r#use_32_bit_worker: Box<bool>,
    /// Are all outbound traffic to NAT Gateways, Network Security Groups and User Defined Routes applied?
    #[builder(into)]
    #[serde(rename = "vnetRouteAllEnabled")]
    pub r#vnet_route_all_enabled: Box<bool>,
    /// Are Web Sockets enabled?
    #[builder(into)]
    #[serde(rename = "websocketsEnabled")]
    pub r#websockets_enabled: Box<bool>,
    /// The number of Workers for this Linux Function App.
    #[builder(into)]
    #[serde(rename = "workerCount")]
    pub r#worker_count: Box<i32>,
}
