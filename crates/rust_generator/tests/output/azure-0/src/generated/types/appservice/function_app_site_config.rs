#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FunctionAppSiteConfig {
    /// Should the Function App be loaded at all times? Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "alwaysOn")]
    pub r#always_on: Box<Option<bool>>,
    /// The number of workers this function app can scale out to. Only applicable to apps on the Consumption and Premium plan.
    #[builder(into, default)]
    #[serde(rename = "appScaleLimit")]
    pub r#app_scale_limit: Box<Option<i32>>,
    /// The name of the slot to automatically swap to during deployment
    /// 
    /// > **NOTE:** This attribute is only used for slots.
    #[builder(into, default)]
    #[serde(rename = "autoSwapSlotName")]
    pub r#auto_swap_slot_name: Box<Option<String>>,
    /// A `cors` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "cors")]
    pub r#cors: Box<Option<super::super::types::appservice::FunctionAppSiteConfigCors>>,
    /// The version of the .NET framework's CLR used in this function app. Possible values are `v4.0` (including .NET Core 2.1 and 3.1), `v5.0` and `v6.0`. [For more information on which .NET Framework version to use based on the runtime version you're targeting - please see this table](https://docs.microsoft.com/azure/azure-functions/functions-dotnet-class-library#supported-versions). Defaults to `v4.0`.
    #[builder(into, default)]
    #[serde(rename = "dotnetFrameworkVersion")]
    pub r#dotnet_framework_version: Box<Option<String>>,
    /// The number of minimum instances for this function app. Only affects apps on the Premium plan. Possible values are between `1` and `20`.
    #[builder(into, default)]
    #[serde(rename = "elasticInstanceMinimum")]
    pub r#elastic_instance_minimum: Box<Option<i32>>,
    /// State of FTP / FTPS service for this function app. Possible values include: `AllAllowed`, `FtpsOnly` and `Disabled`. Defaults to `AllAllowed`.
    #[builder(into, default)]
    #[serde(rename = "ftpsState")]
    pub r#ftps_state: Box<Option<String>>,
    /// Path which will be checked for this function app health.
    #[builder(into, default)]
    #[serde(rename = "healthCheckPath")]
    pub r#health_check_path: Box<Option<String>>,
    /// Specifies whether or not the HTTP2 protocol should be enabled. Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "http2Enabled")]
    pub r#http_2_enabled: Box<Option<bool>>,
    /// A list of `ip_restriction` objects representing IP restrictions as defined below.
    /// 
    /// > **NOTE** User has to explicitly set `ip_restriction` to empty slice (`[]`) to remove it.
    #[builder(into, default)]
    #[serde(rename = "ipRestrictions")]
    pub r#ip_restrictions: Box<Option<Vec<super::super::types::appservice::FunctionAppSiteConfigIpRestriction>>>,
    /// Java version hosted by the function app in Azure. Possible values are `1.8`, `11` & `17` (In-Preview).
    #[builder(into, default)]
    #[serde(rename = "javaVersion")]
    pub r#java_version: Box<Option<String>>,
    /// Linux App Framework and version for the AppService, e.g. `DOCKER|(golang:latest)`.
    #[builder(into, default)]
    #[serde(rename = "linuxFxVersion")]
    pub r#linux_fx_version: Box<Option<String>>,
    /// The minimum supported TLS version for the function app. Possible values are `1.0`, `1.1`, and `1.2`. Defaults to `1.2` for new function apps.
    #[builder(into, default)]
    #[serde(rename = "minTlsVersion")]
    pub r#min_tls_version: Box<Option<String>>,
    /// The number of pre-warmed instances for this function app. Only affects apps on the Premium plan.
    #[builder(into, default)]
    #[serde(rename = "preWarmedInstanceCount")]
    pub r#pre_warmed_instance_count: Box<Option<i32>>,
    /// Should Runtime Scale Monitoring be enabled?. Only applicable to apps on the Premium plan. Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "runtimeScaleMonitoringEnabled")]
    pub r#runtime_scale_monitoring_enabled: Box<Option<bool>>,
    /// A list of `scm_ip_restriction` objects representing IP restrictions as defined below.
    /// 
    /// > **NOTE** User has to explicitly set `scm_ip_restriction` to empty slice (`[]`) to remove it.
    #[builder(into, default)]
    #[serde(rename = "scmIpRestrictions")]
    pub r#scm_ip_restrictions: Box<Option<Vec<super::super::types::appservice::FunctionAppSiteConfigScmIpRestriction>>>,
    /// The type of Source Control used by the Function App. Valid values include: `BitBucketGit`, `BitBucketHg`, `CodePlexGit`, `CodePlexHg`, `Dropbox`, `ExternalGit`, `ExternalHg`, `GitHub`, `LocalGit`, `None` (default), `OneDrive`, `Tfs`, `VSO`, and `VSTSRM`.
    /// 
    /// > **NOTE:** This setting is incompatible with the `source_control` block which updates this value based on the setting provided.
    #[builder(into, default)]
    #[serde(rename = "scmType")]
    pub r#scm_type: Box<Option<String>>,
    /// IP security restrictions for scm to use main. Defaults to `false`.
    /// 
    /// > **NOTE** Any `scm_ip_restriction` blocks configured are ignored by the service when `scm_use_main_ip_restriction` is set to `true`. Any scm restrictions will become active if this is subsequently set to `false` or removed.
    #[builder(into, default)]
    #[serde(rename = "scmUseMainIpRestriction")]
    pub r#scm_use_main_ip_restriction: Box<Option<bool>>,
    /// Should the Function App run in 32 bit mode, rather than 64 bit mode? Defaults to `true`.
    /// 
    /// > **Note:** when using an App Service Plan in the `Free` or `Shared` Tiers `use_32_bit_worker_process` must be set to `true`.
    #[builder(into, default)]
    #[serde(rename = "use32BitWorkerProcess")]
    pub r#use_32_bit_worker_process: Box<Option<bool>>,
    #[builder(into, default)]
    #[serde(rename = "vnetRouteAllEnabled")]
    pub r#vnet_route_all_enabled: Box<Option<bool>>,
    /// Should WebSockets be enabled?
    #[builder(into, default)]
    #[serde(rename = "websocketsEnabled")]
    pub r#websockets_enabled: Box<Option<bool>>,
}
