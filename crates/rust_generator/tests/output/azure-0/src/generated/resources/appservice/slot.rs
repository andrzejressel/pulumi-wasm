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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod slot {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SlotArgs {
        /// The name of the App Service within which to create the App Service Slot. Changing this forces a new resource to be created.
        #[builder(into)]
        pub app_service_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the App Service Plan within which to create this App Service Slot. Changing this forces a new resource to be created.
        #[builder(into)]
        pub app_service_plan_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A key-value pair of App Settings.
        #[builder(into, default)]
        pub app_settings: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A `auth_settings` block as defined below.
        #[builder(into, default)]
        pub auth_settings: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::appservice::SlotAuthSettings>,
        >,
        /// Should the App Service Slot send session affinity cookies, which route client requests in the same session to the same instance?
        #[builder(into, default)]
        pub client_affinity_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// An `connection_string` block as defined below.
        #[builder(into, default)]
        pub connection_strings: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::appservice::SlotConnectionString>>,
        >,
        /// Is the App Service Slot Enabled? Defaults to `true`.
        #[builder(into, default)]
        pub enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Can the App Service Slot only be accessed via HTTPS? Defaults to `false`.
        #[builder(into, default)]
        pub https_only: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// An `identity` block as defined below.
        #[builder(into, default)]
        pub identity: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::appservice::SlotIdentity>,
        >,
        /// The User Assigned Identity Id used for looking up KeyVault secrets. The identity must be assigned to the application. See [Access vaults with a user-assigned identity](https://docs.microsoft.com/azure/app-service/app-service-key-vault-references#access-vaults-with-a-user-assigned-identity) for more information.
        #[builder(into, default)]
        pub key_vault_reference_identity_id: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `logs` block as defined below.
        #[builder(into, default)]
        pub logs: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::appservice::SlotLogs>,
        >,
        /// Specifies the name of the App Service Slot component. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the resource group in which to create the App Service Slot component. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A `site_config` object as defined below.
        #[builder(into, default)]
        pub site_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::appservice::SlotSiteConfig>,
        >,
        /// One or more `storage_account` blocks as defined below.
        #[builder(into, default)]
        pub storage_accounts: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::appservice::SlotStorageAccount>>,
        >,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct SlotResult {
        /// The name of the App Service within which to create the App Service Slot. Changing this forces a new resource to be created.
        pub app_service_name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the App Service Plan within which to create this App Service Slot. Changing this forces a new resource to be created.
        pub app_service_plan_id: pulumi_gestalt_rust::Output<String>,
        /// A key-value pair of App Settings.
        pub app_settings: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// A `auth_settings` block as defined below.
        pub auth_settings: pulumi_gestalt_rust::Output<
            super::super::types::appservice::SlotAuthSettings,
        >,
        /// Should the App Service Slot send session affinity cookies, which route client requests in the same session to the same instance?
        pub client_affinity_enabled: pulumi_gestalt_rust::Output<bool>,
        /// An `connection_string` block as defined below.
        pub connection_strings: pulumi_gestalt_rust::Output<
            Vec<super::super::types::appservice::SlotConnectionString>,
        >,
        /// The Default Hostname associated with the App Service Slot - such as `mysite.azurewebsites.net`
        pub default_site_hostname: pulumi_gestalt_rust::Output<String>,
        /// Is the App Service Slot Enabled? Defaults to `true`.
        pub enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Can the App Service Slot only be accessed via HTTPS? Defaults to `false`.
        pub https_only: pulumi_gestalt_rust::Output<Option<bool>>,
        /// An `identity` block as defined below.
        pub identity: pulumi_gestalt_rust::Output<
            Option<super::super::types::appservice::SlotIdentity>,
        >,
        /// The User Assigned Identity Id used for looking up KeyVault secrets. The identity must be assigned to the application. See [Access vaults with a user-assigned identity](https://docs.microsoft.com/azure/app-service/app-service-key-vault-references#access-vaults-with-a-user-assigned-identity) for more information.
        pub key_vault_reference_identity_id: pulumi_gestalt_rust::Output<String>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// A `logs` block as defined below.
        pub logs: pulumi_gestalt_rust::Output<super::super::types::appservice::SlotLogs>,
        /// Specifies the name of the App Service Slot component. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the resource group in which to create the App Service Slot component. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A `site_config` object as defined below.
        pub site_config: pulumi_gestalt_rust::Output<
            super::super::types::appservice::SlotSiteConfig,
        >,
        /// A `site_credential` block as defined below, which contains the site-level credentials used to publish to this App Service slot.
        pub site_credentials: pulumi_gestalt_rust::Output<
            Vec<super::super::types::appservice::SlotSiteCredential>,
        >,
        /// One or more `storage_account` blocks as defined below.
        pub storage_accounts: pulumi_gestalt_rust::Output<
            Vec<super::super::types::appservice::SlotStorageAccount>,
        >,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SlotArgs,
    ) -> SlotResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let app_service_name_binding = args.app_service_name.get_output(context);
        let app_service_plan_id_binding = args.app_service_plan_id.get_output(context);
        let app_settings_binding = args.app_settings.get_output(context);
        let auth_settings_binding = args.auth_settings.get_output(context);
        let client_affinity_enabled_binding = args
            .client_affinity_enabled
            .get_output(context);
        let connection_strings_binding = args.connection_strings.get_output(context);
        let enabled_binding = args.enabled.get_output(context);
        let https_only_binding = args.https_only.get_output(context);
        let identity_binding = args.identity.get_output(context);
        let key_vault_reference_identity_id_binding = args
            .key_vault_reference_identity_id
            .get_output(context);
        let location_binding = args.location.get_output(context);
        let logs_binding = args.logs.get_output(context);
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let site_config_binding = args.site_config.get_output(context);
        let storage_accounts_binding = args.storage_accounts.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:appservice/slot:Slot".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "appServiceName".into(),
                    value: &app_service_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "appServicePlanId".into(),
                    value: &app_service_plan_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "appSettings".into(),
                    value: &app_settings_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "authSettings".into(),
                    value: &auth_settings_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clientAffinityEnabled".into(),
                    value: &client_affinity_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "connectionStrings".into(),
                    value: &connection_strings_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "httpsOnly".into(),
                    value: &https_only_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "identity".into(),
                    value: &identity_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "keyVaultReferenceIdentityId".into(),
                    value: &key_vault_reference_identity_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "logs".into(),
                    value: &logs_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "siteConfig".into(),
                    value: &site_config_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storageAccounts".into(),
                    value: &storage_accounts_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        SlotResult {
            app_service_name: o.get_field("appServiceName"),
            app_service_plan_id: o.get_field("appServicePlanId"),
            app_settings: o.get_field("appSettings"),
            auth_settings: o.get_field("authSettings"),
            client_affinity_enabled: o.get_field("clientAffinityEnabled"),
            connection_strings: o.get_field("connectionStrings"),
            default_site_hostname: o.get_field("defaultSiteHostname"),
            enabled: o.get_field("enabled"),
            https_only: o.get_field("httpsOnly"),
            identity: o.get_field("identity"),
            key_vault_reference_identity_id: o.get_field("keyVaultReferenceIdentityId"),
            location: o.get_field("location"),
            logs: o.get_field("logs"),
            name: o.get_field("name"),
            resource_group_name: o.get_field("resourceGroupName"),
            site_config: o.get_field("siteConfig"),
            site_credentials: o.get_field("siteCredentials"),
            storage_accounts: o.get_field("storageAccounts"),
            tags: o.get_field("tags"),
        }
    }
}
