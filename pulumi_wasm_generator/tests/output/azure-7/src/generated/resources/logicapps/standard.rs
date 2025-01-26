/// Manages a Logic App (Standard / Single Tenant)
///
/// ## Example Usage
///
/// ### With App Service Plan)
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: azure-functions-test-rg
///       location: West Europe
///   exampleAccount:
///     type: azure:storage:Account
///     name: example
///     properties:
///       name: functionsapptestsa
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       accountTier: Standard
///       accountReplicationType: LRS
///   examplePlan:
///     type: azure:appservice:Plan
///     name: example
///     properties:
///       name: azure-functions-test-service-plan
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       kind: elastic
///       sku:
///         tier: WorkflowStandard
///         size: WS1
///   exampleStandard:
///     type: azure:logicapps:Standard
///     name: example
///     properties:
///       name: test-azure-functions
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       appServicePlanId: ${examplePlan.id}
///       storageAccountName: ${exampleAccount.name}
///       storageAccountAccessKey: ${exampleAccount.primaryAccessKey}
///       appSettings:
///         FUNCTIONS_WORKER_RUNTIME: node
///         WEBSITE_NODE_DEFAULT_VERSION: ~18
/// ```
///
///
/// ### For Container Mode)
///
/// > **Note:** You must set `azure.appservice.Plan` `kind` to `Linux` and `reserved` to `true` when used with `linux_fx_version`
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: azure-functions-test-rg
///       location: West Europe
///   exampleAccount:
///     type: azure:storage:Account
///     name: example
///     properties:
///       name: functionsapptestsa
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       accountTier: Standard
///       accountReplicationType: LRS
///   examplePlan:
///     type: azure:appservice:Plan
///     name: example
///     properties:
///       name: azure-functions-test-service-plan
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       kind: Linux
///       reserved: true
///       sku:
///         tier: WorkflowStandard
///         size: WS1
///   exampleStandard:
///     type: azure:logicapps:Standard
///     name: example
///     properties:
///       name: test-azure-functions
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       appServicePlanId: ${examplePlan.id}
///       storageAccountName: ${exampleAccount.name}
///       storageAccountAccessKey: ${exampleAccount.primaryAccessKey}
///       siteConfig:
///         linuxFxVersion: DOCKER|mcr.microsoft.com/azure-functions/dotnet:3.0-appservice
///       appSettings:
///         DOCKER_REGISTRY_SERVER_URL: https://<server-name>.azurecr.io
///         DOCKER_REGISTRY_SERVER_USERNAME: username
///         DOCKER_REGISTRY_SERVER_PASSWORD: password
/// ```
///
/// ## Import
///
/// Logic Apps can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:logicapps/standard:Standard logicapp1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Web/sites/logicapp1
/// ```
///
pub mod standard {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct StandardArgs {
        /// The ID of the App Service Plan within which to create this Logic App.
        #[builder(into)]
        pub app_service_plan_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// A map of key-value pairs for [App Settings](https://docs.microsoft.com/azure/azure-functions/functions-app-settings) and custom values.
        ///
        /// > **NOTE:** There are a number of application settings that will be managed for you by this resource type and *shouldn't* be configured separately as part of the app_settings you specify.  `AzureWebJobsStorage` is filled based on `storage_account_name` and `storage_account_access_key`. `WEBSITE_CONTENTSHARE` is detailed below. `FUNCTIONS_EXTENSION_VERSION` is filled based on `version`. `APP_KIND` is set to workflowApp and `AzureFunctionsJobHost__extensionBundle__id` and `AzureFunctionsJobHost__extensionBundle__version` are set as detailed below.
        #[builder(into, default)]
        pub app_settings: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// If `use_extension_bundle` then controls the allowed range for bundle versions. Defaults to `[1.*, 2.0.0)`.
        #[builder(into, default)]
        pub bundle_version: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Should the Logic App send session affinity cookies, which route client requests in the same session to the same instance?
        #[builder(into, default)]
        pub client_affinity_enabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// The mode of the Logic App's client certificates requirement for incoming requests. Possible values are `Required` and `Optional`.
        #[builder(into, default)]
        pub client_certificate_mode: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A `connection_string` block as defined below.
        #[builder(into, default)]
        pub connection_strings: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::logicapps::StandardConnectionString>>,
        >,
        /// Is the Logic App enabled? Defaults to `true`.
        #[builder(into, default)]
        pub enabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Can the Logic App only be accessed via HTTPS? Defaults to `false`.
        #[builder(into, default)]
        pub https_only: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// An `identity` block as defined below.
        #[builder(into, default)]
        pub identity: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::logicapps::StandardIdentity>,
        >,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of the Logic App Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Whether Public Network Access should be enabled or not. Possible values are `Enabled` and `Disabled`. Defaults to `Enabled`.
        ///
        /// > **Note:** Setting this property will also set it in the Site Config.
        #[builder(into, default)]
        pub public_network_access: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the resource group in which to create the Logic App. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// A `site_config` object as defined below.
        #[builder(into, default)]
        pub site_config: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::logicapps::StandardSiteConfig>,
        >,
        /// The access key which will be used to access the backend storage account for the Logic App.
        #[builder(into)]
        pub storage_account_access_key: pulumi_wasm_rust::InputOrOutput<String>,
        /// The backend storage account name which will be used by this Logic App (e.g. for Stateful workflows data). Changing this forces a new resource to be created.
        #[builder(into)]
        pub storage_account_name: pulumi_wasm_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub storage_account_share_name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Should the logic app use the bundled extension package? If true, then application settings for `AzureFunctionsJobHost__extensionBundle__id` and `AzureFunctionsJobHost__extensionBundle__version` will be created. Defaults to `true`.
        #[builder(into, default)]
        pub use_extension_bundle: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// The runtime version associated with the Logic App. Defaults to `~4`.
        ///
        /// > **Note:**  Logic App version `3.x` will be out of support from December 3 2022. For more details refer [Logic Apps Standard Support for Functions Runtime V4](https://azure.microsoft.com/en-us/updates/logic-apps-standard-support-for-functions-runtime-v4/)
        #[builder(into, default)]
        pub version: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub virtual_network_subnet_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct StandardResult {
        /// The ID of the App Service Plan within which to create this Logic App.
        pub app_service_plan_id: pulumi_wasm_rust::Output<String>,
        /// A map of key-value pairs for [App Settings](https://docs.microsoft.com/azure/azure-functions/functions-app-settings) and custom values.
        ///
        /// > **NOTE:** There are a number of application settings that will be managed for you by this resource type and *shouldn't* be configured separately as part of the app_settings you specify.  `AzureWebJobsStorage` is filled based on `storage_account_name` and `storage_account_access_key`. `WEBSITE_CONTENTSHARE` is detailed below. `FUNCTIONS_EXTENSION_VERSION` is filled based on `version`. `APP_KIND` is set to workflowApp and `AzureFunctionsJobHost__extensionBundle__id` and `AzureFunctionsJobHost__extensionBundle__version` are set as detailed below.
        pub app_settings: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// If `use_extension_bundle` then controls the allowed range for bundle versions. Defaults to `[1.*, 2.0.0)`.
        pub bundle_version: pulumi_wasm_rust::Output<Option<String>>,
        /// Should the Logic App send session affinity cookies, which route client requests in the same session to the same instance?
        pub client_affinity_enabled: pulumi_wasm_rust::Output<bool>,
        /// The mode of the Logic App's client certificates requirement for incoming requests. Possible values are `Required` and `Optional`.
        pub client_certificate_mode: pulumi_wasm_rust::Output<Option<String>>,
        /// A `connection_string` block as defined below.
        pub connection_strings: pulumi_wasm_rust::Output<
            Vec<super::super::types::logicapps::StandardConnectionString>,
        >,
        /// An identifier used by App Service to perform domain ownership verification via DNS TXT record.
        pub custom_domain_verification_id: pulumi_wasm_rust::Output<String>,
        /// The default hostname associated with the Logic App - such as `mysite.azurewebsites.net`
        pub default_hostname: pulumi_wasm_rust::Output<String>,
        /// Is the Logic App enabled? Defaults to `true`.
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Can the Logic App only be accessed via HTTPS? Defaults to `false`.
        pub https_only: pulumi_wasm_rust::Output<Option<bool>>,
        /// An `identity` block as defined below.
        pub identity: pulumi_wasm_rust::Output<
            Option<super::super::types::logicapps::StandardIdentity>,
        >,
        /// The Logic App kind - will be `functionapp,workflowapp`
        pub kind: pulumi_wasm_rust::Output<String>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the Logic App Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A comma separated list of outbound IP addresses - such as `52.23.25.3,52.143.43.12`
        pub outbound_ip_addresses: pulumi_wasm_rust::Output<String>,
        /// A comma separated list of outbound IP addresses - such as `52.23.25.3,52.143.43.12,52.143.43.17` - not all of which are necessarily in use. Superset of `outbound_ip_addresses`.
        pub possible_outbound_ip_addresses: pulumi_wasm_rust::Output<String>,
        /// Whether Public Network Access should be enabled or not. Possible values are `Enabled` and `Disabled`. Defaults to `Enabled`.
        ///
        /// > **Note:** Setting this property will also set it in the Site Config.
        pub public_network_access: pulumi_wasm_rust::Output<String>,
        /// The name of the resource group in which to create the Logic App. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A `site_config` object as defined below.
        pub site_config: pulumi_wasm_rust::Output<
            super::super::types::logicapps::StandardSiteConfig,
        >,
        /// A `site_credential` block as defined below, which contains the site-level credentials used to publish to this App Service.
        pub site_credentials: pulumi_wasm_rust::Output<
            Vec<super::super::types::logicapps::StandardSiteCredential>,
        >,
        /// The access key which will be used to access the backend storage account for the Logic App.
        pub storage_account_access_key: pulumi_wasm_rust::Output<String>,
        /// The backend storage account name which will be used by this Logic App (e.g. for Stateful workflows data). Changing this forces a new resource to be created.
        pub storage_account_name: pulumi_wasm_rust::Output<String>,
        pub storage_account_share_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Should the logic app use the bundled extension package? If true, then application settings for `AzureFunctionsJobHost__extensionBundle__id` and `AzureFunctionsJobHost__extensionBundle__version` will be created. Defaults to `true`.
        pub use_extension_bundle: pulumi_wasm_rust::Output<Option<bool>>,
        /// The runtime version associated with the Logic App. Defaults to `~4`.
        ///
        /// > **Note:**  Logic App version `3.x` will be out of support from December 3 2022. For more details refer [Logic Apps Standard Support for Functions Runtime V4](https://azure.microsoft.com/en-us/updates/logic-apps-standard-support-for-functions-runtime-v4/)
        pub version: pulumi_wasm_rust::Output<Option<String>>,
        pub virtual_network_subnet_id: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: StandardArgs,
    ) -> StandardResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let app_service_plan_id_binding = args
            .app_service_plan_id
            .get_output(context)
            .get_inner();
        let app_settings_binding = args.app_settings.get_output(context).get_inner();
        let bundle_version_binding = args.bundle_version.get_output(context).get_inner();
        let client_affinity_enabled_binding = args
            .client_affinity_enabled
            .get_output(context)
            .get_inner();
        let client_certificate_mode_binding = args
            .client_certificate_mode
            .get_output(context)
            .get_inner();
        let connection_strings_binding = args
            .connection_strings
            .get_output(context)
            .get_inner();
        let enabled_binding = args.enabled.get_output(context).get_inner();
        let https_only_binding = args.https_only.get_output(context).get_inner();
        let identity_binding = args.identity.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let public_network_access_binding = args
            .public_network_access
            .get_output(context)
            .get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let site_config_binding = args.site_config.get_output(context).get_inner();
        let storage_account_access_key_binding = args
            .storage_account_access_key
            .get_output(context)
            .get_inner();
        let storage_account_name_binding = args
            .storage_account_name
            .get_output(context)
            .get_inner();
        let storage_account_share_name_binding = args
            .storage_account_share_name
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let use_extension_bundle_binding = args
            .use_extension_bundle
            .get_output(context)
            .get_inner();
        let version_binding = args.version.get_output(context).get_inner();
        let virtual_network_subnet_id_binding = args
            .virtual_network_subnet_id
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:logicapps/standard:Standard".into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
                    name: "bundleVersion".into(),
                    value: &bundle_version_binding,
                },
                register_interface::ObjectField {
                    name: "clientAffinityEnabled".into(),
                    value: &client_affinity_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "clientCertificateMode".into(),
                    value: &client_certificate_mode_binding,
                },
                register_interface::ObjectField {
                    name: "connectionStrings".into(),
                    value: &connection_strings_binding,
                },
                register_interface::ObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding,
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
                    name: "publicNetworkAccess".into(),
                    value: &public_network_access_binding,
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
                    name: "storageAccountShareName".into(),
                    value: &storage_account_share_name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "useExtensionBundle".into(),
                    value: &use_extension_bundle_binding,
                },
                register_interface::ObjectField {
                    name: "version".into(),
                    value: &version_binding,
                },
                register_interface::ObjectField {
                    name: "virtualNetworkSubnetId".into(),
                    value: &virtual_network_subnet_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        StandardResult {
            app_service_plan_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("appServicePlanId"),
            ),
            app_settings: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("appSettings"),
            ),
            bundle_version: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("bundleVersion"),
            ),
            client_affinity_enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("clientAffinityEnabled"),
            ),
            client_certificate_mode: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("clientCertificateMode"),
            ),
            connection_strings: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("connectionStrings"),
            ),
            custom_domain_verification_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("customDomainVerificationId"),
            ),
            default_hostname: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("defaultHostname"),
            ),
            enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("enabled"),
            ),
            https_only: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("httpsOnly"),
            ),
            identity: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("identity"),
            ),
            kind: pulumi_wasm_rust::__private::into_domain(o.extract_field("kind")),
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            outbound_ip_addresses: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("outboundIpAddresses"),
            ),
            possible_outbound_ip_addresses: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("possibleOutboundIpAddresses"),
            ),
            public_network_access: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("publicNetworkAccess"),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            site_config: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("siteConfig"),
            ),
            site_credentials: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("siteCredentials"),
            ),
            storage_account_access_key: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("storageAccountAccessKey"),
            ),
            storage_account_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("storageAccountName"),
            ),
            storage_account_share_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("storageAccountShareName"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            use_extension_bundle: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("useExtensionBundle"),
            ),
            version: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("version"),
            ),
            virtual_network_subnet_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("virtualNetworkSubnetId"),
            ),
        }
    }
}
