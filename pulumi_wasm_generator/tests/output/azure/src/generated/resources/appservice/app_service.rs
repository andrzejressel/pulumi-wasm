/// Manages an App Service (within an App Service Plan).
///
/// !> **NOTE:** This resource has been deprecated in version 5.0 of the provider and will be removed in version 6.0. Please use `azure.appservice.LinuxWebApp` and `azure.appservice.WindowsWebApp` resources instead.
///
/// > **Note:** When using Slots - the `app_settings`, `connection_string` and `site_config` blocks on the `azure.appservice.AppService` resource will be overwritten when promoting a Slot using the `azure.appservice.ActiveSlot` resource.
///
/// ## Example Usage
///
/// This example provisions a Windows App Service.
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Europe
///   examplePlan:
///     type: azure:appservice:Plan
///     name: example
///     properties:
///       name: example-appserviceplan
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       sku:
///         tier: Standard
///         size: S1
///   exampleAppService:
///     type: azure:appservice:AppService
///     name: example
///     properties:
///       name: example-app-service
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       appServicePlanId: ${examplePlan.id}
///       siteConfig:
///         dotnetFrameworkVersion: v4.0
///         scmType: LocalGit
///       appSettings:
///         SOME_KEY: some-value
///       connectionStrings:
///         - name: Database
///           type: SQLServer
///           value: Server=some-server.mydomain.com;Integrated Security=SSPI
/// ```
///
/// ## Import
///
/// App Services can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:appservice/appService:AppService instance1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Web/sites/instance1
/// ```
///
pub mod app_service {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AppServiceArgs {
        /// The ID of the App Service Plan within which to create this App Service.
        #[builder(into)]
        pub app_service_plan_id: pulumi_wasm_rust::Output<String>,
        /// A key-value pair of App Settings.
        #[builder(into, default)]
        pub app_settings: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A `auth_settings` block as defined below.
        #[builder(into, default)]
        pub auth_settings: pulumi_wasm_rust::Output<
            Option<super::super::types::appservice::AppServiceAuthSettings>,
        >,
        /// A `backup` block as defined below.
        #[builder(into, default)]
        pub backup: pulumi_wasm_rust::Output<
            Option<super::super::types::appservice::AppServiceBackup>,
        >,
        /// Should the App Service send session affinity cookies, which route client requests in the same session to the same instance?
        #[builder(into, default)]
        pub client_affinity_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Does the App Service require client certificates for incoming requests? Defaults to `false`.
        #[builder(into, default)]
        pub client_cert_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Mode of client certificates for this App Service. Possible values are `Required`, `Optional` and `OptionalInteractiveUser`. If this parameter is set, `client_cert_enabled` must be set to `true`, otherwise this parameter is ignored.
        #[builder(into, default)]
        pub client_cert_mode: pulumi_wasm_rust::Output<Option<String>>,
        /// One or more `connection_string` blocks as defined below.
        #[builder(into, default)]
        pub connection_strings: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::appservice::AppServiceConnectionString>>,
        >,
        /// Is the App Service Enabled? Defaults to `true`.
        #[builder(into, default)]
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Can the App Service only be accessed via HTTPS? Defaults to `false`.
        #[builder(into, default)]
        pub https_only: pulumi_wasm_rust::Output<Option<bool>>,
        /// An `identity` block as defined below.
        #[builder(into, default)]
        pub identity: pulumi_wasm_rust::Output<
            Option<super::super::types::appservice::AppServiceIdentity>,
        >,
        /// The User Assigned Identity Id used for looking up KeyVault secrets. The identity must be assigned to the application. [For more information see - Access vaults with a user-assigned identity](https://docs.microsoft.com/azure/app-service/app-service-key-vault-references#access-vaults-with-a-user-assigned-identity)
        #[builder(into, default)]
        pub key_vault_reference_identity_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// A `logs` block as defined below.
        #[builder(into, default)]
        pub logs: pulumi_wasm_rust::Output<
            Option<super::super::types::appservice::AppServiceLogs>,
        >,
        /// Specifies the name of the App Service. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the resource group in which to create the App Service. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A `site_config` block as defined below.
        #[builder(into, default)]
        pub site_config: pulumi_wasm_rust::Output<
            Option<super::super::types::appservice::AppServiceSiteConfig>,
        >,
        /// A `source_control` block as defined below.
        #[builder(into, default)]
        pub source_control: pulumi_wasm_rust::Output<
            Option<super::super::types::appservice::AppServiceSourceControl>,
        >,
        /// One or more `storage_account` blocks as defined below.
        #[builder(into, default)]
        pub storage_accounts: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::appservice::AppServiceStorageAccount>>,
        >,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct AppServiceResult {
        /// The ID of the App Service Plan within which to create this App Service.
        pub app_service_plan_id: pulumi_wasm_rust::Output<String>,
        /// A key-value pair of App Settings.
        pub app_settings: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// A `auth_settings` block as defined below.
        pub auth_settings: pulumi_wasm_rust::Output<
            super::super::types::appservice::AppServiceAuthSettings,
        >,
        /// A `backup` block as defined below.
        pub backup: pulumi_wasm_rust::Output<
            Option<super::super::types::appservice::AppServiceBackup>,
        >,
        /// Should the App Service send session affinity cookies, which route client requests in the same session to the same instance?
        pub client_affinity_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Does the App Service require client certificates for incoming requests? Defaults to `false`.
        pub client_cert_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Mode of client certificates for this App Service. Possible values are `Required`, `Optional` and `OptionalInteractiveUser`. If this parameter is set, `client_cert_enabled` must be set to `true`, otherwise this parameter is ignored.
        pub client_cert_mode: pulumi_wasm_rust::Output<String>,
        /// One or more `connection_string` blocks as defined below.
        pub connection_strings: pulumi_wasm_rust::Output<
            Vec<super::super::types::appservice::AppServiceConnectionString>,
        >,
        /// An identifier used by App Service to perform domain ownership verification via DNS TXT record.
        pub custom_domain_verification_id: pulumi_wasm_rust::Output<String>,
        /// The Default Hostname associated with the App Service - such as `mysite.azurewebsites.net`
        pub default_site_hostname: pulumi_wasm_rust::Output<String>,
        /// Is the App Service Enabled? Defaults to `true`.
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Can the App Service only be accessed via HTTPS? Defaults to `false`.
        pub https_only: pulumi_wasm_rust::Output<Option<bool>>,
        /// An `identity` block as defined below.
        pub identity: pulumi_wasm_rust::Output<
            Option<super::super::types::appservice::AppServiceIdentity>,
        >,
        /// The User Assigned Identity Id used for looking up KeyVault secrets. The identity must be assigned to the application. [For more information see - Access vaults with a user-assigned identity](https://docs.microsoft.com/azure/app-service/app-service-key-vault-references#access-vaults-with-a-user-assigned-identity)
        pub key_vault_reference_identity_id: pulumi_wasm_rust::Output<String>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// A `logs` block as defined below.
        pub logs: pulumi_wasm_rust::Output<
            super::super::types::appservice::AppServiceLogs,
        >,
        /// Specifies the name of the App Service. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A list of outbound IP addresses - such as `["52.23.25.3", "52.143.43.12"]`
        pub outbound_ip_address_lists: pulumi_wasm_rust::Output<Vec<String>>,
        /// A comma separated list of outbound IP addresses - such as `52.23.25.3,52.143.43.12`
        pub outbound_ip_addresses: pulumi_wasm_rust::Output<String>,
        /// A list of outbound IP addresses - such as `["52.23.25.3", "52.143.43.12", "52.143.43.17"]` - not all of which are necessarily in use. Superset of `outbound_ip_address_list`.
        pub possible_outbound_ip_address_lists: pulumi_wasm_rust::Output<Vec<String>>,
        /// A comma separated list of outbound IP addresses - such as `52.23.25.3,52.143.43.12,52.143.43.17` - not all of which are necessarily in use. Superset of `outbound_ip_addresses`.
        pub possible_outbound_ip_addresses: pulumi_wasm_rust::Output<String>,
        /// The name of the resource group in which to create the App Service. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A `site_config` block as defined below.
        pub site_config: pulumi_wasm_rust::Output<
            super::super::types::appservice::AppServiceSiteConfig,
        >,
        /// A `site_credential` block as defined below, which contains the site-level credentials used to publish to this App Service.
        pub site_credentials: pulumi_wasm_rust::Output<
            Vec<super::super::types::appservice::AppServiceSiteCredential>,
        >,
        /// A `source_control` block as defined below.
        pub source_control: pulumi_wasm_rust::Output<
            super::super::types::appservice::AppServiceSourceControl,
        >,
        /// One or more `storage_account` blocks as defined below.
        pub storage_accounts: pulumi_wasm_rust::Output<
            Vec<super::super::types::appservice::AppServiceStorageAccount>,
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
    pub fn create(name: &str, args: AppServiceArgs) -> AppServiceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let app_service_plan_id_binding = args.app_service_plan_id.get_inner();
        let app_settings_binding = args.app_settings.get_inner();
        let auth_settings_binding = args.auth_settings.get_inner();
        let backup_binding = args.backup.get_inner();
        let client_affinity_enabled_binding = args.client_affinity_enabled.get_inner();
        let client_cert_enabled_binding = args.client_cert_enabled.get_inner();
        let client_cert_mode_binding = args.client_cert_mode.get_inner();
        let connection_strings_binding = args.connection_strings.get_inner();
        let enabled_binding = args.enabled.get_inner();
        let https_only_binding = args.https_only.get_inner();
        let identity_binding = args.identity.get_inner();
        let key_vault_reference_identity_id_binding = args
            .key_vault_reference_identity_id
            .get_inner();
        let location_binding = args.location.get_inner();
        let logs_binding = args.logs.get_inner();
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let site_config_binding = args.site_config.get_inner();
        let source_control_binding = args.source_control.get_inner();
        let storage_accounts_binding = args.storage_accounts.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:appservice/appService:AppService".into(),
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
                    name: "backup".into(),
                    value: &backup_binding,
                },
                register_interface::ObjectField {
                    name: "clientAffinityEnabled".into(),
                    value: &client_affinity_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "clientCertEnabled".into(),
                    value: &client_cert_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "clientCertMode".into(),
                    value: &client_cert_mode_binding,
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
                    name: "sourceControl".into(),
                    value: &source_control_binding,
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
                    name: "appServicePlanId".into(),
                },
                register_interface::ResultField {
                    name: "appSettings".into(),
                },
                register_interface::ResultField {
                    name: "authSettings".into(),
                },
                register_interface::ResultField {
                    name: "backup".into(),
                },
                register_interface::ResultField {
                    name: "clientAffinityEnabled".into(),
                },
                register_interface::ResultField {
                    name: "clientCertEnabled".into(),
                },
                register_interface::ResultField {
                    name: "clientCertMode".into(),
                },
                register_interface::ResultField {
                    name: "connectionStrings".into(),
                },
                register_interface::ResultField {
                    name: "customDomainVerificationId".into(),
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
                    name: "outboundIpAddressLists".into(),
                },
                register_interface::ResultField {
                    name: "outboundIpAddresses".into(),
                },
                register_interface::ResultField {
                    name: "possibleOutboundIpAddressLists".into(),
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
                    name: "sourceControl".into(),
                },
                register_interface::ResultField {
                    name: "storageAccounts".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        AppServiceResult {
            app_service_plan_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("appServicePlanId").unwrap(),
            ),
            app_settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("appSettings").unwrap(),
            ),
            auth_settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("authSettings").unwrap(),
            ),
            backup: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("backup").unwrap(),
            ),
            client_affinity_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clientAffinityEnabled").unwrap(),
            ),
            client_cert_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clientCertEnabled").unwrap(),
            ),
            client_cert_mode: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clientCertMode").unwrap(),
            ),
            connection_strings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("connectionStrings").unwrap(),
            ),
            custom_domain_verification_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customDomainVerificationId").unwrap(),
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
            outbound_ip_address_lists: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("outboundIpAddressLists").unwrap(),
            ),
            outbound_ip_addresses: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("outboundIpAddresses").unwrap(),
            ),
            possible_outbound_ip_address_lists: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("possibleOutboundIpAddressLists").unwrap(),
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
            source_control: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceControl").unwrap(),
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
