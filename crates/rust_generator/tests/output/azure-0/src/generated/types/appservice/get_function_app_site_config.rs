#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetFunctionAppSiteConfig {
    /// Is the app loaded at all times?
    #[builder(into)]
    #[serde(rename = "alwaysOn")]
    pub r#always_on: Box<bool>,
    /// The number of workers this function app can scale out to. Only applicable to apps on the Consumption and Premium plan.
    #[builder(into)]
    #[serde(rename = "appScaleLimit")]
    pub r#app_scale_limit: Box<i32>,
    #[builder(into)]
    #[serde(rename = "autoSwapSlotName")]
    pub r#auto_swap_slot_name: Box<String>,
    /// A `cors` block as defined above.
    #[builder(into)]
    #[serde(rename = "cors")]
    pub r#cors: Box<super::super::types::appservice::GetFunctionAppSiteConfigCors>,
    /// The version of the .NET framework's CLR used in this App Service.
    #[builder(into)]
    #[serde(rename = "dotnetFrameworkVersion")]
    pub r#dotnet_framework_version: Box<String>,
    /// The number of minimum instances for this function app. Only applicable to apps on the Premium plan.
    #[builder(into)]
    #[serde(rename = "elasticInstanceMinimum")]
    pub r#elastic_instance_minimum: Box<i32>,
    /// State of FTP / FTPS service for this AppService.
    #[builder(into)]
    #[serde(rename = "ftpsState")]
    pub r#ftps_state: Box<String>,
    #[builder(into)]
    #[serde(rename = "healthCheckPath")]
    pub r#health_check_path: Box<String>,
    /// Is HTTP2 Enabled on this App Service?
    #[builder(into)]
    #[serde(rename = "http2Enabled")]
    pub r#http_2_enabled: Box<bool>,
    /// One or more `ip_restriction` blocks as defined above.
    #[builder(into)]
    #[serde(rename = "ipRestrictions")]
    pub r#ip_restrictions: Box<Vec<super::super::types::appservice::GetFunctionAppSiteConfigIpRestriction>>,
    /// Java version hosted by the function app in Azure.
    #[builder(into)]
    #[serde(rename = "javaVersion")]
    pub r#java_version: Box<String>,
    /// Linux App Framework and version for the AppService.
    #[builder(into)]
    #[serde(rename = "linuxFxVersion")]
    pub r#linux_fx_version: Box<String>,
    /// The minimum supported TLS version for this App Service.
    #[builder(into)]
    #[serde(rename = "minTlsVersion")]
    pub r#min_tls_version: Box<String>,
    /// The number of pre-warmed instances for this function app. Only applicable to apps on the Premium plan.
    #[builder(into)]
    #[serde(rename = "preWarmedInstanceCount")]
    pub r#pre_warmed_instance_count: Box<i32>,
    /// Is Runtime Scale Monitoring Enabled on this function app?
    #[builder(into)]
    #[serde(rename = "runtimeScaleMonitoringEnabled")]
    pub r#runtime_scale_monitoring_enabled: Box<bool>,
    /// One or more `scm_ip_restriction` blocks as defined above.
    #[builder(into)]
    #[serde(rename = "scmIpRestrictions")]
    pub r#scm_ip_restrictions: Box<Vec<super::super::types::appservice::GetFunctionAppSiteConfigScmIpRestriction>>,
    /// The type of Source Control enabled for this App Service.
    #[builder(into)]
    #[serde(rename = "scmType")]
    pub r#scm_type: Box<String>,
    /// IP security restrictions for scm to use main.
    #[builder(into)]
    #[serde(rename = "scmUseMainIpRestriction")]
    pub r#scm_use_main_ip_restriction: Box<bool>,
    /// Does the App Service run in 32 bit mode, rather than 64 bit mode?
    #[builder(into)]
    #[serde(rename = "use32BitWorkerProcess")]
    pub r#use_32_bit_worker_process: Box<bool>,
    /// (Optional) Should all outbound traffic to have Virtual Network Security Groups and User Defined Routes applied?
    #[builder(into)]
    #[serde(rename = "vnetRouteAllEnabled")]
    pub r#vnet_route_all_enabled: Box<bool>,
    /// Are WebSockets enabled for this App Service?
    #[builder(into)]
    #[serde(rename = "websocketsEnabled")]
    pub r#websockets_enabled: Box<bool>,
}
