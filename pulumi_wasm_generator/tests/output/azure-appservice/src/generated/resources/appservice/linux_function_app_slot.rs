/// Manages a Linux Function App Slot.
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
///     let exampleAccount = account::create(
///         "exampleAccount",
///         AccountArgs::builder()
///             .account_replication_type("LRS")
///             .account_tier("Standard")
///             .location("${example.location}")
///             .name("linuxfunctionappsa")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleLinuxFunctionApp = linux_function_app::create(
///         "exampleLinuxFunctionApp",
///         LinuxFunctionAppArgs::builder()
///             .location("${example.location}")
///             .name("example-linux-function-app")
///             .resource_group_name("${example.name}")
///             .service_plan_id("${exampleServicePlan.id}")
///             .site_config(LinuxFunctionAppSiteConfig::builder().build_struct())
///             .storage_account_name("${exampleAccount.name}")
///             .build_struct(),
///     );
///     let exampleLinuxFunctionAppSlot = linux_function_app_slot::create(
///         "exampleLinuxFunctionAppSlot",
///         LinuxFunctionAppSlotArgs::builder()
///             .function_app_id("${exampleLinuxFunctionApp.id}")
///             .name("example-linux-function-app-slot")
///             .site_config(LinuxFunctionAppSlotSiteConfig::builder().build_struct())
///             .storage_account_name("${exampleAccount.name}")
///             .build_struct(),
///     );
///     let exampleServicePlan = service_plan::create(
///         "exampleServicePlan",
///         ServicePlanArgs::builder()
///             .location("${example.location}")
///             .name("example-app-service-plan")
///             .os_type("Linux")
///             .resource_group_name("${example.name}")
///             .sku_name("Y1")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// A Linux Function App Slot can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:appservice/linuxFunctionAppSlot:LinuxFunctionAppSlot example "/subscriptions/12345678-1234-9876-4563-123456789012/resourceGroups/resGroup1/providers/Microsoft.Web/sites/site1/slots/slot1"
/// ```
///
pub mod linux_function_app_slot {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LinuxFunctionAppSlotArgs {
        /// A map of key-value pairs for [App Settings](https://docs.microsoft.com/azure/azure-functions/functions-app-settings) and custom values.
        #[builder(into, default)]
        pub app_settings: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// an `auth_settings` block as detailed below.
        #[builder(into, default)]
        pub auth_settings: pulumi_wasm_rust::Output<
            Option<super::super::types::appservice::LinuxFunctionAppSlotAuthSettings>,
        >,
        /// an `auth_settings_v2` block as detailed below.
        #[builder(into, default)]
        pub auth_settings_v2: pulumi_wasm_rust::Output<
            Option<super::super::types::appservice::LinuxFunctionAppSlotAuthSettingsV2>,
        >,
        /// a `backup` block as detailed below.
        #[builder(into, default)]
        pub backup: pulumi_wasm_rust::Output<
            Option<super::super::types::appservice::LinuxFunctionAppSlotBackup>,
        >,
        /// Should built in logging be enabled. Configures `AzureWebJobsDashboard` app setting based on the configured storage setting. Defaults to `true`.
        #[builder(into, default)]
        pub builtin_logging_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Should the Function App Slot use Client Certificates.
        #[builder(into, default)]
        pub client_certificate_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Paths to exclude when using client certificates, separated by ;
        #[builder(into, default)]
        pub client_certificate_exclusion_paths: pulumi_wasm_rust::Output<Option<String>>,
        /// The mode of the Function App Slot's client certificates requirement for incoming requests. Possible values are `Required`, `Optional`, and `OptionalInteractiveUser`. Defaults to `Optional`.
        #[builder(into, default)]
        pub client_certificate_mode: pulumi_wasm_rust::Output<Option<String>>,
        /// a `connection_string` block as detailed below.
        #[builder(into, default)]
        pub connection_strings: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::types::appservice::LinuxFunctionAppSlotConnectionString,
                >,
            >,
        >,
        /// Force disable the content share settings.
        #[builder(into, default)]
        pub content_share_force_disabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The amount of memory in gigabyte-seconds that your application is allowed to consume per day. Setting this value only affects function apps in Consumption Plans. Defaults to `0`.
        #[builder(into, default)]
        pub daily_memory_time_quota: pulumi_wasm_rust::Output<Option<i32>>,
        /// Is the Linux Function App Slot enabled. Defaults to `true`.
        #[builder(into, default)]
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Are the default FTP Basic Authentication publishing credentials enabled. Defaults to `true`.
        #[builder(into, default)]
        pub ftp_publish_basic_authentication_enabled: pulumi_wasm_rust::Output<
            Option<bool>,
        >,
        /// The ID of the Linux Function App this Slot is a member of. Changing this forces a new resource to be created.
        #[builder(into)]
        pub function_app_id: pulumi_wasm_rust::Output<String>,
        /// The runtime version associated with the Function App Slot. Defaults to `~4`.
        #[builder(into, default)]
        pub functions_extension_version: pulumi_wasm_rust::Output<Option<String>>,
        /// Can the Function App Slot only be accessed via HTTPS?. Defaults to `false`.
        #[builder(into, default)]
        pub https_only: pulumi_wasm_rust::Output<Option<bool>>,
        /// An `identity` block as detailed below.
        #[builder(into, default)]
        pub identity: pulumi_wasm_rust::Output<
            Option<super::super::types::appservice::LinuxFunctionAppSlotIdentity>,
        >,
        /// The User Assigned Identity ID used for accessing KeyVault secrets. The identity must be assigned to the application in the `identity` block. [For more information see - Access vaults with a user-assigned identity](https://docs.microsoft.com/azure/app-service/app-service-key-vault-references#access-vaults-with-a-user-assigned-identity)
        #[builder(into, default)]
        pub key_vault_reference_identity_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name of the Function App Slot. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Should public network access be enabled for the Function App. Defaults to `true`.
        #[builder(into, default)]
        pub public_network_access_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The ID of the Service Plan in which to run this slot. If not specified the same Service Plan as the Linux Function App will be used.
        #[builder(into, default)]
        pub service_plan_id: pulumi_wasm_rust::Output<Option<String>>,
        /// a `site_config` block as detailed below.
        #[builder(into)]
        pub site_config: pulumi_wasm_rust::Output<
            super::super::types::appservice::LinuxFunctionAppSlotSiteConfig,
        >,
        /// The access key which will be used to access the storage account for the Function App Slot.
        #[builder(into, default)]
        pub storage_account_access_key: pulumi_wasm_rust::Output<Option<String>>,
        /// The backend storage account name which will be used by this Function App Slot.
        #[builder(into, default)]
        pub storage_account_name: pulumi_wasm_rust::Output<Option<String>>,
        /// One or more `storage_account` blocks as defined below.
        #[builder(into, default)]
        pub storage_accounts: pulumi_wasm_rust::Output<
            Option<
                Vec<super::super::types::appservice::LinuxFunctionAppSlotStorageAccount>,
            >,
        >,
        /// The Key Vault Secret ID, optionally including version, that contains the Connection String to connect to the storage account for this Function App.
        ///
        /// > **NOTE:** `storage_key_vault_secret_id` cannot be used with `storage_account_name`.
        ///
        /// > **NOTE:** `storage_key_vault_secret_id` used without a version will use the latest version of the secret, however, the service can take up to 24h to pick up a rotation of the latest version. See the [official docs](https://docs.microsoft.com/azure/app-service/app-service-key-vault-references#rotation) for more information.
        #[builder(into, default)]
        pub storage_key_vault_secret_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Should the Function App Slot use its Managed Identity to access storage.
        ///
        /// > **NOTE:** One of `storage_account_access_key` or `storage_uses_managed_identity` must be specified when using `storage_account_name`.
        #[builder(into, default)]
        pub storage_uses_managed_identity: pulumi_wasm_rust::Output<Option<bool>>,
        /// A mapping of tags which should be assigned to the Linux Function App.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        #[builder(into, default)]
        pub virtual_network_subnet_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Is container image pull over virtual network enabled? Defaults to `false`.
        #[builder(into, default)]
        pub vnet_image_pull_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Should the default WebDeploy Basic Authentication publishing credentials enabled. Defaults to `true`.
        #[builder(into, default)]
        pub webdeploy_publish_basic_authentication_enabled: pulumi_wasm_rust::Output<
            Option<bool>,
        >,
    }
    #[allow(dead_code)]
    pub struct LinuxFunctionAppSlotResult {
        /// A map of key-value pairs for [App Settings](https://docs.microsoft.com/azure/azure-functions/functions-app-settings) and custom values.
        pub app_settings: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// an `auth_settings` block as detailed below.
        pub auth_settings: pulumi_wasm_rust::Output<
            Option<super::super::types::appservice::LinuxFunctionAppSlotAuthSettings>,
        >,
        /// an `auth_settings_v2` block as detailed below.
        pub auth_settings_v2: pulumi_wasm_rust::Output<
            Option<super::super::types::appservice::LinuxFunctionAppSlotAuthSettingsV2>,
        >,
        /// a `backup` block as detailed below.
        pub backup: pulumi_wasm_rust::Output<
            Option<super::super::types::appservice::LinuxFunctionAppSlotBackup>,
        >,
        /// Should built in logging be enabled. Configures `AzureWebJobsDashboard` app setting based on the configured storage setting. Defaults to `true`.
        pub builtin_logging_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Should the Function App Slot use Client Certificates.
        pub client_certificate_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Paths to exclude when using client certificates, separated by ;
        pub client_certificate_exclusion_paths: pulumi_wasm_rust::Output<Option<String>>,
        /// The mode of the Function App Slot's client certificates requirement for incoming requests. Possible values are `Required`, `Optional`, and `OptionalInteractiveUser`. Defaults to `Optional`.
        pub client_certificate_mode: pulumi_wasm_rust::Output<Option<String>>,
        /// a `connection_string` block as detailed below.
        pub connection_strings: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::types::appservice::LinuxFunctionAppSlotConnectionString,
                >,
            >,
        >,
        /// Force disable the content share settings.
        pub content_share_force_disabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The identifier used by App Service to perform domain ownership verification via DNS TXT record.
        pub custom_domain_verification_id: pulumi_wasm_rust::Output<String>,
        /// The amount of memory in gigabyte-seconds that your application is allowed to consume per day. Setting this value only affects function apps in Consumption Plans. Defaults to `0`.
        pub daily_memory_time_quota: pulumi_wasm_rust::Output<Option<i32>>,
        /// The default hostname of the Linux Function App Slot.
        pub default_hostname: pulumi_wasm_rust::Output<String>,
        /// Is the Linux Function App Slot enabled. Defaults to `true`.
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Are the default FTP Basic Authentication publishing credentials enabled. Defaults to `true`.
        pub ftp_publish_basic_authentication_enabled: pulumi_wasm_rust::Output<
            Option<bool>,
        >,
        /// The ID of the Linux Function App this Slot is a member of. Changing this forces a new resource to be created.
        pub function_app_id: pulumi_wasm_rust::Output<String>,
        /// The runtime version associated with the Function App Slot. Defaults to `~4`.
        pub functions_extension_version: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the App Service Environment used by Function App Slot.
        pub hosting_environment_id: pulumi_wasm_rust::Output<String>,
        /// Can the Function App Slot only be accessed via HTTPS?. Defaults to `false`.
        pub https_only: pulumi_wasm_rust::Output<Option<bool>>,
        /// An `identity` block as detailed below.
        pub identity: pulumi_wasm_rust::Output<
            Option<super::super::types::appservice::LinuxFunctionAppSlotIdentity>,
        >,
        /// The User Assigned Identity ID used for accessing KeyVault secrets. The identity must be assigned to the application in the `identity` block. [For more information see - Access vaults with a user-assigned identity](https://docs.microsoft.com/azure/app-service/app-service-key-vault-references#access-vaults-with-a-user-assigned-identity)
        pub key_vault_reference_identity_id: pulumi_wasm_rust::Output<String>,
        /// The Kind value for this Linux Function App Slot.
        pub kind: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the Function App Slot. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A list of outbound IP addresses. For example `["52.23.25.3", "52.143.43.12"]`
        pub outbound_ip_address_lists: pulumi_wasm_rust::Output<Vec<String>>,
        /// A comma separated list of outbound IP addresses as a string. For example `52.23.25.3,52.143.43.12`.
        pub outbound_ip_addresses: pulumi_wasm_rust::Output<String>,
        /// A list of possible outbound IP addresses, not all of which are necessarily in use. This is a superset of `outbound_ip_address_list`. For example `["52.23.25.3", "52.143.43.12"]`.
        pub possible_outbound_ip_address_lists: pulumi_wasm_rust::Output<Vec<String>>,
        /// A comma separated list of possible outbound IP addresses as a string. For example `52.23.25.3,52.143.43.12,52.143.43.17`. This is a superset of `outbound_ip_addresses`. For example `["52.23.25.3", "52.143.43.12","52.143.43.17"]`.
        pub possible_outbound_ip_addresses: pulumi_wasm_rust::Output<String>,
        /// Should public network access be enabled for the Function App. Defaults to `true`.
        pub public_network_access_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The ID of the Service Plan in which to run this slot. If not specified the same Service Plan as the Linux Function App will be used.
        pub service_plan_id: pulumi_wasm_rust::Output<Option<String>>,
        /// a `site_config` block as detailed below.
        pub site_config: pulumi_wasm_rust::Output<
            super::super::types::appservice::LinuxFunctionAppSlotSiteConfig,
        >,
        /// A `site_credential` block as defined below.
        pub site_credentials: pulumi_wasm_rust::Output<
            Vec<super::super::types::appservice::LinuxFunctionAppSlotSiteCredential>,
        >,
        /// The access key which will be used to access the storage account for the Function App Slot.
        pub storage_account_access_key: pulumi_wasm_rust::Output<Option<String>>,
        /// The backend storage account name which will be used by this Function App Slot.
        pub storage_account_name: pulumi_wasm_rust::Output<Option<String>>,
        /// One or more `storage_account` blocks as defined below.
        pub storage_accounts: pulumi_wasm_rust::Output<
            Option<
                Vec<super::super::types::appservice::LinuxFunctionAppSlotStorageAccount>,
            >,
        >,
        /// The Key Vault Secret ID, optionally including version, that contains the Connection String to connect to the storage account for this Function App.
        ///
        /// > **NOTE:** `storage_key_vault_secret_id` cannot be used with `storage_account_name`.
        ///
        /// > **NOTE:** `storage_key_vault_secret_id` used without a version will use the latest version of the secret, however, the service can take up to 24h to pick up a rotation of the latest version. See the [official docs](https://docs.microsoft.com/azure/app-service/app-service-key-vault-references#rotation) for more information.
        pub storage_key_vault_secret_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Should the Function App Slot use its Managed Identity to access storage.
        ///
        /// > **NOTE:** One of `storage_account_access_key` or `storage_uses_managed_identity` must be specified when using `storage_account_name`.
        pub storage_uses_managed_identity: pulumi_wasm_rust::Output<Option<bool>>,
        /// A mapping of tags which should be assigned to the Linux Function App.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub virtual_network_subnet_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Is container image pull over virtual network enabled? Defaults to `false`.
        pub vnet_image_pull_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Should the default WebDeploy Basic Authentication publishing credentials enabled. Defaults to `true`.
        pub webdeploy_publish_basic_authentication_enabled: pulumi_wasm_rust::Output<
            Option<bool>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: LinuxFunctionAppSlotArgs,
    ) -> LinuxFunctionAppSlotResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let app_settings_binding = args.app_settings.get_inner();
        let auth_settings_binding = args.auth_settings.get_inner();
        let auth_settings_v2_binding = args.auth_settings_v2.get_inner();
        let backup_binding = args.backup.get_inner();
        let builtin_logging_enabled_binding = args.builtin_logging_enabled.get_inner();
        let client_certificate_enabled_binding = args
            .client_certificate_enabled
            .get_inner();
        let client_certificate_exclusion_paths_binding = args
            .client_certificate_exclusion_paths
            .get_inner();
        let client_certificate_mode_binding = args.client_certificate_mode.get_inner();
        let connection_strings_binding = args.connection_strings.get_inner();
        let content_share_force_disabled_binding = args
            .content_share_force_disabled
            .get_inner();
        let daily_memory_time_quota_binding = args.daily_memory_time_quota.get_inner();
        let enabled_binding = args.enabled.get_inner();
        let ftp_publish_basic_authentication_enabled_binding = args
            .ftp_publish_basic_authentication_enabled
            .get_inner();
        let function_app_id_binding = args.function_app_id.get_inner();
        let functions_extension_version_binding = args
            .functions_extension_version
            .get_inner();
        let https_only_binding = args.https_only.get_inner();
        let identity_binding = args.identity.get_inner();
        let key_vault_reference_identity_id_binding = args
            .key_vault_reference_identity_id
            .get_inner();
        let name_binding = args.name.get_inner();
        let public_network_access_enabled_binding = args
            .public_network_access_enabled
            .get_inner();
        let service_plan_id_binding = args.service_plan_id.get_inner();
        let site_config_binding = args.site_config.get_inner();
        let storage_account_access_key_binding = args
            .storage_account_access_key
            .get_inner();
        let storage_account_name_binding = args.storage_account_name.get_inner();
        let storage_accounts_binding = args.storage_accounts.get_inner();
        let storage_key_vault_secret_id_binding = args
            .storage_key_vault_secret_id
            .get_inner();
        let storage_uses_managed_identity_binding = args
            .storage_uses_managed_identity
            .get_inner();
        let tags_binding = args.tags.get_inner();
        let virtual_network_subnet_id_binding = args
            .virtual_network_subnet_id
            .get_inner();
        let vnet_image_pull_enabled_binding = args.vnet_image_pull_enabled.get_inner();
        let webdeploy_publish_basic_authentication_enabled_binding = args
            .webdeploy_publish_basic_authentication_enabled
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:appservice/linuxFunctionAppSlot:LinuxFunctionAppSlot".into(),
            name: name.to_string(),
            object: Vec::from([
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
                    name: "builtinLoggingEnabled".into(),
                    value: &builtin_logging_enabled_binding,
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
                    name: "contentShareForceDisabled".into(),
                    value: &content_share_force_disabled_binding,
                },
                register_interface::ObjectField {
                    name: "dailyMemoryTimeQuota".into(),
                    value: &daily_memory_time_quota_binding,
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
                    name: "functionAppId".into(),
                    value: &function_app_id_binding,
                },
                register_interface::ObjectField {
                    name: "functionsExtensionVersion".into(),
                    value: &functions_extension_version_binding,
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
                    name: "storageAccountAccessKey".into(),
                    value: &storage_account_access_key_binding,
                },
                register_interface::ObjectField {
                    name: "storageAccountName".into(),
                    value: &storage_account_name_binding,
                },
                register_interface::ObjectField {
                    name: "storageAccounts".into(),
                    value: &storage_accounts_binding,
                },
                register_interface::ObjectField {
                    name: "storageKeyVaultSecretId".into(),
                    value: &storage_key_vault_secret_id_binding,
                },
                register_interface::ObjectField {
                    name: "storageUsesManagedIdentity".into(),
                    value: &storage_uses_managed_identity_binding,
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
                    name: "vnetImagePullEnabled".into(),
                    value: &vnet_image_pull_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "webdeployPublishBasicAuthenticationEnabled".into(),
                    value: &webdeploy_publish_basic_authentication_enabled_binding,
                },
            ]),
            results: Vec::from([
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
                    name: "builtinLoggingEnabled".into(),
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
                    name: "contentShareForceDisabled".into(),
                },
                register_interface::ResultField {
                    name: "customDomainVerificationId".into(),
                },
                register_interface::ResultField {
                    name: "dailyMemoryTimeQuota".into(),
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
                    name: "functionAppId".into(),
                },
                register_interface::ResultField {
                    name: "functionsExtensionVersion".into(),
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
                    name: "storageAccountAccessKey".into(),
                },
                register_interface::ResultField {
                    name: "storageAccountName".into(),
                },
                register_interface::ResultField {
                    name: "storageAccounts".into(),
                },
                register_interface::ResultField {
                    name: "storageKeyVaultSecretId".into(),
                },
                register_interface::ResultField {
                    name: "storageUsesManagedIdentity".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "virtualNetworkSubnetId".into(),
                },
                register_interface::ResultField {
                    name: "vnetImagePullEnabled".into(),
                },
                register_interface::ResultField {
                    name: "webdeployPublishBasicAuthenticationEnabled".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        LinuxFunctionAppSlotResult {
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
            builtin_logging_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("builtinLoggingEnabled").unwrap(),
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
            content_share_force_disabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("contentShareForceDisabled").unwrap(),
            ),
            custom_domain_verification_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customDomainVerificationId").unwrap(),
            ),
            daily_memory_time_quota: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dailyMemoryTimeQuota").unwrap(),
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
            function_app_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("functionAppId").unwrap(),
            ),
            functions_extension_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("functionsExtensionVersion").unwrap(),
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
            storage_account_access_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageAccountAccessKey").unwrap(),
            ),
            storage_account_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageAccountName").unwrap(),
            ),
            storage_accounts: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageAccounts").unwrap(),
            ),
            storage_key_vault_secret_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageKeyVaultSecretId").unwrap(),
            ),
            storage_uses_managed_identity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageUsesManagedIdentity").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            virtual_network_subnet_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("virtualNetworkSubnetId").unwrap(),
            ),
            vnet_image_pull_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vnetImagePullEnabled").unwrap(),
            ),
            webdeploy_publish_basic_authentication_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("webdeployPublishBasicAuthenticationEnabled").unwrap(),
            ),
        }
    }
}