#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetAppServiceSiteConfig {
    /// Are Managed Identity Credentials used for Azure Container Registry pull.
    #[builder(into)]
    #[serde(rename = "acrUseManagedIdentityCredentials")]
    pub r#acr_use_managed_identity_credentials: Box<bool>,
    /// The User Managed Identity Client Id.
    #[builder(into)]
    #[serde(rename = "acrUserManagedIdentityClientId")]
    pub r#acr_user_managed_identity_client_id: Box<String>,
    /// Is the app loaded at all times?
    #[builder(into)]
    #[serde(rename = "alwaysOn")]
    pub r#always_on: Box<bool>,
    /// App command line to launch.
    #[builder(into)]
    #[serde(rename = "appCommandLine")]
    pub r#app_command_line: Box<String>,
    /// A `cors` block as defined above.
    #[builder(into)]
    #[serde(rename = "cors")]
    pub r#cors: Box<Vec<super::super::types::appservice::GetAppServiceSiteConfigCor>>,
    /// The ordering of default documents to load, if an address isn't specified.
    #[builder(into)]
    #[serde(rename = "defaultDocuments")]
    pub r#default_documents: Box<Vec<String>>,
    /// The version of the .NET framework's CLR used in this App Service.
    #[builder(into)]
    #[serde(rename = "dotnetFrameworkVersion")]
    pub r#dotnet_framework_version: Box<String>,
    /// State of FTP / FTPS service for this AppService.
    #[builder(into)]
    #[serde(rename = "ftpsState")]
    pub r#ftps_state: Box<String>,
    /// The health check path to be pinged by App Service.
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
    pub r#ip_restrictions: Box<Vec<super::super::types::appservice::GetAppServiceSiteConfigIpRestriction>>,
    /// The Java Container in use.
    #[builder(into)]
    #[serde(rename = "javaContainer")]
    pub r#java_container: Box<String>,
    /// The version of the Java Container in use.
    #[builder(into)]
    #[serde(rename = "javaContainerVersion")]
    pub r#java_container_version: Box<String>,
    /// The version of Java in use.
    #[builder(into)]
    #[serde(rename = "javaVersion")]
    pub r#java_version: Box<String>,
    /// Linux App Framework and version for the AppService.
    #[builder(into)]
    #[serde(rename = "linuxFxVersion")]
    pub r#linux_fx_version: Box<String>,
    /// Is "MySQL In App" Enabled? This runs a local MySQL instance with your app and shares resources from the App Service plan.
    #[builder(into)]
    #[serde(rename = "localMysqlEnabled")]
    pub r#local_mysql_enabled: Box<bool>,
    /// The Managed Pipeline Mode used in this App Service.
    #[builder(into)]
    #[serde(rename = "managedPipelineMode")]
    pub r#managed_pipeline_mode: Box<String>,
    /// The minimum supported TLS version for this App Service.
    #[builder(into)]
    #[serde(rename = "minTlsVersion")]
    pub r#min_tls_version: Box<String>,
    /// The scaled number of workers (for per site scaling) of this App Service.
    #[builder(into)]
    #[serde(rename = "numberOfWorkers")]
    pub r#number_of_workers: Box<i32>,
    /// The version of PHP used in this App Service.
    #[builder(into)]
    #[serde(rename = "phpVersion")]
    pub r#php_version: Box<String>,
    /// The version of Python used in this App Service.
    #[builder(into)]
    #[serde(rename = "pythonVersion")]
    pub r#python_version: Box<String>,
    /// Is Remote Debugging Enabled in this App Service?
    #[builder(into)]
    #[serde(rename = "remoteDebuggingEnabled")]
    pub r#remote_debugging_enabled: Box<bool>,
    /// Which version of Visual Studio is the Remote Debugger compatible with?
    #[builder(into)]
    #[serde(rename = "remoteDebuggingVersion")]
    pub r#remote_debugging_version: Box<String>,
    /// One or more `scm_ip_restriction` blocks as defined above.
    #[builder(into)]
    #[serde(rename = "scmIpRestrictions")]
    pub r#scm_ip_restrictions: Box<Vec<super::super::types::appservice::GetAppServiceSiteConfigScmIpRestriction>>,
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
    /// Windows Container Docker Image for the AppService.
    #[builder(into)]
    #[serde(rename = "windowsFxVersion")]
    pub r#windows_fx_version: Box<String>,
}
