/// Manages a Windows Function App Slot.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
///             .name("windowsfunctionappsa")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleServicePlan = service_plan::create(
///         "exampleServicePlan",
///         ServicePlanArgs::builder()
///             .location("${example.location}")
///             .name("example-app-service-plan")
///             .os_type("Windows")
///             .resource_group_name("${example.name}")
///             .sku_name("Y1")
///             .build_struct(),
///     );
///     let exampleWindowsFunctionApp = windows_function_app::create(
///         "exampleWindowsFunctionApp",
///         WindowsFunctionAppArgs::builder()
///             .location("${example.location}")
///             .name("example-windows-function-app")
///             .resource_group_name("${example.name}")
///             .service_plan_id("${exampleServicePlan.id}")
///             .site_config(WindowsFunctionAppSiteConfig::builder().build_struct())
///             .storage_account_name("${exampleAccount.name}")
///             .build_struct(),
///     );
///     let exampleWindowsFunctionAppSlot = windows_function_app_slot::create(
///         "exampleWindowsFunctionAppSlot",
///         WindowsFunctionAppSlotArgs::builder()
///             .function_app_id("${exampleWindowsFunctionApp.id}")
///             .name("example-slot")
///             .site_config(WindowsFunctionAppSlotSiteConfig::builder().build_struct())
///             .storage_account_name("${exampleAccount.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// A Windows Function App Slot can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:appservice/windowsFunctionAppSlot:WindowsFunctionAppSlot example "/subscriptions/12345678-1234-9876-4563-123456789012/resourceGroups/resGroup1/providers/Microsoft.Web/sites/site1/slots/slot1"
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod windows_function_app_slot {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WindowsFunctionAppSlotArgs {
        /// A map of key-value pairs for [App Settings](https://docs.microsoft.com/azure/azure-functions/functions-app-settings) and custom values.
        #[builder(into, default)]
        pub app_settings: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// an `auth_settings` block as detailed below.
        #[builder(into, default)]
        pub auth_settings: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::appservice::WindowsFunctionAppSlotAuthSettings>,
        >,
        /// an `auth_settings_v2` block as detailed below.
        #[builder(into, default)]
        pub auth_settings_v2: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::appservice::WindowsFunctionAppSlotAuthSettingsV2>,
        >,
        /// a `backup` block as detailed below.
        #[builder(into, default)]
        pub backup: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::appservice::WindowsFunctionAppSlotBackup>,
        >,
        /// Should built-in logging be enabled. Configures `AzureWebJobsDashboard` app setting based on the configured storage setting. Defaults to `true`.
        #[builder(into, default)]
        pub builtin_logging_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Should the Function App Slot use Client Certificates.
        #[builder(into, default)]
        pub client_certificate_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Paths to exclude when using client certificates, separated by ;
        #[builder(into, default)]
        pub client_certificate_exclusion_paths: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The mode of the Function App Slot's client certificates requirement for incoming requests. Possible values are `Required`, `Optional`, and `OptionalInteractiveUser`. Defaults to `Optional`.
        #[builder(into, default)]
        pub client_certificate_mode: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// a `connection_string` block as detailed below.
        #[builder(into, default)]
        pub connection_strings: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::types::appservice::WindowsFunctionAppSlotConnectionString,
                >,
            >,
        >,
        /// Force disable the content share settings.
        #[builder(into, default)]
        pub content_share_force_disabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// The amount of memory in gigabyte-seconds that your application is allowed to consume per day. Setting this value only affects function apps in Consumption Plans. Defaults to `0`.
        #[builder(into, default)]
        pub daily_memory_time_quota: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Is the Windows Function App Slot enabled. Defaults to `true`.
        #[builder(into, default)]
        pub enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Should the default FTP Basic Authentication publishing profile be enabled. Defaults to `true`.
        #[builder(into, default)]
        pub ftp_publish_basic_authentication_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// The name of the Windows Function App this Slot is a member of. Changing this forces a new resource to be created.
        #[builder(into)]
        pub function_app_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The runtime version associated with the Function App Slot. Defaults to `~4`.
        #[builder(into, default)]
        pub functions_extension_version: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Can the Function App Slot only be accessed via HTTPS?. Defaults to `false`.
        #[builder(into, default)]
        pub https_only: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// an `identity` block as detailed below.
        #[builder(into, default)]
        pub identity: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::appservice::WindowsFunctionAppSlotIdentity>,
        >,
        /// The User Assigned Identity ID used for accessing KeyVault secrets. The identity must be assigned to the application in the `identity` block. [For more information see - Access vaults with a user-assigned identity](https://docs.microsoft.com/azure/app-service/app-service-key-vault-references#access-vaults-with-a-user-assigned-identity)
        #[builder(into, default)]
        pub key_vault_reference_identity_id: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Specifies the name of the Windows Function App Slot. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Should public network access be enabled for the Function App. Defaults to `true`.
        #[builder(into, default)]
        pub public_network_access_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// The ID of the Service Plan in which to run this slot. If not specified the same Service Plan as the Windows Function App will be used.
        #[builder(into, default)]
        pub service_plan_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// a `site_config` block as detailed below.
        #[builder(into)]
        pub site_config: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::appservice::WindowsFunctionAppSlotSiteConfig,
        >,
        /// The access key which will be used to access the storage account for the Function App Slot.
        #[builder(into, default)]
        pub storage_account_access_key: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The backend storage account name which will be used by this Function App Slot.
        #[builder(into, default)]
        pub storage_account_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// One or more `storage_account` blocks as defined below.
        #[builder(into, default)]
        pub storage_accounts: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::types::appservice::WindowsFunctionAppSlotStorageAccount,
                >,
            >,
        >,
        /// The Key Vault Secret ID, optionally including version, that contains the Connection String to connect to the storage account for this Function App Slot.
        ///
        /// > **NOTE:** `storage_key_vault_secret_id` cannot be used with `storage_account_name`.
        ///
        /// > **NOTE:** `storage_key_vault_secret_id` used without a version will use the latest version of the secret, however, the service can take up to 24h to pick up a rotation of the latest version. See the [official docs](https://docs.microsoft.com/azure/app-service/app-service-key-vault-references#rotation) for more information.
        #[builder(into, default)]
        pub storage_key_vault_secret_id: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Should the Function App Slot use its Managed Identity to access storage.
        ///
        /// > **NOTE:** One of `storage_account_access_key` or `storage_uses_managed_identity` must be specified when using `storage_account_name`.
        #[builder(into, default)]
        pub storage_uses_managed_identity: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// A mapping of tags which should be assigned to the Windows Function App Slot.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        #[builder(into, default)]
        pub virtual_network_subnet_id: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Is container image pull over virtual network enabled? Defaults to `false`.
        #[builder(into, default)]
        pub vnet_image_pull_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Should the default WebDeploy Basic Authentication publishing credentials enabled. Defaults to `true`.
        #[builder(into, default)]
        pub webdeploy_publish_basic_authentication_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
    }
    #[allow(dead_code)]
    pub struct WindowsFunctionAppSlotResult {
        /// A map of key-value pairs for [App Settings](https://docs.microsoft.com/azure/azure-functions/functions-app-settings) and custom values.
        pub app_settings: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// an `auth_settings` block as detailed below.
        pub auth_settings: pulumi_gestalt_rust::Output<
            Option<super::super::types::appservice::WindowsFunctionAppSlotAuthSettings>,
        >,
        /// an `auth_settings_v2` block as detailed below.
        pub auth_settings_v2: pulumi_gestalt_rust::Output<
            Option<super::super::types::appservice::WindowsFunctionAppSlotAuthSettingsV2>,
        >,
        /// a `backup` block as detailed below.
        pub backup: pulumi_gestalt_rust::Output<
            Option<super::super::types::appservice::WindowsFunctionAppSlotBackup>,
        >,
        /// Should built-in logging be enabled. Configures `AzureWebJobsDashboard` app setting based on the configured storage setting. Defaults to `true`.
        pub builtin_logging_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Should the Function App Slot use Client Certificates.
        pub client_certificate_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Paths to exclude when using client certificates, separated by ;
        pub client_certificate_exclusion_paths: pulumi_gestalt_rust::Output<
            Option<String>,
        >,
        /// The mode of the Function App Slot's client certificates requirement for incoming requests. Possible values are `Required`, `Optional`, and `OptionalInteractiveUser`. Defaults to `Optional`.
        pub client_certificate_mode: pulumi_gestalt_rust::Output<Option<String>>,
        /// a `connection_string` block as detailed below.
        pub connection_strings: pulumi_gestalt_rust::Output<
            Option<
                Vec<
                    super::super::types::appservice::WindowsFunctionAppSlotConnectionString,
                >,
            >,
        >,
        /// Force disable the content share settings.
        pub content_share_force_disabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The identifier used by App Service to perform domain ownership verification via DNS TXT record.
        pub custom_domain_verification_id: pulumi_gestalt_rust::Output<String>,
        /// The amount of memory in gigabyte-seconds that your application is allowed to consume per day. Setting this value only affects function apps in Consumption Plans. Defaults to `0`.
        pub daily_memory_time_quota: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The default hostname of the Windows Function App Slot.
        pub default_hostname: pulumi_gestalt_rust::Output<String>,
        /// Is the Windows Function App Slot enabled. Defaults to `true`.
        pub enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Should the default FTP Basic Authentication publishing profile be enabled. Defaults to `true`.
        pub ftp_publish_basic_authentication_enabled: pulumi_gestalt_rust::Output<
            Option<bool>,
        >,
        /// The name of the Windows Function App this Slot is a member of. Changing this forces a new resource to be created.
        pub function_app_id: pulumi_gestalt_rust::Output<String>,
        /// The runtime version associated with the Function App Slot. Defaults to `~4`.
        pub functions_extension_version: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of the App Service Environment used by Function App Slot.
        pub hosting_environment_id: pulumi_gestalt_rust::Output<String>,
        /// Can the Function App Slot only be accessed via HTTPS?. Defaults to `false`.
        pub https_only: pulumi_gestalt_rust::Output<Option<bool>>,
        /// an `identity` block as detailed below.
        pub identity: pulumi_gestalt_rust::Output<
            Option<super::super::types::appservice::WindowsFunctionAppSlotIdentity>,
        >,
        /// The User Assigned Identity ID used for accessing KeyVault secrets. The identity must be assigned to the application in the `identity` block. [For more information see - Access vaults with a user-assigned identity](https://docs.microsoft.com/azure/app-service/app-service-key-vault-references#access-vaults-with-a-user-assigned-identity)
        pub key_vault_reference_identity_id: pulumi_gestalt_rust::Output<String>,
        /// The Kind value for this Windows Function App Slot.
        pub kind: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the Windows Function App Slot. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A list of outbound IP addresses. For example `["52.23.25.3", "52.143.43.12"]`.
        pub outbound_ip_address_lists: pulumi_gestalt_rust::Output<Vec<String>>,
        /// A comma separated list of outbound IP addresses as a string. For example `52.23.25.3,52.143.43.12`.
        pub outbound_ip_addresses: pulumi_gestalt_rust::Output<String>,
        /// A list of possible outbound IP addresses, not all of which are necessarily in use. This is a superset of `outbound_ip_address_list`. For example `["52.23.25.3", "52.143.43.12"]`.
        pub possible_outbound_ip_address_lists: pulumi_gestalt_rust::Output<Vec<String>>,
        /// A comma separated list of possible outbound IP addresses as a string. For example `52.23.25.3,52.143.43.12,52.143.43.17`. This is a superset of `outbound_ip_addresses`. For example `["52.23.25.3", "52.143.43.12","52.143.43.17"]`.
        pub possible_outbound_ip_addresses: pulumi_gestalt_rust::Output<String>,
        /// Should public network access be enabled for the Function App. Defaults to `true`.
        pub public_network_access_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The ID of the Service Plan in which to run this slot. If not specified the same Service Plan as the Windows Function App will be used.
        pub service_plan_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// a `site_config` block as detailed below.
        pub site_config: pulumi_gestalt_rust::Output<
            super::super::types::appservice::WindowsFunctionAppSlotSiteConfig,
        >,
        /// A `site_credential` block as defined below.
        pub site_credentials: pulumi_gestalt_rust::Output<
            Vec<super::super::types::appservice::WindowsFunctionAppSlotSiteCredential>,
        >,
        /// The access key which will be used to access the storage account for the Function App Slot.
        pub storage_account_access_key: pulumi_gestalt_rust::Output<Option<String>>,
        /// The backend storage account name which will be used by this Function App Slot.
        pub storage_account_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// One or more `storage_account` blocks as defined below.
        pub storage_accounts: pulumi_gestalt_rust::Output<
            Option<
                Vec<
                    super::super::types::appservice::WindowsFunctionAppSlotStorageAccount,
                >,
            >,
        >,
        /// The Key Vault Secret ID, optionally including version, that contains the Connection String to connect to the storage account for this Function App Slot.
        ///
        /// > **NOTE:** `storage_key_vault_secret_id` cannot be used with `storage_account_name`.
        ///
        /// > **NOTE:** `storage_key_vault_secret_id` used without a version will use the latest version of the secret, however, the service can take up to 24h to pick up a rotation of the latest version. See the [official docs](https://docs.microsoft.com/azure/app-service/app-service-key-vault-references#rotation) for more information.
        pub storage_key_vault_secret_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// Should the Function App Slot use its Managed Identity to access storage.
        ///
        /// > **NOTE:** One of `storage_account_access_key` or `storage_uses_managed_identity` must be specified when using `storage_account_name`.
        pub storage_uses_managed_identity: pulumi_gestalt_rust::Output<Option<bool>>,
        /// A mapping of tags which should be assigned to the Windows Function App Slot.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub virtual_network_subnet_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// Is container image pull over virtual network enabled? Defaults to `false`.
        pub vnet_image_pull_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Should the default WebDeploy Basic Authentication publishing credentials enabled. Defaults to `true`.
        pub webdeploy_publish_basic_authentication_enabled: pulumi_gestalt_rust::Output<
            Option<bool>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: WindowsFunctionAppSlotArgs,
    ) -> WindowsFunctionAppSlotResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let app_settings_binding = args.app_settings.get_output(context);
        let auth_settings_binding = args.auth_settings.get_output(context);
        let auth_settings_v2_binding = args.auth_settings_v2.get_output(context);
        let backup_binding = args.backup.get_output(context);
        let builtin_logging_enabled_binding = args
            .builtin_logging_enabled
            .get_output(context);
        let client_certificate_enabled_binding = args
            .client_certificate_enabled
            .get_output(context);
        let client_certificate_exclusion_paths_binding = args
            .client_certificate_exclusion_paths
            .get_output(context);
        let client_certificate_mode_binding = args
            .client_certificate_mode
            .get_output(context);
        let connection_strings_binding = args.connection_strings.get_output(context);
        let content_share_force_disabled_binding = args
            .content_share_force_disabled
            .get_output(context);
        let daily_memory_time_quota_binding = args
            .daily_memory_time_quota
            .get_output(context);
        let enabled_binding = args.enabled.get_output(context);
        let ftp_publish_basic_authentication_enabled_binding = args
            .ftp_publish_basic_authentication_enabled
            .get_output(context);
        let function_app_id_binding = args.function_app_id.get_output(context);
        let functions_extension_version_binding = args
            .functions_extension_version
            .get_output(context);
        let https_only_binding = args.https_only.get_output(context);
        let identity_binding = args.identity.get_output(context);
        let key_vault_reference_identity_id_binding = args
            .key_vault_reference_identity_id
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let public_network_access_enabled_binding = args
            .public_network_access_enabled
            .get_output(context);
        let service_plan_id_binding = args.service_plan_id.get_output(context);
        let site_config_binding = args.site_config.get_output(context);
        let storage_account_access_key_binding = args
            .storage_account_access_key
            .get_output(context);
        let storage_account_name_binding = args.storage_account_name.get_output(context);
        let storage_accounts_binding = args.storage_accounts.get_output(context);
        let storage_key_vault_secret_id_binding = args
            .storage_key_vault_secret_id
            .get_output(context);
        let storage_uses_managed_identity_binding = args
            .storage_uses_managed_identity
            .get_output(context);
        let tags_binding = args.tags.get_output(context);
        let virtual_network_subnet_id_binding = args
            .virtual_network_subnet_id
            .get_output(context);
        let vnet_image_pull_enabled_binding = args
            .vnet_image_pull_enabled
            .get_output(context);
        let webdeploy_publish_basic_authentication_enabled_binding = args
            .webdeploy_publish_basic_authentication_enabled
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:appservice/windowsFunctionAppSlot:WindowsFunctionAppSlot"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "appSettings".into(),
                    value: &app_settings_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "authSettings".into(),
                    value: &auth_settings_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "authSettingsV2".into(),
                    value: &auth_settings_v2_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "backup".into(),
                    value: &backup_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "builtinLoggingEnabled".into(),
                    value: &builtin_logging_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clientCertificateEnabled".into(),
                    value: &client_certificate_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clientCertificateExclusionPaths".into(),
                    value: &client_certificate_exclusion_paths_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clientCertificateMode".into(),
                    value: &client_certificate_mode_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "connectionStrings".into(),
                    value: &connection_strings_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "contentShareForceDisabled".into(),
                    value: &content_share_force_disabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dailyMemoryTimeQuota".into(),
                    value: &daily_memory_time_quota_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ftpPublishBasicAuthenticationEnabled".into(),
                    value: &ftp_publish_basic_authentication_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "functionAppId".into(),
                    value: &function_app_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "functionsExtensionVersion".into(),
                    value: &functions_extension_version_binding.drop_type(),
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
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "publicNetworkAccessEnabled".into(),
                    value: &public_network_access_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "servicePlanId".into(),
                    value: &service_plan_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "siteConfig".into(),
                    value: &site_config_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storageAccountAccessKey".into(),
                    value: &storage_account_access_key_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storageAccountName".into(),
                    value: &storage_account_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storageAccounts".into(),
                    value: &storage_accounts_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storageKeyVaultSecretId".into(),
                    value: &storage_key_vault_secret_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storageUsesManagedIdentity".into(),
                    value: &storage_uses_managed_identity_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "virtualNetworkSubnetId".into(),
                    value: &virtual_network_subnet_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vnetImagePullEnabled".into(),
                    value: &vnet_image_pull_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "webdeployPublishBasicAuthenticationEnabled".into(),
                    value: &webdeploy_publish_basic_authentication_enabled_binding
                        .drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        WindowsFunctionAppSlotResult {
            app_settings: o.get_field("appSettings"),
            auth_settings: o.get_field("authSettings"),
            auth_settings_v2: o.get_field("authSettingsV2"),
            backup: o.get_field("backup"),
            builtin_logging_enabled: o.get_field("builtinLoggingEnabled"),
            client_certificate_enabled: o.get_field("clientCertificateEnabled"),
            client_certificate_exclusion_paths: o
                .get_field("clientCertificateExclusionPaths"),
            client_certificate_mode: o.get_field("clientCertificateMode"),
            connection_strings: o.get_field("connectionStrings"),
            content_share_force_disabled: o.get_field("contentShareForceDisabled"),
            custom_domain_verification_id: o.get_field("customDomainVerificationId"),
            daily_memory_time_quota: o.get_field("dailyMemoryTimeQuota"),
            default_hostname: o.get_field("defaultHostname"),
            enabled: o.get_field("enabled"),
            ftp_publish_basic_authentication_enabled: o
                .get_field("ftpPublishBasicAuthenticationEnabled"),
            function_app_id: o.get_field("functionAppId"),
            functions_extension_version: o.get_field("functionsExtensionVersion"),
            hosting_environment_id: o.get_field("hostingEnvironmentId"),
            https_only: o.get_field("httpsOnly"),
            identity: o.get_field("identity"),
            key_vault_reference_identity_id: o.get_field("keyVaultReferenceIdentityId"),
            kind: o.get_field("kind"),
            name: o.get_field("name"),
            outbound_ip_address_lists: o.get_field("outboundIpAddressLists"),
            outbound_ip_addresses: o.get_field("outboundIpAddresses"),
            possible_outbound_ip_address_lists: o
                .get_field("possibleOutboundIpAddressLists"),
            possible_outbound_ip_addresses: o.get_field("possibleOutboundIpAddresses"),
            public_network_access_enabled: o.get_field("publicNetworkAccessEnabled"),
            service_plan_id: o.get_field("servicePlanId"),
            site_config: o.get_field("siteConfig"),
            site_credentials: o.get_field("siteCredentials"),
            storage_account_access_key: o.get_field("storageAccountAccessKey"),
            storage_account_name: o.get_field("storageAccountName"),
            storage_accounts: o.get_field("storageAccounts"),
            storage_key_vault_secret_id: o.get_field("storageKeyVaultSecretId"),
            storage_uses_managed_identity: o.get_field("storageUsesManagedIdentity"),
            tags: o.get_field("tags"),
            virtual_network_subnet_id: o.get_field("virtualNetworkSubnetId"),
            vnet_image_pull_enabled: o.get_field("vnetImagePullEnabled"),
            webdeploy_publish_basic_authentication_enabled: o
                .get_field("webdeployPublishBasicAuthenticationEnabled"),
        }
    }
}
