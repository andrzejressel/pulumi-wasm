#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct StandardSiteConfig {
    /// Should the Logic App be loaded at all times? Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "alwaysOn")]
    pub r#always_on: Box<Option<bool>>,
    /// The number of workers this Logic App can scale out to. Only applicable to apps on the Consumption and Premium plan.
    #[builder(into, default)]
    #[serde(rename = "appScaleLimit")]
    pub r#app_scale_limit: Box<Option<i32>>,
    /// The Auto-swap slot name.
    #[builder(into, default)]
    #[serde(rename = "autoSwapSlotName")]
    pub r#auto_swap_slot_name: Box<Option<String>>,
    /// A `cors` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "cors")]
    pub r#cors: Box<Option<super::super::types::logicapps::StandardSiteConfigCors>>,
    /// The version of the .NET framework's CLR used in this Logic App Possible values are `v4.0` (including .NET Core 2.1 and 3.1), `v5.0`, `v6.0` and `v8.0`. [For more information on which .NET Framework version to use based on the runtime version you're targeting - please see this table](https://docs.microsoft.com/azure/azure-functions/functions-dotnet-class-library#supported-versions). Defaults to `v4.0`.
    #[builder(into, default)]
    #[serde(rename = "dotnetFrameworkVersion")]
    pub r#dotnet_framework_version: Box<Option<String>>,
    /// The number of minimum instances for this Logic App Only affects apps on the Premium plan.
    #[builder(into, default)]
    #[serde(rename = "elasticInstanceMinimum")]
    pub r#elastic_instance_minimum: Box<Option<i32>>,
    /// State of FTP / FTPS service for this Logic App. Possible values include: `AllAllowed`, `FtpsOnly` and `Disabled`. Defaults to `AllAllowed`.
    #[builder(into, default)]
    #[serde(rename = "ftpsState")]
    pub r#ftps_state: Box<Option<String>>,
    /// Path which will be checked for this Logic App health.
    #[builder(into, default)]
    #[serde(rename = "healthCheckPath")]
    pub r#health_check_path: Box<Option<String>>,
    /// Specifies whether the HTTP2 protocol should be enabled. Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "http2Enabled")]
    pub r#http_2_enabled: Box<Option<bool>>,
    /// A list of `ip_restriction` objects representing IP restrictions as defined below.
    /// 
    /// > **NOTE** User has to explicitly set `ip_restriction` to empty slice (`[]`) to remove it.
    #[builder(into, default)]
    #[serde(rename = "ipRestrictions")]
    pub r#ip_restrictions: Box<Option<Vec<super::super::types::logicapps::StandardSiteConfigIpRestriction>>>,
    /// Linux App Framework and version for the App Service, e.g. `DOCKER|(golang:latest)`. Setting this value will also set the `kind` of application deployed to `functionapp,linux,container,workflowapp`
    #[builder(into, default)]
    #[serde(rename = "linuxFxVersion")]
    pub r#linux_fx_version: Box<Option<String>>,
    /// The minimum supported TLS version for the Logic App. Possible values are `1.0`, `1.1`, and `1.2`. Defaults to `1.2` for new Logic Apps.
    /// 
    /// > **Note** Azure Services will require TLS 1.2+ by August 2025, please see this [announcement](https://azure.microsoft.com/en-us/updates/v2/update-retirement-tls1-0-tls1-1-versions-azure-services/) for more.
    #[builder(into, default)]
    #[serde(rename = "minTlsVersion")]
    pub r#min_tls_version: Box<Option<String>>,
    /// The number of pre-warmed instances for this Logic App Only affects apps on the Premium plan.
    #[builder(into, default)]
    #[serde(rename = "preWarmedInstanceCount")]
    pub r#pre_warmed_instance_count: Box<Option<i32>>,
    #[builder(into, default)]
    #[serde(rename = "publicNetworkAccessEnabled")]
    pub r#public_network_access_enabled: Box<Option<bool>>,
    /// Should Runtime Scale Monitoring be enabled?. Only applicable to apps on the Premium plan. Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "runtimeScaleMonitoringEnabled")]
    pub r#runtime_scale_monitoring_enabled: Box<Option<bool>>,
    /// A list of `scm_ip_restriction` objects representing SCM IP restrictions as defined below.
    /// 
    /// > **NOTE** User has to explicitly set `scm_ip_restriction` to empty slice (`[]`) to remove it.
    #[builder(into, default)]
    #[serde(rename = "scmIpRestrictions")]
    pub r#scm_ip_restrictions: Box<Option<Vec<super::super::types::logicapps::StandardSiteConfigScmIpRestriction>>>,
    /// Configures the minimum version of TLS required for SSL requests to the SCM site. Possible values are `1.0`, `1.1` and `1.2`.
    /// 
    /// > **Note** Azure Services will require TLS 1.2+ by August 2025, please see this [announcement](https://azure.microsoft.com/en-us/updates/v2/update-retirement-tls1-0-tls1-1-versions-azure-services/) for more.
    #[builder(into, default)]
    #[serde(rename = "scmMinTlsVersion")]
    pub r#scm_min_tls_version: Box<Option<String>>,
    /// The type of Source Control used by the Logic App in use by the Windows Function App. Defaults to `None`. Possible values are: `BitbucketGit`, `BitbucketHg`, `CodePlexGit`, `CodePlexHg`, `Dropbox`, `ExternalGit`, `ExternalHg`, `GitHub`, `LocalGit`, `None`, `OneDrive`, `Tfs`, `VSO`, and `VSTSRM`
    #[builder(into, default)]
    #[serde(rename = "scmType")]
    pub r#scm_type: Box<Option<String>>,
    /// Should the Logic App `ip_restriction` configuration be used for the SCM too. Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "scmUseMainIpRestriction")]
    pub r#scm_use_main_ip_restriction: Box<Option<bool>>,
    /// Should the Logic App run in 32 bit mode, rather than 64 bit mode? Defaults to `true`.
    /// 
    /// > **Note:** when using an App Service Plan in the `Free` or `Shared` Tiers `use_32_bit_worker_process` must be set to `true`.
    #[builder(into, default)]
    #[serde(rename = "use32BitWorkerProcess")]
    pub r#use_32_bit_worker_process: Box<Option<bool>>,
    /// Should all outbound traffic to have Virtual Network Security Groups and User Defined Routes applied.
    #[builder(into, default)]
    #[serde(rename = "vnetRouteAllEnabled")]
    pub r#vnet_route_all_enabled: Box<Option<bool>>,
    /// Should WebSockets be enabled?
    #[builder(into, default)]
    #[serde(rename = "websocketsEnabled")]
    pub r#websockets_enabled: Box<Option<bool>>,
}
