/// Manages a Windows Web App Slot.
///
/// ## Example Usage
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
///             .name("example-resources")
///             .build_struct(),
///     );
///     let exampleServicePlan = service_plan::create(
///         "exampleServicePlan",
///         ServicePlanArgs::builder()
///             .location("${example.location}")
///             .name("example-plan")
///             .os_type("Windows")
///             .resource_group_name("${example.name}")
///             .sku_name("P1v2")
///             .build_struct(),
///     );
///     let exampleWindowsWebApp = windows_web_app::create(
///         "exampleWindowsWebApp",
///         WindowsWebAppArgs::builder()
///             .location("${exampleServicePlan.location}")
///             .name("example-windows-web-app")
///             .resource_group_name("${example.name}")
///             .service_plan_id("${exampleServicePlan.id}")
///             .site_config(WindowsWebAppSiteConfig::builder().build_struct())
///             .build_struct(),
///     );
///     let exampleWindowsWebAppSlot = windows_web_app_slot::create(
///         "exampleWindowsWebAppSlot",
///         WindowsWebAppSlotArgs::builder()
///             .app_service_id("${exampleWindowsWebApp.id}")
///             .name("example-slot")
///             .site_config(WindowsWebAppSlotSiteConfig::builder().build_struct())
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Windows Web Apps can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:appservice/windowsWebAppSlot:WindowsWebAppSlot example /subscriptions/12345678-1234-9876-4563-123456789012/resourceGroups/resGroup1/providers/Microsoft.Web/sites/site1/slots/slot1
/// ```
///
pub mod windows_web_app_slot {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WindowsWebAppSlotArgs {
        /// The ID of the Windows Web App this Deployment Slot will be part of. Changing this forces a new Windows Web App to be created.
        #[builder(into)]
        pub app_service_id: pulumi_wasm_rust::Output<String>,
        /// A map of key-value pairs of App Settings.
        #[builder(into, default)]
        pub app_settings: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// An `auth_settings` block as defined below.
        #[builder(into, default)]
        pub auth_settings: pulumi_wasm_rust::Output<
            Option<super::super::types::appservice::WindowsWebAppSlotAuthSettings>,
        >,
        /// An `auth_settings_v2` block as defined below.
        #[builder(into, default)]
        pub auth_settings_v2: pulumi_wasm_rust::Output<
            Option<super::super::types::appservice::WindowsWebAppSlotAuthSettingsV2>,
        >,
        /// A `backup` block as defined below.
        #[builder(into, default)]
        pub backup: pulumi_wasm_rust::Output<
            Option<super::super::types::appservice::WindowsWebAppSlotBackup>,
        >,
        /// Should Client Affinity be enabled?
        #[builder(into, default)]
        pub client_affinity_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Should Client Certificates be enabled?
        #[builder(into, default)]
        pub client_certificate_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Paths to exclude when using client certificates, separated by ;
        #[builder(into, default)]
        pub client_certificate_exclusion_paths: pulumi_wasm_rust::Output<Option<String>>,
        /// The Client Certificate mode. Possible values are `Required`, `Optional`, and `OptionalInteractiveUser`. This property has no effect when `client_cert_enabled` is `false`. Defaults to `Required`.
        #[builder(into, default)]
        pub client_certificate_mode: pulumi_wasm_rust::Output<Option<String>>,
        /// One or more `connection_string` blocks as defined below.
        #[builder(into, default)]
        pub connection_strings: pulumi_wasm_rust::Output<
            Option<
                Vec<super::super::types::appservice::WindowsWebAppSlotConnectionString>,
            >,
        >,
        /// Should the Windows Web App Slot be enabled? Defaults to `true`.
        #[builder(into, default)]
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Should the default FTP Basic Authentication publishing profile be enabled. Defaults to `true`.
        #[builder(into, default)]
        pub ftp_publish_basic_authentication_enabled: pulumi_wasm_rust::Output<
            Option<bool>,
        >,
        /// Should the Windows Web App Slot require HTTPS connections. Defaults to `false`.
        #[builder(into, default)]
        pub https_only: pulumi_wasm_rust::Output<Option<bool>>,
        /// An `identity` block as defined below.
        #[builder(into, default)]
        pub identity: pulumi_wasm_rust::Output<
            Option<super::super::types::appservice::WindowsWebAppSlotIdentity>,
        >,
        /// The User Assigned Identity ID used for accessing KeyVault secrets. The identity must be assigned to the application in the `identity` block. [For more information see - Access vaults with a user-assigned identity](https://docs.microsoft.com/azure/app-service/app-service-key-vault-references#access-vaults-with-a-user-assigned-identity)
        #[builder(into, default)]
        pub key_vault_reference_identity_id: pulumi_wasm_rust::Output<Option<String>>,
        /// A `logs` block as defined below.
        #[builder(into, default)]
        pub logs: pulumi_wasm_rust::Output<
            Option<super::super::types::appservice::WindowsWebAppSlotLogs>,
        >,
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Should public network access be enabled for the Web App. Defaults to `true`.
        #[builder(into, default)]
        pub public_network_access_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The ID of the Service Plan in which to run this slot. If not specified the same Service Plan as the Windows Web App will be used.
        ///
        /// > **Note:** `service_plan_id` should only be specified if it differs from the Service Plan of the associated Windows Web App.
        #[builder(into, default)]
        pub service_plan_id: pulumi_wasm_rust::Output<Option<String>>,
        /// A `site_config` block as defined below.
        #[builder(into)]
        pub site_config: pulumi_wasm_rust::Output<
            super::super::types::appservice::WindowsWebAppSlotSiteConfig,
        >,
        /// One or more `storage_account` blocks as defined below.
        ///
        /// > **Note:** Using this value requires `WEBSITE_RUN_FROM_PACKAGE=1` to be set on the App in `app_settings`. Refer to the [Azure docs](https://docs.microsoft.com/en-us/azure/app-service/deploy-run-package) for further details.
        #[builder(into, default)]
        pub storage_accounts: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::appservice::WindowsWebAppSlotStorageAccount>>,
        >,
        /// A mapping of tags which should be assigned to the Windows Web App Slot.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        #[builder(into, default)]
        pub virtual_network_subnet_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Should the default WebDeploy Basic Authentication publishing credentials enabled. Defaults to `true`.
        ///
        /// > **NOTE:** Setting this value to true will disable the ability to use `zip_deploy_file` which currently relies on the default publishing profile.
        #[builder(into, default)]
        pub webdeploy_publish_basic_authentication_enabled: pulumi_wasm_rust::Output<
            Option<bool>,
        >,
        /// The local path and filename of the Zip packaged application to deploy to this Windows Web App.
        #[builder(into, default)]
        pub zip_deploy_file: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct WindowsWebAppSlotResult {
        /// The ID of the Windows Web App this Deployment Slot will be part of. Changing this forces a new Windows Web App to be created.
        pub app_service_id: pulumi_wasm_rust::Output<String>,
        /// A map of key-value pairs of App Settings.
        pub app_settings: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// An `auth_settings` block as defined below.
        pub auth_settings: pulumi_wasm_rust::Output<
            Option<super::super::types::appservice::WindowsWebAppSlotAuthSettings>,
        >,
        /// An `auth_settings_v2` block as defined below.
        pub auth_settings_v2: pulumi_wasm_rust::Output<
            Option<super::super::types::appservice::WindowsWebAppSlotAuthSettingsV2>,
        >,
        /// A `backup` block as defined below.
        pub backup: pulumi_wasm_rust::Output<
            Option<super::super::types::appservice::WindowsWebAppSlotBackup>,
        >,
        /// Should Client Affinity be enabled?
        pub client_affinity_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Should Client Certificates be enabled?
        pub client_certificate_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Paths to exclude when using client certificates, separated by ;
        pub client_certificate_exclusion_paths: pulumi_wasm_rust::Output<Option<String>>,
        /// The Client Certificate mode. Possible values are `Required`, `Optional`, and `OptionalInteractiveUser`. This property has no effect when `client_cert_enabled` is `false`. Defaults to `Required`.
        pub client_certificate_mode: pulumi_wasm_rust::Output<Option<String>>,
        /// One or more `connection_string` blocks as defined below.
        pub connection_strings: pulumi_wasm_rust::Output<
            Option<
                Vec<super::super::types::appservice::WindowsWebAppSlotConnectionString>,
            >,
        >,
        /// The identifier used by App Service to perform domain ownership verification via DNS TXT record.
        pub custom_domain_verification_id: pulumi_wasm_rust::Output<String>,
        /// The default hostname of the Windows Web App Slot.
        pub default_hostname: pulumi_wasm_rust::Output<String>,
        /// Should the Windows Web App Slot be enabled? Defaults to `true`.
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Should the default FTP Basic Authentication publishing profile be enabled. Defaults to `true`.
        pub ftp_publish_basic_authentication_enabled: pulumi_wasm_rust::Output<
            Option<bool>,
        >,
        /// The ID of the App Service Environment used by App Service Slot.
        pub hosting_environment_id: pulumi_wasm_rust::Output<String>,
        /// Should the Windows Web App Slot require HTTPS connections. Defaults to `false`.
        pub https_only: pulumi_wasm_rust::Output<Option<bool>>,
        /// An `identity` block as defined below.
        pub identity: pulumi_wasm_rust::Output<
            Option<super::super::types::appservice::WindowsWebAppSlotIdentity>,
        >,
        /// The User Assigned Identity ID used for accessing KeyVault secrets. The identity must be assigned to the application in the `identity` block. [For more information see - Access vaults with a user-assigned identity](https://docs.microsoft.com/azure/app-service/app-service-key-vault-references#access-vaults-with-a-user-assigned-identity)
        pub key_vault_reference_identity_id: pulumi_wasm_rust::Output<String>,
        /// The Kind value for this Windows Web App Slot.
        pub kind: pulumi_wasm_rust::Output<String>,
        /// A `logs` block as defined below.
        pub logs: pulumi_wasm_rust::Output<
            Option<super::super::types::appservice::WindowsWebAppSlotLogs>,
        >,
        pub name: pulumi_wasm_rust::Output<String>,
        /// A list of outbound IP addresses - such as `["52.23.25.3", "52.143.43.12"]`
        pub outbound_ip_address_lists: pulumi_wasm_rust::Output<Vec<String>>,
        /// A comma separated list of outbound IP addresses - such as `52.23.25.3,52.143.43.12`.
        pub outbound_ip_addresses: pulumi_wasm_rust::Output<String>,
        /// A list of possible outbound ip address.
        pub possible_outbound_ip_address_lists: pulumi_wasm_rust::Output<Vec<String>>,
        /// A comma separated list of outbound IP addresses - such as `52.23.25.3,52.143.43.12,52.143.43.17` - not all of which are necessarily in use. Superset of `outbound_ip_addresses`.
        pub possible_outbound_ip_addresses: pulumi_wasm_rust::Output<String>,
        /// Should public network access be enabled for the Web App. Defaults to `true`.
        pub public_network_access_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The ID of the Service Plan in which to run this slot. If not specified the same Service Plan as the Windows Web App will be used.
        ///
        /// > **Note:** `service_plan_id` should only be specified if it differs from the Service Plan of the associated Windows Web App.
        pub service_plan_id: pulumi_wasm_rust::Output<Option<String>>,
        /// A `site_config` block as defined below.
        pub site_config: pulumi_wasm_rust::Output<
            super::super::types::appservice::WindowsWebAppSlotSiteConfig,
        >,
        /// A `site_credential` block as defined below.
        pub site_credentials: pulumi_wasm_rust::Output<
            Vec<super::super::types::appservice::WindowsWebAppSlotSiteCredential>,
        >,
        /// One or more `storage_account` blocks as defined below.
        ///
        /// > **Note:** Using this value requires `WEBSITE_RUN_FROM_PACKAGE=1` to be set on the App in `app_settings`. Refer to the [Azure docs](https://docs.microsoft.com/en-us/azure/app-service/deploy-run-package) for further details.
        pub storage_accounts: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::appservice::WindowsWebAppSlotStorageAccount>>,
        >,
        /// A mapping of tags which should be assigned to the Windows Web App Slot.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub virtual_network_subnet_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Should the default WebDeploy Basic Authentication publishing credentials enabled. Defaults to `true`.
        ///
        /// > **NOTE:** Setting this value to true will disable the ability to use `zip_deploy_file` which currently relies on the default publishing profile.
        pub webdeploy_publish_basic_authentication_enabled: pulumi_wasm_rust::Output<
            Option<bool>,
        >,
        /// The local path and filename of the Zip packaged application to deploy to this Windows Web App.
        pub zip_deploy_file: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: WindowsWebAppSlotArgs) -> WindowsWebAppSlotResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let app_service_id_binding = args.app_service_id.get_inner();
        let app_settings_binding = args.app_settings.get_inner();
        let auth_settings_binding = args.auth_settings.get_inner();
        let auth_settings_v2_binding = args.auth_settings_v2.get_inner();
        let backup_binding = args.backup.get_inner();
        let client_affinity_enabled_binding = args.client_affinity_enabled.get_inner();
        let client_certificate_enabled_binding = args
            .client_certificate_enabled
            .get_inner();
        let client_certificate_exclusion_paths_binding = args
            .client_certificate_exclusion_paths
            .get_inner();
        let client_certificate_mode_binding = args.client_certificate_mode.get_inner();
        let connection_strings_binding = args.connection_strings.get_inner();
        let enabled_binding = args.enabled.get_inner();
        let ftp_publish_basic_authentication_enabled_binding = args
            .ftp_publish_basic_authentication_enabled
            .get_inner();
        let https_only_binding = args.https_only.get_inner();
        let identity_binding = args.identity.get_inner();
        let key_vault_reference_identity_id_binding = args
            .key_vault_reference_identity_id
            .get_inner();
        let logs_binding = args.logs.get_inner();
        let name_binding = args.name.get_inner();
        let public_network_access_enabled_binding = args
            .public_network_access_enabled
            .get_inner();
        let service_plan_id_binding = args.service_plan_id.get_inner();
        let site_config_binding = args.site_config.get_inner();
        let storage_accounts_binding = args.storage_accounts.get_inner();
        let tags_binding = args.tags.get_inner();
        let virtual_network_subnet_id_binding = args
            .virtual_network_subnet_id
            .get_inner();
        let webdeploy_publish_basic_authentication_enabled_binding = args
            .webdeploy_publish_basic_authentication_enabled
            .get_inner();
        let zip_deploy_file_binding = args.zip_deploy_file.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:appservice/windowsWebAppSlot:WindowsWebAppSlot".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "appServiceId".into(),
                    value: &app_service_id_binding,
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
                    name: "authSettingsV2".into(),
                    value: &auth_settings_v2_binding,
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
                    name: "clientCertificateEnabled".into(),
                    value: &client_certificate_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "clientCertificateExclusionPaths".into(),
                    value: &client_certificate_exclusion_paths_binding,
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
                    name: "ftpPublishBasicAuthenticationEnabled".into(),
                    value: &ftp_publish_basic_authentication_enabled_binding,
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
                    name: "logs".into(),
                    value: &logs_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "publicNetworkAccessEnabled".into(),
                    value: &public_network_access_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "servicePlanId".into(),
                    value: &service_plan_id_binding,
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
                register_interface::ObjectField {
                    name: "virtualNetworkSubnetId".into(),
                    value: &virtual_network_subnet_id_binding,
                },
                register_interface::ObjectField {
                    name: "webdeployPublishBasicAuthenticationEnabled".into(),
                    value: &webdeploy_publish_basic_authentication_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "zipDeployFile".into(),
                    value: &zip_deploy_file_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "appServiceId".into(),
                },
                register_interface::ResultField {
                    name: "appSettings".into(),
                },
                register_interface::ResultField {
                    name: "authSettings".into(),
                },
                register_interface::ResultField {
                    name: "authSettingsV2".into(),
                },
                register_interface::ResultField {
                    name: "backup".into(),
                },
                register_interface::ResultField {
                    name: "clientAffinityEnabled".into(),
                },
                register_interface::ResultField {
                    name: "clientCertificateEnabled".into(),
                },
                register_interface::ResultField {
                    name: "clientCertificateExclusionPaths".into(),
                },
                register_interface::ResultField {
                    name: "clientCertificateMode".into(),
                },
                register_interface::ResultField {
                    name: "connectionStrings".into(),
                },
                register_interface::ResultField {
                    name: "customDomainVerificationId".into(),
                },
                register_interface::ResultField {
                    name: "defaultHostname".into(),
                },
                register_interface::ResultField {
                    name: "enabled".into(),
                },
                register_interface::ResultField {
                    name: "ftpPublishBasicAuthenticationEnabled".into(),
                },
                register_interface::ResultField {
                    name: "hostingEnvironmentId".into(),
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
                    name: "kind".into(),
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
                    name: "publicNetworkAccessEnabled".into(),
                },
                register_interface::ResultField {
                    name: "servicePlanId".into(),
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
                register_interface::ResultField {
                    name: "virtualNetworkSubnetId".into(),
                },
                register_interface::ResultField {
                    name: "webdeployPublishBasicAuthenticationEnabled".into(),
                },
                register_interface::ResultField {
                    name: "zipDeployFile".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        WindowsWebAppSlotResult {
            app_service_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("appServiceId").unwrap(),
            ),
            app_settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("appSettings").unwrap(),
            ),
            auth_settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("authSettings").unwrap(),
            ),
            auth_settings_v2: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("authSettingsV2").unwrap(),
            ),
            backup: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("backup").unwrap(),
            ),
            client_affinity_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clientAffinityEnabled").unwrap(),
            ),
            client_certificate_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clientCertificateEnabled").unwrap(),
            ),
            client_certificate_exclusion_paths: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clientCertificateExclusionPaths").unwrap(),
            ),
            client_certificate_mode: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clientCertificateMode").unwrap(),
            ),
            connection_strings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("connectionStrings").unwrap(),
            ),
            custom_domain_verification_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customDomainVerificationId").unwrap(),
            ),
            default_hostname: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultHostname").unwrap(),
            ),
            enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enabled").unwrap(),
            ),
            ftp_publish_basic_authentication_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ftpPublishBasicAuthenticationEnabled").unwrap(),
            ),
            hosting_environment_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hostingEnvironmentId").unwrap(),
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
            kind: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kind").unwrap(),
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
            public_network_access_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("publicNetworkAccessEnabled").unwrap(),
            ),
            service_plan_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("servicePlanId").unwrap(),
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
            virtual_network_subnet_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("virtualNetworkSubnetId").unwrap(),
            ),
            webdeploy_publish_basic_authentication_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("webdeployPublishBasicAuthenticationEnabled").unwrap(),
            ),
            zip_deploy_file: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("zipDeployFile").unwrap(),
            ),
        }
    }
}
