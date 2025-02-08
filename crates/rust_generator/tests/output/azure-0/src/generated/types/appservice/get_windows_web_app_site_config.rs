#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetWindowsWebAppSiteConfig {
    /// Is this Windows Web App is Always On enabled.
    #[builder(into)]
    #[serde(rename = "alwaysOn")]
    pub r#always_on: Box<bool>,
    /// The ID of the APIM configuration for this Windows Web App.
    #[builder(into)]
    #[serde(rename = "apiDefinitionUrl")]
    pub r#api_definition_url: Box<String>,
    /// The ID of the API Management setting linked to the Windows Web App.
    #[builder(into)]
    #[serde(rename = "apiManagementApiId")]
    pub r#api_management_api_id: Box<String>,
    /// The command line used to launch this app.
    #[builder(into)]
    #[serde(rename = "appCommandLine")]
    pub r#app_command_line: Box<String>,
    /// A `application_stack` block as defined above.
    #[builder(into)]
    #[serde(rename = "applicationStacks")]
    pub r#application_stacks: Box<Vec<super::super::types::appservice::GetWindowsWebAppSiteConfigApplicationStack>>,
    /// A `auto_heal_setting` block as defined above.
    #[builder(into)]
    #[serde(rename = "autoHealSettings")]
    pub r#auto_heal_settings: Box<Vec<super::super::types::appservice::GetWindowsWebAppSiteConfigAutoHealSetting>>,
    /// The Client ID of the Managed Service Identity used for connections to the Azure Container Registry.
    #[builder(into)]
    #[serde(rename = "containerRegistryManagedIdentityClientId")]
    pub r#container_registry_managed_identity_client_id: Box<String>,
    /// Do connections for Azure Container Registry use Managed Identity.
    #[builder(into)]
    #[serde(rename = "containerRegistryUseManagedIdentity")]
    pub r#container_registry_use_managed_identity: Box<bool>,
    /// A `cors` block as defined above.
    #[builder(into)]
    #[serde(rename = "cors")]
    pub r#cors: Box<Vec<super::super::types::appservice::GetWindowsWebAppSiteConfigCor>>,
    /// The list of Default Documents for the Windows Web App.
    #[builder(into)]
    #[serde(rename = "defaultDocuments")]
    pub r#default_documents: Box<Vec<String>>,
    /// Is Detailed Error Logging enabled.
    #[builder(into)]
    #[serde(rename = "detailedErrorLoggingEnabled")]
    pub r#detailed_error_logging_enabled: Box<bool>,
    /// The State of FTP / FTPS service.
    #[builder(into)]
    #[serde(rename = "ftpsState")]
    pub r#ftps_state: Box<String>,
    /// A `handler_mapping` block as defined below.
    #[builder(into)]
    #[serde(rename = "handlerMappings")]
    pub r#handler_mappings: Box<Vec<super::super::types::appservice::GetWindowsWebAppSiteConfigHandlerMapping>>,
    /// (Optional) The amount of time in minutes that a node can be unhealthy before being removed from the load balancer. Possible values are between `2` and `10`. Only valid in conjunction with `health_check_path`.
    #[builder(into)]
    #[serde(rename = "healthCheckEvictionTimeInMin")]
    pub r#health_check_eviction_time_in_min: Box<i32>,
    /// The path to the Health Check endpoint.
    #[builder(into)]
    #[serde(rename = "healthCheckPath")]
    pub r#health_check_path: Box<String>,
    /// Is HTTP2.0 enabled.
    #[builder(into)]
    #[serde(rename = "http2Enabled")]
    pub r#http_2_enabled: Box<bool>,
    /// The Default action for traffic that does not match any `ip_restriction` rule.
    #[builder(into)]
    #[serde(rename = "ipRestrictionDefaultAction")]
    pub r#ip_restriction_default_action: Box<String>,
    /// A `ip_restriction` block as defined above.
    #[builder(into)]
    #[serde(rename = "ipRestrictions")]
    pub r#ip_restrictions: Box<Vec<super::super::types::appservice::GetWindowsWebAppSiteConfigIpRestriction>>,
    /// The site Load Balancing Mode.
    #[builder(into)]
    #[serde(rename = "loadBalancingMode")]
    pub r#load_balancing_mode: Box<String>,
    /// Is the Local MySQL enabled.
    #[builder(into)]
    #[serde(rename = "localMysqlEnabled")]
    pub r#local_mysql_enabled: Box<bool>,
    /// The Managed Pipeline Mode.
    #[builder(into)]
    #[serde(rename = "managedPipelineMode")]
    pub r#managed_pipeline_mode: Box<String>,
    /// The Minimum version of TLS for requests.
    #[builder(into)]
    #[serde(rename = "minimumTlsVersion")]
    pub r#minimum_tls_version: Box<String>,
    #[builder(into)]
    #[serde(rename = "remoteDebuggingEnabled")]
    pub r#remote_debugging_enabled: Box<bool>,
    /// The Remote Debugging Version.
    #[builder(into)]
    #[serde(rename = "remoteDebuggingVersion")]
    pub r#remote_debugging_version: Box<String>,
    /// The Default action for traffic that does not match any `scm_ip_restriction` rule.
    #[builder(into)]
    #[serde(rename = "scmIpRestrictionDefaultAction")]
    pub r#scm_ip_restriction_default_action: Box<String>,
    /// A `scm_ip_restriction` block as defined above.
    #[builder(into)]
    #[serde(rename = "scmIpRestrictions")]
    pub r#scm_ip_restrictions: Box<Vec<super::super::types::appservice::GetWindowsWebAppSiteConfigScmIpRestriction>>,
    /// The Minimum version of TLS for requests to SCM.
    #[builder(into)]
    #[serde(rename = "scmMinimumTlsVersion")]
    pub r#scm_minimum_tls_version: Box<String>,
    /// The Source Control Management Type in use.
    #[builder(into)]
    #[serde(rename = "scmType")]
    pub r#scm_type: Box<String>,
    /// Is the Windows Web App `ip_restriction` configuration used for the SCM also.
    #[builder(into)]
    #[serde(rename = "scmUseMainIpRestriction")]
    pub r#scm_use_main_ip_restriction: Box<bool>,
    /// Does the Windows Web App use a 32-bit worker.
    #[builder(into)]
    #[serde(rename = "use32BitWorker")]
    pub r#use_32_bit_worker: Box<bool>,
    /// A `virtual_application` block as defined below.
    #[builder(into)]
    #[serde(rename = "virtualApplications")]
    pub r#virtual_applications: Box<Vec<super::super::types::appservice::GetWindowsWebAppSiteConfigVirtualApplication>>,
    /// Are all outbound traffic to NAT Gateways, Network Security Groups and User Defined Routes applied?
    #[builder(into)]
    #[serde(rename = "vnetRouteAllEnabled")]
    pub r#vnet_route_all_enabled: Box<bool>,
    /// Are Web Sockets enabled?
    #[builder(into)]
    #[serde(rename = "websocketsEnabled")]
    pub r#websockets_enabled: Box<bool>,
    /// The string representation of the Windows FX Version.
    #[builder(into)]
    #[serde(rename = "windowsFxVersion")]
    pub r#windows_fx_version: Box<String>,
    /// The number of Workers for this Windows App Service.
    #[builder(into)]
    #[serde(rename = "workerCount")]
    pub r#worker_count: Box<i32>,
}
