#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AppServiceSiteConfig {
    /// Are Managed Identity Credentials used for Azure Container Registry pull
    #[builder(into, default)]
    #[serde(rename = "acrUseManagedIdentityCredentials")]
    pub r#acr_use_managed_identity_credentials: Box<Option<bool>>,
    /// If using User Managed Identity, the User Managed Identity Client Id
    /// 
    /// > **NOTE:** When using User Managed Identity with Azure Container Registry the Identity will need to have the [ACRPull role assigned](https://docs.microsoft.com/azure/container-registry/container-registry-authentication-managed-identity#example-1-access-with-a-user-assigned-identity)
    #[builder(into, default)]
    #[serde(rename = "acrUserManagedIdentityClientId")]
    pub r#acr_user_managed_identity_client_id: Box<Option<String>>,
    /// Should the app be loaded at all times? Defaults to `false`.
    /// 
    /// > **NOTE:** when using an App Service Plan in the `Free` or `Shared` Tiers `always_on` must be set to `false`.
    #[builder(into, default)]
    #[serde(rename = "alwaysOn")]
    pub r#always_on: Box<Option<bool>>,
    /// App command line to launch, e.g. `/sbin/myserver -b 0.0.0.0`.
    #[builder(into, default)]
    #[serde(rename = "appCommandLine")]
    pub r#app_command_line: Box<Option<String>>,
    /// The name of the slot to automatically swap to during deployment
    #[builder(into, default)]
    #[serde(rename = "autoSwapSlotName")]
    pub r#auto_swap_slot_name: Box<Option<String>>,
    /// A `cors` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "cors")]
    pub r#cors: Box<Option<super::super::types::appservice::AppServiceSiteConfigCors>>,
    /// The ordering of default documents to load, if an address isn't specified.
    #[builder(into, default)]
    #[serde(rename = "defaultDocuments")]
    pub r#default_documents: Box<Option<Vec<String>>>,
    /// The version of the .NET framework's CLR used in this App Service. Possible values are `v2.0` (which will use the latest version of the .NET framework for the .NET CLR v2 - currently `.net 3.5`), `v4.0` (which corresponds to the latest version of the .NET CLR v4 - which at the time of writing is `.net 4.7.1`), `v5.0` and `v6.0`. [For more information on which .NET CLR version to use based on the .NET framework you're targeting - please see this table](https://en.wikipedia.org/wiki/.NET_Framework_version_history#Overview). Defaults to `v4.0`.
    #[builder(into, default)]
    #[serde(rename = "dotnetFrameworkVersion")]
    pub r#dotnet_framework_version: Box<Option<String>>,
    /// State of FTP / FTPS service for this App Service. Possible values include: `AllAllowed`, `FtpsOnly` and `Disabled`.
    #[builder(into, default)]
    #[serde(rename = "ftpsState")]
    pub r#ftps_state: Box<Option<String>>,
    /// The health check path to be pinged by App Service. [For more information - please see App Service health check announcement](https://azure.github.io/AppService/2020/08/24/healthcheck-on-app-service.html).
    #[builder(into, default)]
    #[serde(rename = "healthCheckPath")]
    pub r#health_check_path: Box<Option<String>>,
    /// Is HTTP2 Enabled on this App Service? Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "http2Enabled")]
    pub r#http_2_enabled: Box<Option<bool>>,
    /// A list of objects representing ip restrictions as defined below.
    /// 
    /// > **NOTE** User has to explicitly set `ip_restriction` to empty slice (`[]`) to remove it.
    #[builder(into, default)]
    #[serde(rename = "ipRestrictions")]
    pub r#ip_restrictions: Box<Option<Vec<super::super::types::appservice::AppServiceSiteConfigIpRestriction>>>,
    /// The Java Container to use. If specified `java_version` and `java_container_version` must also be specified. Possible values are `JAVA`, `JETTY`, and `TOMCAT`.
    #[builder(into, default)]
    #[serde(rename = "javaContainer")]
    pub r#java_container: Box<Option<String>>,
    /// The version of the Java Container to use. If specified `java_version` and `java_container` must also be specified.
    #[builder(into, default)]
    #[serde(rename = "javaContainerVersion")]
    pub r#java_container_version: Box<Option<String>>,
    /// The version of Java to use. If specified `java_container` and `java_container_version` must also be specified. Possible values are `1.7`, `1.8` and `11` and their specific versions - except for Java 11 (e.g. `1.7.0_80`, `1.8.0_181`, `11`)
    #[builder(into, default)]
    #[serde(rename = "javaVersion")]
    pub r#java_version: Box<Option<String>>,
    /// Linux App Framework and version for the App Service. Possible options are a Docker container (`DOCKER|<user/image:tag>`), a base-64 encoded Docker Compose file (`COMPOSE|${filebase64("compose.yml")}`) or a base-64 encoded Kubernetes Manifest (`KUBE|${filebase64("kubernetes.yml")}`).
    /// 
    /// > **NOTE:** To set this property the App Service Plan to which the App belongs must be configured with `kind = "Linux"`, and `reserved = true` or the API will reject any value supplied.
    #[builder(into, default)]
    #[serde(rename = "linuxFxVersion")]
    pub r#linux_fx_version: Box<Option<String>>,
    /// Is "MySQL In App" Enabled? This runs a local MySQL instance with your app and shares resources from the App Service plan.
    /// 
    /// > **NOTE:** MySQL In App is not intended for production environments and will not scale beyond a single instance. Instead you may wish to use Azure Database for MySQL.
    #[builder(into, default)]
    #[serde(rename = "localMysqlEnabled")]
    pub r#local_mysql_enabled: Box<Option<bool>>,
    /// The Managed Pipeline Mode. Possible values are `Integrated` and `Classic`. Defaults to `Integrated`.
    #[builder(into, default)]
    #[serde(rename = "managedPipelineMode")]
    pub r#managed_pipeline_mode: Box<Option<String>>,
    /// The minimum supported TLS version for the app service. Possible values are `1.0`, `1.1`, and `1.2`. Defaults to `1.2` for new app services.
    #[builder(into, default)]
    #[serde(rename = "minTlsVersion")]
    pub r#min_tls_version: Box<Option<String>>,
    /// The scaled number of workers (for per site scaling) of this App Service. Requires that `per_site_scaling` is enabled on the `azure.appservice.Plan`. [For more information - please see Microsoft documentation on high-density hosting](https://docs.microsoft.com/azure/app-service/manage-scale-per-app).
    #[builder(into, default)]
    #[serde(rename = "numberOfWorkers")]
    pub r#number_of_workers: Box<Option<i32>>,
    /// The version of PHP to use in this App Service. Possible values are `5.5`, `5.6`, `7.0`, `7.1`, `7.2`, `7.3` and `7.4`.
    #[builder(into, default)]
    #[serde(rename = "phpVersion")]
    pub r#php_version: Box<Option<String>>,
    /// The version of Python to use in this App Service. Possible values are `2.7` and `3.4`.
    #[builder(into, default)]
    #[serde(rename = "pythonVersion")]
    pub r#python_version: Box<Option<String>>,
    /// Is Remote Debugging Enabled? Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "remoteDebuggingEnabled")]
    pub r#remote_debugging_enabled: Box<Option<bool>>,
    /// Which version of Visual Studio should the Remote Debugger be compatible with? Possible values are `VS2017`, `VS2019`, `VS2022`.
    #[builder(into, default)]
    #[serde(rename = "remoteDebuggingVersion")]
    pub r#remote_debugging_version: Box<Option<String>>,
    /// A list of `scm_ip_restriction` objects representing IP restrictions as defined below.
    /// 
    /// > **NOTE** User has to explicitly set `scm_ip_restriction` to empty slice (`[]`) to remove it.
    #[builder(into, default)]
    #[serde(rename = "scmIpRestrictions")]
    pub r#scm_ip_restrictions: Box<Option<Vec<super::super::types::appservice::AppServiceSiteConfigScmIpRestriction>>>,
    /// The type of Source Control enabled for this App Service. Defaults to `None`. Possible values are: `BitbucketGit`, `BitbucketHg`, `CodePlexGit`, `CodePlexHg`, `Dropbox`, `ExternalGit`, `ExternalHg`, `GitHub`, `LocalGit`, `None`, `OneDrive`, `Tfs`, `VSO`, and `VSTSRM`
    #[builder(into, default)]
    #[serde(rename = "scmType")]
    pub r#scm_type: Box<Option<String>>,
    /// IP security restrictions for scm to use main. Defaults to `false`. 
    /// 
    /// > **NOTE** Any `scm_ip_restriction` blocks configured are ignored by the service when `scm_use_main_ip_restriction` is set to `true`. Any scm restrictions will become active if this is subsequently set to `false` or removed.
    #[builder(into, default)]
    #[serde(rename = "scmUseMainIpRestriction")]
    pub r#scm_use_main_ip_restriction: Box<Option<bool>>,
    /// Should the App Service run in 32 bit mode, rather than 64 bit mode?
    /// 
    /// > **NOTE:** when using an App Service Plan in the `Free` or `Shared` Tiers `use_32_bit_worker_process` must be set to `true`.
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
    /// The Windows Docker container image (`DOCKER|<user/image:tag>`)
    #[builder(into, default)]
    #[serde(rename = "windowsFxVersion")]
    pub r#windows_fx_version: Box<Option<String>>,
}
