pub mod get_linux_function_app {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetLinuxFunctionAppArgs {
        /// The name which should be used for this Linux Function App.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the Resource Group where the Linux Function App should exist.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetLinuxFunctionAppResult {
        /// A map of key-value pairs for [App Settings](https://docs.microsoft.com/azure/azure-functions/functions-app-settings) and custom values.
        pub app_settings: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// A `auth_settings` block as defined below.
        pub auth_settings: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::appservice::GetLinuxFunctionAppAuthSetting>,
        >,
        /// A `auth_settings_v2` block as defined below.
        pub auth_settings_v2s: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::appservice::GetLinuxFunctionAppAuthSettingsV2,
            >,
        >,
        /// The current availability state. Possible values are `Normal`, `Limited`, and `DisasterRecoveryMode`.
        pub availability: pulumi_wasm_rust::Output<String>,
        /// A `backup` block as defined below.
        pub backups: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::appservice::GetLinuxFunctionAppBackup>,
        >,
        /// Is built in logging enabled?
        pub builtin_logging_enabled: pulumi_wasm_rust::Output<bool>,
        /// Are Client Certificates enabled?
        pub client_certificate_enabled: pulumi_wasm_rust::Output<bool>,
        /// Paths to exclude when using client certificates, separated by ;
        pub client_certificate_exclusion_paths: pulumi_wasm_rust::Output<String>,
        /// The mode of the Function App's client certificates requirement for incoming requests.
        pub client_certificate_mode: pulumi_wasm_rust::Output<String>,
        /// A `connection_string` blocks as defined below.
        pub connection_strings: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::appservice::GetLinuxFunctionAppConnectionString,
            >,
        >,
        /// Are the settings for linking the Function App to storage suppressed?
        pub content_share_force_disabled: pulumi_wasm_rust::Output<bool>,
        /// The identifier used by App Service to perform domain ownership verification via DNS TXT record.
        pub custom_domain_verification_id: pulumi_wasm_rust::Output<String>,
        /// The amount of memory in gigabyte-seconds that your application is allowed to consume per day.
        pub daily_memory_time_quota: pulumi_wasm_rust::Output<i32>,
        /// The default hostname of the Linux Function App.
        pub default_hostname: pulumi_wasm_rust::Output<String>,
        /// Is this backup job enabled?
        pub enabled: pulumi_wasm_rust::Output<bool>,
        /// Are the default FTP Basic Authentication publishing credentials enabled.
        pub ftp_publish_basic_authentication_enabled: pulumi_wasm_rust::Output<bool>,
        /// The runtime version associated with the Function App.
        pub functions_extension_version: pulumi_wasm_rust::Output<String>,
        /// The ID of the App Service Environment used by Function App.
        pub hosting_environment_id: pulumi_wasm_rust::Output<String>,
        /// Can the Function App only be accessed via HTTPS?
        pub https_only: pulumi_wasm_rust::Output<bool>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// A `identity` block as defined below.
        pub identities: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::appservice::GetLinuxFunctionAppIdentity>,
        >,
        /// The Kind value for this Linux Function App.
        pub kind: pulumi_wasm_rust::Output<String>,
        /// The Azure Region where the Linux Function App exists.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The Site Credentials Username used for publishing.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A list of outbound IP addresses. For example `["52.23.25.3", "52.143.43.12"]`
        pub outbound_ip_address_lists: pulumi_wasm_rust::Output<Vec<String>>,
        /// A comma separated list of outbound IP addresses as a string. For example `52.23.25.3,52.143.43.12`.
        pub outbound_ip_addresses: pulumi_wasm_rust::Output<String>,
        /// A list of possible outbound IP addresses, not all of which are necessarily in use. This is a superset of `outbound_ip_address_list`. For example `["52.23.25.3", "52.143.43.12"]`.
        pub possible_outbound_ip_address_lists: pulumi_wasm_rust::Output<Vec<String>>,
        /// A comma separated list of possible outbound IP addresses as a string. For example `52.23.25.3,52.143.43.12,52.143.43.17`. This is a superset of `outbound_ip_addresses`.
        pub possible_outbound_ip_addresses: pulumi_wasm_rust::Output<String>,
        /// Is Public Network Access enabled for this Linux Function App.
        pub public_network_access_enabled: pulumi_wasm_rust::Output<bool>,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The ID of the App Service Plan within which this Function App has been created.
        pub service_plan_id: pulumi_wasm_rust::Output<String>,
        /// A `site_config` block as defined below.
        pub site_configs: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::appservice::GetLinuxFunctionAppSiteConfig>,
        >,
        /// A `site_credential` block as defined below.
        pub site_credentials: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::appservice::GetLinuxFunctionAppSiteCredential,
            >,
        >,
        /// A `sticky_settings` block as defined below.
        pub sticky_settings: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::appservice::GetLinuxFunctionAppStickySetting>,
        >,
        /// The access key used to access the backend storage account for the Function App.
        pub storage_account_access_key: pulumi_wasm_rust::Output<String>,
        /// The backend storage account name used by this Function App.
        pub storage_account_name: pulumi_wasm_rust::Output<String>,
        /// The Key Vault Secret ID, including version, that contains the Connection String to connect to the storage account for this Function App.
        pub storage_key_vault_secret_id: pulumi_wasm_rust::Output<String>,
        /// Does the Function App use Managed Identity to access the storage account?
        pub storage_uses_managed_identity: pulumi_wasm_rust::Output<bool>,
        /// A mapping of tags which are assigned to the Linux Function App.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// The current usage state. Possible values are `Normal` and `Exceeded`.
        pub usage: pulumi_wasm_rust::Output<String>,
        /// The Virtual Network Subnet ID used for this IP Restriction.
        pub virtual_network_subnet_id: pulumi_wasm_rust::Output<String>,
        /// Are the default WebDeploy Basic Authentication publishing credentials enabled.
        pub webdeploy_publish_basic_authentication_enabled: pulumi_wasm_rust::Output<
            bool,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetLinuxFunctionAppArgs) -> GetLinuxFunctionAppResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:appservice/getLinuxFunctionApp:getLinuxFunctionApp".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
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
                    name: "authSettingsV2s".into(),
                },
                register_interface::ResultField {
                    name: "availability".into(),
                },
                register_interface::ResultField {
                    name: "backups".into(),
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
                    name: "functionsExtensionVersion".into(),
                },
                register_interface::ResultField {
                    name: "hostingEnvironmentId".into(),
                },
                register_interface::ResultField {
                    name: "httpsOnly".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "identities".into(),
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
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "servicePlanId".into(),
                },
                register_interface::ResultField {
                    name: "siteConfigs".into(),
                },
                register_interface::ResultField {
                    name: "siteCredentials".into(),
                },
                register_interface::ResultField {
                    name: "stickySettings".into(),
                },
                register_interface::ResultField {
                    name: "storageAccountAccessKey".into(),
                },
                register_interface::ResultField {
                    name: "storageAccountName".into(),
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
                    name: "usage".into(),
                },
                register_interface::ResultField {
                    name: "virtualNetworkSubnetId".into(),
                },
                register_interface::ResultField {
                    name: "webdeployPublishBasicAuthenticationEnabled".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetLinuxFunctionAppResult {
            app_settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("appSettings").unwrap(),
            ),
            auth_settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("authSettings").unwrap(),
            ),
            auth_settings_v2s: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("authSettingsV2s").unwrap(),
            ),
            availability: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("availability").unwrap(),
            ),
            backups: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("backups").unwrap(),
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
            functions_extension_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("functionsExtensionVersion").unwrap(),
            ),
            hosting_environment_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hostingEnvironmentId").unwrap(),
            ),
            https_only: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("httpsOnly").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            identities: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("identities").unwrap(),
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
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            service_plan_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("servicePlanId").unwrap(),
            ),
            site_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("siteConfigs").unwrap(),
            ),
            site_credentials: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("siteCredentials").unwrap(),
            ),
            sticky_settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("stickySettings").unwrap(),
            ),
            storage_account_access_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageAccountAccessKey").unwrap(),
            ),
            storage_account_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageAccountName").unwrap(),
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
            usage: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("usage").unwrap(),
            ),
            virtual_network_subnet_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("virtualNetworkSubnetId").unwrap(),
            ),
            webdeploy_publish_basic_authentication_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("webdeployPublishBasicAuthenticationEnabled").unwrap(),
            ),
        }
    }
}
