#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetStandardSiteConfig {
    /// Should the Logic App be loaded at all times?
    #[builder(into, default)]
    #[serde(rename = "alwaysOn")]
    pub r#always_on: Box<Option<bool>>,
    /// The number of workers this Logic App can scale out to. Only applicable to apps on the Consumption and Premium plan.
    #[builder(into)]
    #[serde(rename = "appScaleLimit")]
    pub r#app_scale_limit: Box<i32>,
    /// The Auto-swap slot name.
    #[builder(into)]
    #[serde(rename = "autoSwapSlotName")]
    pub r#auto_swap_slot_name: Box<String>,
    /// A `cors` block as defined below.
    #[builder(into)]
    #[serde(rename = "cors")]
    pub r#cors: Box<super::super::types::logicapps::GetStandardSiteConfigCors>,
    /// The version of the .NET framework's CLR used in this Logic App.
    #[builder(into, default)]
    #[serde(rename = "dotnetFrameworkVersion")]
    pub r#dotnet_framework_version: Box<Option<String>>,
    /// The number of minimum instances for this Logic App Only affects apps on the Premium plan.
    #[builder(into)]
    #[serde(rename = "elasticInstanceMinimum")]
    pub r#elastic_instance_minimum: Box<i32>,
    /// The state of FTP / FTPS service for this Logic App.
    #[builder(into)]
    #[serde(rename = "ftpsState")]
    pub r#ftps_state: Box<String>,
    /// Path which will be checked for this Logic App health.
    #[builder(into, default)]
    #[serde(rename = "healthCheckPath")]
    pub r#health_check_path: Box<Option<String>>,
    /// Specifies whether the HTTP2 protocol should be enabled.
    #[builder(into, default)]
    #[serde(rename = "http2Enabled")]
    pub r#http_2_enabled: Box<Option<bool>>,
    /// A list of `ip_restriction` objects representing IP restrictions as defined below.
    #[builder(into)]
    #[serde(rename = "ipRestrictions")]
    pub r#ip_restrictions: Box<Vec<super::super::types::logicapps::GetStandardSiteConfigIpRestriction>>,
    /// Linux App Framework and version for the Logic App.
    #[builder(into)]
    #[serde(rename = "linuxFxVersion")]
    pub r#linux_fx_version: Box<String>,
    /// The minimum supported TLS version for the Logic App.
    #[builder(into)]
    #[serde(rename = "minTlsVersion")]
    pub r#min_tls_version: Box<String>,
    /// The number of pre-warmed instances for this Logic App Only affects apps on the Premium plan.
    #[builder(into)]
    #[serde(rename = "preWarmedInstanceCount")]
    pub r#pre_warmed_instance_count: Box<i32>,
    #[builder(into)]
    #[serde(rename = "publicNetworkAccessEnabled")]
    pub r#public_network_access_enabled: Box<bool>,
    /// Should Runtime Scale Monitoring be enabled?. Only applicable to apps on the Premium plan.
    #[builder(into, default)]
    #[serde(rename = "runtimeScaleMonitoringEnabled")]
    pub r#runtime_scale_monitoring_enabled: Box<Option<bool>>,
    /// A list of `scm_ip_restriction` objects representing SCM IP restrictions as defined below.
    #[builder(into)]
    #[serde(rename = "scmIpRestrictions")]
    pub r#scm_ip_restrictions: Box<Vec<super::super::types::logicapps::GetStandardSiteConfigScmIpRestriction>>,
    /// The minimum version of TLS required for SSL requests to the SCM site.
    #[builder(into)]
    #[serde(rename = "scmMinTlsVersion")]
    pub r#scm_min_tls_version: Box<String>,
    /// The type of Source Control used by the Logic App in use by the Windows Function App.
    #[builder(into)]
    #[serde(rename = "scmType")]
    pub r#scm_type: Box<String>,
    /// Should the Logic App `ip_restriction` configuration be used for the SCM too.
    #[builder(into, default)]
    #[serde(rename = "scmUseMainIpRestriction")]
    pub r#scm_use_main_ip_restriction: Box<Option<bool>>,
    /// Should the Logic App run in 32 bit mode, rather than 64 bit mode?
    #[builder(into, default)]
    #[serde(rename = "use32BitWorkerProcess")]
    pub r#use_32_bit_worker_process: Box<Option<bool>>,
    /// Should all outbound traffic to have Virtual Network Security Groups and User Defined Routes applied.
    #[builder(into)]
    #[serde(rename = "vnetRouteAllEnabled")]
    pub r#vnet_route_all_enabled: Box<bool>,
    /// Should WebSockets be enabled?
    #[builder(into, default)]
    #[serde(rename = "websocketsEnabled")]
    pub r#websockets_enabled: Box<Option<bool>>,
}
