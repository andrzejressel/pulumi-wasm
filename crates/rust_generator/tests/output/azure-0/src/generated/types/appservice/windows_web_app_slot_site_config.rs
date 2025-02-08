#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct WindowsWebAppSlotSiteConfig {
    /// If this Windows Web App Slot is Always On enabled. Defaults to `true`.
    #[builder(into, default)]
    #[serde(rename = "alwaysOn")]
    pub r#always_on: Box<Option<bool>>,
    /// The URL to the API Definition for this Windows Web App Slot.
    #[builder(into, default)]
    #[serde(rename = "apiDefinitionUrl")]
    pub r#api_definition_url: Box<Option<String>>,
    /// The API Management API ID this Windows Web App Slot os associated with.
    #[builder(into, default)]
    #[serde(rename = "apiManagementApiId")]
    pub r#api_management_api_id: Box<Option<String>>,
    /// The App command line to launch.
    #[builder(into, default)]
    #[serde(rename = "appCommandLine")]
    pub r#app_command_line: Box<Option<String>>,
    /// A `application_stack` block as defined above.
    #[builder(into, default)]
    #[serde(rename = "applicationStack")]
    pub r#application_stack: Box<Option<super::super::types::appservice::WindowsWebAppSlotSiteConfigApplicationStack>>,
    /// A `auto_heal_setting` block as defined above. Required with `auto_heal`.
    #[builder(into, default)]
    #[serde(rename = "autoHealSetting")]
    pub r#auto_heal_setting: Box<Option<super::super::types::appservice::WindowsWebAppSlotSiteConfigAutoHealSetting>>,
    /// The Windows Web App Slot Name to automatically swap to when deployment to that slot is successfully completed.
    /// 
    /// > **Note:** This must be a valid slot name on the target Windows Web App Slot.
    #[builder(into, default)]
    #[serde(rename = "autoSwapSlotName")]
    pub r#auto_swap_slot_name: Box<Option<String>>,
    /// The Client ID of the Managed Service Identity to use for connections to the Azure Container Registry.
    #[builder(into, default)]
    #[serde(rename = "containerRegistryManagedIdentityClientId")]
    pub r#container_registry_managed_identity_client_id: Box<Option<String>>,
    /// Should connections for Azure Container Registry use Managed Identity.
    #[builder(into, default)]
    #[serde(rename = "containerRegistryUseManagedIdentity")]
    pub r#container_registry_use_managed_identity: Box<Option<bool>>,
    /// A `cors` block as defined above.
    #[builder(into, default)]
    #[serde(rename = "cors")]
    pub r#cors: Box<Option<super::super::types::appservice::WindowsWebAppSlotSiteConfigCors>>,
    /// Specifies a list of Default Documents for the Windows Web App Slot.
    #[builder(into, default)]
    #[serde(rename = "defaultDocuments")]
    pub r#default_documents: Box<Option<Vec<String>>>,
    #[builder(into, default)]
    #[serde(rename = "detailedErrorLoggingEnabled")]
    pub r#detailed_error_logging_enabled: Box<Option<bool>>,
    #[builder(into, default)]
    #[serde(rename = "ftpsState")]
    pub r#ftps_state: Box<Option<String>>,
    /// One or more `handler_mapping` blocks as defined below.
    #[builder(into, default)]
    #[serde(rename = "handlerMappings")]
    pub r#handler_mappings: Box<Option<Vec<super::super::types::appservice::WindowsWebAppSlotSiteConfigHandlerMapping>>>,
    /// The amount of time in minutes that a node can be unhealthy before being removed from the load balancer. Possible values are between `2` and `10`. Only valid in conjunction with `health_check_path`.
    #[builder(into, default)]
    #[serde(rename = "healthCheckEvictionTimeInMin")]
    pub r#health_check_eviction_time_in_min: Box<Option<i32>>,
    /// The path to the Health Check.
    #[builder(into, default)]
    #[serde(rename = "healthCheckPath")]
    pub r#health_check_path: Box<Option<String>>,
    /// Should the HTTP2 be enabled?
    #[builder(into, default)]
    #[serde(rename = "http2Enabled")]
    pub r#http_2_enabled: Box<Option<bool>>,
    /// The Default action for traffic that does not match any `ip_restriction` rule. possible values include `Allow` and `Deny`. Defaults to `Allow`.
    #[builder(into, default)]
    #[serde(rename = "ipRestrictionDefaultAction")]
    pub r#ip_restriction_default_action: Box<Option<String>>,
    /// One or more `ip_restriction` blocks as defined above.
    #[builder(into, default)]
    #[serde(rename = "ipRestrictions")]
    pub r#ip_restrictions: Box<Option<Vec<super::super::types::appservice::WindowsWebAppSlotSiteConfigIpRestriction>>>,
    /// The Site load balancing. Possible values include: `WeightedRoundRobin`, `LeastRequests`, `LeastResponseTime`, `WeightedTotalTraffic`, `RequestHash`, `PerSiteRoundRobin`. Defaults to `LeastRequests` if omitted.
    #[builder(into, default)]
    #[serde(rename = "loadBalancingMode")]
    pub r#load_balancing_mode: Box<Option<String>>,
    /// Use Local MySQL. Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "localMysqlEnabled")]
    pub r#local_mysql_enabled: Box<Option<bool>>,
    /// Managed pipeline mode. Possible values include: `Integrated`, `Classic`. Defaults to `Integrated`.
    #[builder(into, default)]
    #[serde(rename = "managedPipelineMode")]
    pub r#managed_pipeline_mode: Box<Option<String>>,
    /// The configures the minimum version of TLS required for SSL requests. Possible values include: `1.0`, `1.1`, and `1.2`. Defaults to `1.2`.
    #[builder(into, default)]
    #[serde(rename = "minimumTlsVersion")]
    pub r#minimum_tls_version: Box<Option<String>>,
    /// Should Remote Debugging be enabled. Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "remoteDebuggingEnabled")]
    pub r#remote_debugging_enabled: Box<Option<bool>>,
    /// The Remote Debugging Version. Possible values include `VS2017`, `VS2019` and `VS2022`
    #[builder(into, default)]
    #[serde(rename = "remoteDebuggingVersion")]
    pub r#remote_debugging_version: Box<Option<String>>,
    /// The Default action for traffic that does not match any `scm_ip_restriction` rule. possible values include `Allow` and `Deny`. Defaults to `Allow`.
    #[builder(into, default)]
    #[serde(rename = "scmIpRestrictionDefaultAction")]
    pub r#scm_ip_restriction_default_action: Box<Option<String>>,
    /// One or more `scm_ip_restriction` blocks as defined above.
    #[builder(into, default)]
    #[serde(rename = "scmIpRestrictions")]
    pub r#scm_ip_restrictions: Box<Option<Vec<super::super::types::appservice::WindowsWebAppSlotSiteConfigScmIpRestriction>>>,
    /// The configures the minimum version of TLS required for SSL requests to the SCM site Possible values include: `1.0`, `1.1`, and `1.2`. Defaults to `1.2`.
    #[builder(into, default)]
    #[serde(rename = "scmMinimumTlsVersion")]
    pub r#scm_minimum_tls_version: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "scmType")]
    pub r#scm_type: Box<Option<String>>,
    /// Should the Windows Web App Slot `ip_restriction` configuration be used for the SCM also.
    #[builder(into, default)]
    #[serde(rename = "scmUseMainIpRestriction")]
    pub r#scm_use_main_ip_restriction: Box<Option<bool>>,
    /// Should the Windows Web App Slot use a 32-bit worker. The default value varies from different service plans.
    #[builder(into, default)]
    #[serde(rename = "use32BitWorker")]
    pub r#use_32_bit_worker: Box<Option<bool>>,
    /// One or more `virtual_application` blocks as defined below.
    #[builder(into, default)]
    #[serde(rename = "virtualApplications")]
    pub r#virtual_applications: Box<Option<Vec<super::super::types::appservice::WindowsWebAppSlotSiteConfigVirtualApplication>>>,
    /// Should all outbound traffic to have NAT Gateways, Network Security Groups and User Defined Routes applied? Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "vnetRouteAllEnabled")]
    pub r#vnet_route_all_enabled: Box<Option<bool>>,
    /// Should Web Sockets be enabled. Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "websocketsEnabled")]
    pub r#websockets_enabled: Box<Option<bool>>,
    #[builder(into, default)]
    #[serde(rename = "windowsFxVersion")]
    pub r#windows_fx_version: Box<Option<String>>,
    /// The number of Workers for this Windows App Service Slot.
    #[builder(into, default)]
    #[serde(rename = "workerCount")]
    pub r#worker_count: Box<Option<i32>>,
}
