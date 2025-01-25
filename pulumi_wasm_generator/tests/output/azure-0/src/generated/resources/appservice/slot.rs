/// Manages an App Service Slot (within an App Service).
///
/// !> **NOTE:** This resource has been deprecated in version 5.0 of the provider and will be removed in version 6.0. Please use `azure.appservice.LinuxWebAppSlot` and `azure.appservice.WindowsWebAppSlot` resources instead.
///
/// > **Note:** When using Slots - the `app_settings`, `connection_string` and `site_config` blocks on the `azure.appservice.AppService` resource will be overwritten when promoting a Slot using the `azure.appservice.ActiveSlot` resource.
///
/// ## Example Usage
///
/// ### NET 4.X)
///
/// ```yaml
/// resources:
///   server:
///     type: random:RandomId
///     properties:
///       keepers:
///         azi_id: 1
///       byteLength: 8
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: some-resource-group
///       location: West Europe
///   examplePlan:
///     type: azure:appservice:Plan
///     name: example
///     properties:
///       name: some-app-service-plan
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       sku:
///         tier: Standard
///         size: S1
///   exampleAppService:
///     type: azure:appservice:AppService
///     name: example
///     properties:
///       name: ${server.hex}
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       appServicePlanId: ${examplePlan.id}
///       siteConfig:
///         dotnetFrameworkVersion: v4.0
///       appSettings:
///         SOME_KEY: some-value
///       connectionStrings:
///         - name: Database
///           type: SQLServer
///           value: Server=some-server.mydomain.com;Integrated Security=SSPI
///   exampleSlot:
///     type: azure:appservice:Slot
///     name: example
///     properties:
///       name: ${server.hex}
///       appServiceName: ${exampleAppService.name}
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       appServicePlanId: ${examplePlan.id}
///       siteConfig:
///         dotnetFrameworkVersion: v4.0
///       appSettings:
///         SOME_KEY: some-value
///       connectionStrings:
///         - name: Database
///           type: SQLServer
///           value: Server=some-server.mydomain.com;Integrated Security=SSPI
/// ```
///
///
/// ### Java 1.8)
///
/// ```yaml
/// resources:
///   server:
///     type: random:RandomId
///     properties:
///       keepers:
///         azi_id: 1
///       byteLength: 8
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: some-resource-group
///       location: West Europe
///   examplePlan:
///     type: azure:appservice:Plan
///     name: example
///     properties:
///       name: some-app-service-plan
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       sku:
///         tier: Standard
///         size: S1
///   exampleAppService:
///     type: azure:appservice:AppService
///     name: example
///     properties:
///       name: ${server.hex}
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       appServicePlanId: ${examplePlan.id}
///       siteConfig:
///         javaVersion: '1.8'
///         javaContainer: JETTY
///         javaContainerVersion: '9.3'
///   exampleSlot:
///     type: azure:appservice:Slot
///     name: example
///     properties:
///       name: ${server.hex}
///       appServiceName: ${exampleAppService.name}
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       appServicePlanId: ${examplePlan.id}
///       siteConfig:
///         javaVersion: '1.8'
///         javaContainer: JETTY
///         javaContainerVersion: '9.3'
/// ```
///
/// ## Import
///
/// App Service Slots can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:appservice/slot:Slot instance1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Web/sites/website1/slots/instance1
/// ```
///
pub mod slot {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SlotArgs {
        /// The name of the App Service within which to create the App Service Slot. Changing this forces a new resource to be created.
        #[builder(into)]
        pub app_service_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The ID of the App Service Plan within which to create this App Service Slot. Changing this forces a new resource to be created.
        #[builder(into)]
        pub app_service_plan_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// A key-value pair of App Settings.
        #[builder(into, default)]
        pub app_settings: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A `auth_settings` block as defined below.
        #[builder(into, default)]
        pub auth_settings: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::appservice::SlotAuthSettings>,
        >,
        /// Should the App Service Slot send session affinity cookies, which route client requests in the same session to the same instance?
        #[builder(into, default)]
        pub client_affinity_enabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// An `connection_string` block as defined below.
        #[builder(into, default)]
        pub connection_strings: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::appservice::SlotConnectionString>>,
        >,
        /// Is the App Service Slot Enabled? Defaults to `true`.
        #[builder(into, default)]
        pub enabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Can the App Service Slot only be accessed via HTTPS? Defaults to `false`.
        #[builder(into, default)]
        pub https_only: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// An `identity` block as defined below.
        #[builder(into, default)]
        pub identity: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::appservice::SlotIdentity>,
        >,
        /// The User Assigned Identity Id used for looking up KeyVault secrets. The identity must be assigned to the application. See [Access vaults with a user-assigned identity](https://docs.microsoft.com/azure/app-service/app-service-key-vault-references#access-vaults-with-a-user-assigned-identity) for more information.
        #[builder(into, default)]
        pub key_vault_reference_identity_id: pulumi_wasm_rust::InputOrOutput<
            Option<String>,
        >,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A `logs` block as defined below.
        #[builder(into, default)]
        pub logs: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::appservice::SlotLogs>,
        >,
        /// Specifies the name of the App Service Slot component. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the resource group in which to create the App Service Slot component. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// A `site_config` object as defined below.
        #[builder(into, default)]
        pub site_config: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::appservice::SlotSiteConfig>,
        >,
        /// One or more `storage_account` blocks as defined below.
        #[builder(into, default)]
        pub storage_accounts: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::appservice::SlotStorageAccount>>,
        >,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct SlotResult {
        /// The name of the App Service within which to create the App Service Slot. Changing this forces a new resource to be created.
        pub app_service_name: pulumi_wasm_rust::Output<String>,
        /// The ID of the App Service Plan within which to create this App Service Slot. Changing this forces a new resource to be created.
        pub app_service_plan_id: pulumi_wasm_rust::Output<String>,
        /// A key-value pair of App Settings.
        pub app_settings: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// A `auth_settings` block as defined below.
        pub auth_settings: pulumi_wasm_rust::Output<
            super::super::types::appservice::SlotAuthSettings,
        >,
        /// Should the App Service Slot send session affinity cookies, which route client requests in the same session to the same instance?
        pub client_affinity_enabled: pulumi_wasm_rust::Output<bool>,
        /// An `connection_string` block as defined below.
        pub connection_strings: pulumi_wasm_rust::Output<
            Vec<super::super::types::appservice::SlotConnectionString>,
        >,
        /// The Default Hostname associated with the App Service Slot - such as `mysite.azurewebsites.net`
        pub default_site_hostname: pulumi_wasm_rust::Output<String>,
        /// Is the App Service Slot Enabled? Defaults to `true`.
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Can the App Service Slot only be accessed via HTTPS? Defaults to `false`.
        pub https_only: pulumi_wasm_rust::Output<Option<bool>>,
        /// An `identity` block as defined below.
        pub identity: pulumi_wasm_rust::Output<
            Option<super::super::types::appservice::SlotIdentity>,
        >,
        /// The User Assigned Identity Id used for looking up KeyVault secrets. The identity must be assigned to the application. See [Access vaults with a user-assigned identity](https://docs.microsoft.com/azure/app-service/app-service-key-vault-references#access-vaults-with-a-user-assigned-identity) for more information.
        pub key_vault_reference_identity_id: pulumi_wasm_rust::Output<String>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// A `logs` block as defined below.
        pub logs: pulumi_wasm_rust::Output<super::super::types::appservice::SlotLogs>,
        /// Specifies the name of the App Service Slot component. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the resource group in which to create the App Service Slot component. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A `site_config` object as defined below.
        pub site_config: pulumi_wasm_rust::Output<
            super::super::types::appservice::SlotSiteConfig,
        >,
        /// A `site_credential` block as defined below, which contains the site-level credentials used to publish to this App Service slot.
        pub site_credentials: pulumi_wasm_rust::Output<
            Vec<super::super::types::appservice::SlotSiteCredential>,
        >,
        /// One or more `storage_account` blocks as defined below.
        pub storage_accounts: pulumi_wasm_rust::Output<
            Vec<super::super::types::appservice::SlotStorageAccount>,
        >,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: SlotArgs,
    ) -> SlotResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let app_service_name_binding = args
            .app_service_name
            .get_output(context)
            .get_inner();
        let app_service_plan_id_binding = args
            .app_service_plan_id
            .get_output(context)
            .get_inner();
        let app_settings_binding = args.app_settings.get_output(context).get_inner();
        let auth_settings_binding = args.auth_settings.get_output(context).get_inner();
        let client_affinity_enabled_binding = args
            .client_affinity_enabled
            .get_output(context)
            .get_inner();
        let connection_strings_binding = args
            .connection_strings
            .get_output(context)
            .get_inner();
        let enabled_binding = args.enabled.get_output(context).get_inner();
        let https_only_binding = args.https_only.get_output(context).get_inner();
        let identity_binding = args.identity.get_output(context).get_inner();
        let key_vault_reference_identity_id_binding = args
            .key_vault_reference_identity_id
            .get_output(context)
            .get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let logs_binding = args.logs.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let site_config_binding = args.site_config.get_output(context).get_inner();
        let storage_accounts_binding = args
            .storage_accounts
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:appservice/slot:Slot".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "appServiceName".into(),
                    value: &app_service_name_binding,
                },
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
                    name: "clientAffinityEnabled".into(),
                    value: &client_affinity_enabled_binding,
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
                    name: "keyVaultReferenceIdentityId".into(),
                    value: &key_vault_reference_identity_id_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "logs".into(),
                    value: &logs_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
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
                    name: "storageAccounts".into(),
                    value: &storage_accounts_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "appServiceName".into(),
                },
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
                    name: "clientAffinityEnabled".into(),
                },
                register_interface::ResultField {
                    name: "connectionStrings".into(),
                },
                register_interface::ResultField {
                    name: "defaultSiteHostname".into(),
                },
                register_interface::ResultField {
                    name: "enabled".into(),
                },
                register_interface::ResultField {
                    name: "httpsOnly".into(),
                },
                register_interface::ResultField {
                    name: "identity".into(),
                },
                register_interface::ResultField {
                    name: "keyVaultReferenceIdentityId".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "logs".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
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
                    name: "storageAccounts".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        SlotResult {
            app_service_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("appServiceName").unwrap(),
            ),
            app_service_plan_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("appServicePlanId").unwrap(),
            ),
            app_settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("appSettings").unwrap(),
            ),
            auth_settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("authSettings").unwrap(),
            ),
            client_affinity_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clientAffinityEnabled").unwrap(),
            ),
            connection_strings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("connectionStrings").unwrap(),
            ),
            default_site_hostname: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultSiteHostname").unwrap(),
            ),
            enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enabled").unwrap(),
            ),
            https_only: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("httpsOnly").unwrap(),
            ),
            identity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("identity").unwrap(),
            ),
            key_vault_reference_identity_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("keyVaultReferenceIdentityId").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            logs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("logs").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
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
            storage_accounts: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageAccounts").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}
