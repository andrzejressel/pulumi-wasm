/// Manages a Function App deployment Slot.
///
/// !> **NOTE:** This resource has been deprecated in version 5.0 of the provider and will be removed in version 6.0. Please use `azure.appservice.LinuxFunctionAppSlot` and `azure.appservice.WindowsFunctionAppSlot` resources instead.
///
/// ## Example Usage
///
/// ### With App Service Plan)
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("azure-functions-test-rg")
///             .build_struct(),
///     );
///     let exampleAccount = account::create(
///         "exampleAccount",
///         AccountArgs::builder()
///             .account_replication_type("LRS")
///             .account_tier("Standard")
///             .location("${example.location}")
///             .name("functionsapptestsa")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleFunctionApp = function_app::create(
///         "exampleFunctionApp",
///         FunctionAppArgs::builder()
///             .app_service_plan_id("${examplePlan.id}")
///             .location("${example.location}")
///             .name("test-azure-functions")
///             .resource_group_name("${example.name}")
///             .storage_account_access_key("${exampleAccount.primaryAccessKey}")
///             .storage_account_name("${exampleAccount.name}")
///             .build_struct(),
///     );
///     let exampleFunctionAppSlot = function_app_slot::create(
///         "exampleFunctionAppSlot",
///         FunctionAppSlotArgs::builder()
///             .app_service_plan_id("${examplePlan.id}")
///             .function_app_name("${exampleFunctionApp.name}")
///             .location("${example.location}")
///             .name("test-azure-functions_slot")
///             .resource_group_name("${example.name}")
///             .storage_account_access_key("${exampleAccount.primaryAccessKey}")
///             .storage_account_name("${exampleAccount.name}")
///             .build_struct(),
///     );
///     let examplePlan = plan::create(
///         "examplePlan",
///         PlanArgs::builder()
///             .location("${example.location}")
///             .name("azure-functions-test-service-plan")
///             .resource_group_name("${example.name}")
///             .sku(PlanSku::builder().size("S1").tier("Standard").build_struct())
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Function Apps Deployment Slots can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:appservice/functionAppSlot:FunctionAppSlot functionapp1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Web/sites/functionapp1/slots/staging
/// ```
///
pub mod function_app_slot {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FunctionAppSlotArgs {
        /// The ID of the App Service Plan within which to create this Function App Slot. Changing this forces a new resource to be created.
        #[builder(into)]
        pub app_service_plan_id: pulumi_wasm_rust::Output<String>,
        /// A key-value pair of App Settings.
        ///
        /// > **Note:** When integrating a `CI/CD pipeline` and expecting to run from a deployed package in `Azure` you must seed your `app settings` as part of the application code for function app to be successfully deployed. `Important Default key pairs`: (`"WEBSITE_RUN_FROM_PACKAGE" = ""`, `"FUNCTIONS_WORKER_RUNTIME" = "node"` (or python, etc), `"WEBSITE_NODE_DEFAULT_VERSION" = "10.14.1"`, `"APPINSIGHTS_INSTRUMENTATIONKEY" = ""`).
        ///
        /// > **NOTE:** The values for `AzureWebJobsStorage` and `FUNCTIONS_EXTENSION_VERSION` will be filled by other input arguments and shouldn't be configured separately. `AzureWebJobsStorage` is filled based on `storage_account_name` and `storage_account_access_key`. `FUNCTIONS_EXTENSION_VERSION` is filled based on `version`.
        ///
        /// > **Note:**  When using an App Service Plan in the `Free` or `Shared` Tiers `use_32_bit_worker_process` must be set to `true`.
        #[builder(into, default)]
        pub app_settings: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// An `auth_settings` block as defined below.
        #[builder(into, default)]
        pub auth_settings: pulumi_wasm_rust::Output<
            Option<super::super::types::appservice::FunctionAppSlotAuthSettings>,
        >,
        /// A `connection_string` block as defined below.
        #[builder(into, default)]
        pub connection_strings: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::appservice::FunctionAppSlotConnectionString>>,
        >,
        /// The amount of memory in gigabyte-seconds that your application is allowed to consume per day. Setting this value only affects function apps under the consumption plan.
        #[builder(into, default)]
        pub daily_memory_time_quota: pulumi_wasm_rust::Output<Option<i32>>,
        /// Should the built-in logging of the Function App be enabled? Defaults to `true`.
        #[builder(into, default)]
        pub enable_builtin_logging: pulumi_wasm_rust::Output<Option<bool>>,
        /// Is the Function App enabled? Defaults to `true`.
        #[builder(into, default)]
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name of the Function App within which to create the Function App Slot. Changing this forces a new resource to be created.
        #[builder(into)]
        pub function_app_name: pulumi_wasm_rust::Output<String>,
        /// Can the Function App only be accessed via HTTPS? Defaults to `false`.
        #[builder(into, default)]
        pub https_only: pulumi_wasm_rust::Output<Option<bool>>,
        /// An `identity` block as defined below.
        #[builder(into, default)]
        pub identity: pulumi_wasm_rust::Output<
            Option<super::super::types::appservice::FunctionAppSlotIdentity>,
        >,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name of the Function App. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// A string indicating the Operating System type for this function app. The only possible value is `linux`. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** This value will be `linux` for Linux Derivatives or an empty string for Windows (default).
        #[builder(into, default)]
        pub os_type: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the resource group in which to create the Function App Slot. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A `site_config` object as defined below.
        #[builder(into, default)]
        pub site_config: pulumi_wasm_rust::Output<
            Option<super::super::types::appservice::FunctionAppSlotSiteConfig>,
        >,
        /// The access key which will be used to access the backend storage account for the Function App.
        #[builder(into)]
        pub storage_account_access_key: pulumi_wasm_rust::Output<String>,
        /// The backend storage account name which will be used by the Function App (such as the dashboard, logs). Changing this forces a new resource to be created.
        #[builder(into)]
        pub storage_account_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The runtime version associated with the Function App. Defaults to `~1`.
        #[builder(into, default)]
        pub version: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct FunctionAppSlotResult {
        /// The ID of the App Service Plan within which to create this Function App Slot. Changing this forces a new resource to be created.
        pub app_service_plan_id: pulumi_wasm_rust::Output<String>,
        /// A key-value pair of App Settings.
        ///
        /// > **Note:** When integrating a `CI/CD pipeline` and expecting to run from a deployed package in `Azure` you must seed your `app settings` as part of the application code for function app to be successfully deployed. `Important Default key pairs`: (`"WEBSITE_RUN_FROM_PACKAGE" = ""`, `"FUNCTIONS_WORKER_RUNTIME" = "node"` (or python, etc), `"WEBSITE_NODE_DEFAULT_VERSION" = "10.14.1"`, `"APPINSIGHTS_INSTRUMENTATIONKEY" = ""`).
        ///
        /// > **NOTE:** The values for `AzureWebJobsStorage` and `FUNCTIONS_EXTENSION_VERSION` will be filled by other input arguments and shouldn't be configured separately. `AzureWebJobsStorage` is filled based on `storage_account_name` and `storage_account_access_key`. `FUNCTIONS_EXTENSION_VERSION` is filled based on `version`.
        ///
        /// > **Note:**  When using an App Service Plan in the `Free` or `Shared` Tiers `use_32_bit_worker_process` must be set to `true`.
        pub app_settings: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// An `auth_settings` block as defined below.
        pub auth_settings: pulumi_wasm_rust::Output<
            super::super::types::appservice::FunctionAppSlotAuthSettings,
        >,
        /// A `connection_string` block as defined below.
        pub connection_strings: pulumi_wasm_rust::Output<
            Vec<super::super::types::appservice::FunctionAppSlotConnectionString>,
        >,
        /// The amount of memory in gigabyte-seconds that your application is allowed to consume per day. Setting this value only affects function apps under the consumption plan.
        pub daily_memory_time_quota: pulumi_wasm_rust::Output<Option<i32>>,
        /// The default hostname associated with the Function App - such as `mysite.azurewebsites.net`
        pub default_hostname: pulumi_wasm_rust::Output<String>,
        /// Should the built-in logging of the Function App be enabled? Defaults to `true`.
        pub enable_builtin_logging: pulumi_wasm_rust::Output<Option<bool>>,
        /// Is the Function App enabled? Defaults to `true`.
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name of the Function App within which to create the Function App Slot. Changing this forces a new resource to be created.
        pub function_app_name: pulumi_wasm_rust::Output<String>,
        /// Can the Function App only be accessed via HTTPS? Defaults to `false`.
        pub https_only: pulumi_wasm_rust::Output<Option<bool>>,
        /// An `identity` block as defined below.
        pub identity: pulumi_wasm_rust::Output<
            Option<super::super::types::appservice::FunctionAppSlotIdentity>,
        >,
        /// The Function App kind - such as `functionapp,linux,container`
        pub kind: pulumi_wasm_rust::Output<String>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the Function App. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A string indicating the Operating System type for this function app. The only possible value is `linux`. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** This value will be `linux` for Linux Derivatives or an empty string for Windows (default).
        pub os_type: pulumi_wasm_rust::Output<Option<String>>,
        /// A comma separated list of outbound IP addresses - such as `52.23.25.3,52.143.43.12`
        pub outbound_ip_addresses: pulumi_wasm_rust::Output<String>,
        /// A comma separated list of outbound IP addresses - such as `52.23.25.3,52.143.43.12,52.143.43.17` - not all of which are necessarily in use. Superset of `outbound_ip_addresses`.
        pub possible_outbound_ip_addresses: pulumi_wasm_rust::Output<String>,
        /// The name of the resource group in which to create the Function App Slot. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A `site_config` object as defined below.
        pub site_config: pulumi_wasm_rust::Output<
            super::super::types::appservice::FunctionAppSlotSiteConfig,
        >,
        /// A `site_credential` block as defined below, which contains the site-level credentials used to publish to this Function App Slot.
        pub site_credentials: pulumi_wasm_rust::Output<
            Vec<super::super::types::appservice::FunctionAppSlotSiteCredential>,
        >,
        /// The access key which will be used to access the backend storage account for the Function App.
        pub storage_account_access_key: pulumi_wasm_rust::Output<String>,
        /// The backend storage account name which will be used by the Function App (such as the dashboard, logs). Changing this forces a new resource to be created.
        pub storage_account_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The runtime version associated with the Function App. Defaults to `~1`.
        pub version: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: FunctionAppSlotArgs) -> FunctionAppSlotResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let app_service_plan_id_binding = args.app_service_plan_id.get_inner();
        let app_settings_binding = args.app_settings.get_inner();
        let auth_settings_binding = args.auth_settings.get_inner();
        let connection_strings_binding = args.connection_strings.get_inner();
        let daily_memory_time_quota_binding = args.daily_memory_time_quota.get_inner();
        let enable_builtin_logging_binding = args.enable_builtin_logging.get_inner();
        let enabled_binding = args.enabled.get_inner();
        let function_app_name_binding = args.function_app_name.get_inner();
        let https_only_binding = args.https_only.get_inner();
        let identity_binding = args.identity.get_inner();
        let location_binding = args.location.get_inner();
        let name_binding = args.name.get_inner();
        let os_type_binding = args.os_type.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let site_config_binding = args.site_config.get_inner();
        let storage_account_access_key_binding = args
            .storage_account_access_key
            .get_inner();
        let storage_account_name_binding = args.storage_account_name.get_inner();
        let tags_binding = args.tags.get_inner();
        let version_binding = args.version.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:appservice/functionAppSlot:FunctionAppSlot".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "appServicePlanId".into(),
                    value: &app_service_plan_id_binding,
                },
                register_interface::ObjectField {
                    name: "appSettings".into(),
                    value: &app_settings_binding,
                },
                register_interface::ObjectField {
                    name: "authSettings".into(),
                    value: &auth_settings_binding,
                },
                register_interface::ObjectField {
                    name: "connectionStrings".into(),
                    value: &connection_strings_binding,
                },
                register_interface::ObjectField {
                    name: "dailyMemoryTimeQuota".into(),
                    value: &daily_memory_time_quota_binding,
                },
                register_interface::ObjectField {
                    name: "enableBuiltinLogging".into(),
                    value: &enable_builtin_logging_binding,
                },
                register_interface::ObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding,
                },
                register_interface::ObjectField {
                    name: "functionAppName".into(),
                    value: &function_app_name_binding,
                },
                register_interface::ObjectField {
                    name: "httpsOnly".into(),
                    value: &https_only_binding,
                },
                register_interface::ObjectField {
                    name: "identity".into(),
                    value: &identity_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "osType".into(),
                    value: &os_type_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "siteConfig".into(),
                    value: &site_config_binding,
                },
                register_interface::ObjectField {
                    name: "storageAccountAccessKey".into(),
                    value: &storage_account_access_key_binding,
                },
                register_interface::ObjectField {
                    name: "storageAccountName".into(),
                    value: &storage_account_name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "version".into(),
                    value: &version_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "appServicePlanId".into(),
                },
                register_interface::ResultField {
                    name: "appSettings".into(),
                },
                register_interface::ResultField {
                    name: "authSettings".into(),
                },
                register_interface::ResultField {
                    name: "connectionStrings".into(),
                },
                register_interface::ResultField {
                    name: "dailyMemoryTimeQuota".into(),
                },
                register_interface::ResultField {
                    name: "defaultHostname".into(),
                },
                register_interface::ResultField {
                    name: "enableBuiltinLogging".into(),
                },
                register_interface::ResultField {
                    name: "enabled".into(),
                },
                register_interface::ResultField {
                    name: "functionAppName".into(),
                },
                register_interface::ResultField {
                    name: "httpsOnly".into(),
                },
                register_interface::ResultField {
                    name: "identity".into(),
                },
                register_interface::ResultField {
                    name: "kind".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "osType".into(),
                },
                register_interface::ResultField {
                    name: "outboundIpAddresses".into(),
                },
                register_interface::ResultField {
                    name: "possibleOutboundIpAddresses".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "siteConfig".into(),
                },
                register_interface::ResultField {
                    name: "siteCredentials".into(),
                },
                register_interface::ResultField {
                    name: "storageAccountAccessKey".into(),
                },
                register_interface::ResultField {
                    name: "storageAccountName".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "version".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        FunctionAppSlotResult {
            app_service_plan_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("appServicePlanId").unwrap(),
            ),
            app_settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("appSettings").unwrap(),
            ),
            auth_settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("authSettings").unwrap(),
            ),
            connection_strings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("connectionStrings").unwrap(),
            ),
            daily_memory_time_quota: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dailyMemoryTimeQuota").unwrap(),
            ),
            default_hostname: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultHostname").unwrap(),
            ),
            enable_builtin_logging: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enableBuiltinLogging").unwrap(),
            ),
            enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enabled").unwrap(),
            ),
            function_app_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("functionAppName").unwrap(),
            ),
            https_only: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("httpsOnly").unwrap(),
            ),
            identity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("identity").unwrap(),
            ),
            kind: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kind").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            os_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("osType").unwrap(),
            ),
            outbound_ip_addresses: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("outboundIpAddresses").unwrap(),
            ),
            possible_outbound_ip_addresses: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("possibleOutboundIpAddresses").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            site_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("siteConfig").unwrap(),
            ),
            site_credentials: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("siteCredentials").unwrap(),
            ),
            storage_account_access_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageAccountAccessKey").unwrap(),
            ),
            storage_account_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageAccountName").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("version").unwrap(),
            ),
        }
    }
}