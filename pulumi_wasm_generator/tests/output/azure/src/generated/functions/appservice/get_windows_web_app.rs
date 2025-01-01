pub mod get_windows_web_app {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetWindowsWebAppArgs {
        /// The name of this Windows Web App.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the Resource Group where the Windows Web App exists.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetWindowsWebAppResult {
        /// A map of key-value pairs of App Settings.
        pub app_settings: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// A `auth_settings` block as defined below.
        pub auth_settings: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::appservice::GetWindowsWebAppAuthSetting>,
        >,
        /// An `auth_settings_v2` block as defined below.
        pub auth_settings_v2s: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::appservice::GetWindowsWebAppAuthSettingsV2>,
        >,
        /// A `backup` block as defined below.
        pub backups: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::appservice::GetWindowsWebAppBackup>,
        >,
        /// Is Client Affinity enabled?
        pub client_affinity_enabled: pulumi_wasm_rust::Output<bool>,
        /// Are Client Certificates enabled?
        pub client_certificate_enabled: pulumi_wasm_rust::Output<bool>,
        /// Paths to exclude when using client certificates, separated by ;
        pub client_certificate_exclusion_paths: pulumi_wasm_rust::Output<String>,
        /// The Client Certificate mode.
        pub client_certificate_mode: pulumi_wasm_rust::Output<String>,
        /// A `connection_string` block as defined below.
        pub connection_strings: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::appservice::GetWindowsWebAppConnectionString>,
        >,
        /// The identifier used by App Service to perform domain ownership verification via DNS TXT record.
        pub custom_domain_verification_id: pulumi_wasm_rust::Output<String>,
        /// The Default Hostname of the Windows Web App.
        pub default_hostname: pulumi_wasm_rust::Output<String>,
        /// Is the Backup enabled?
        pub enabled: pulumi_wasm_rust::Output<bool>,
        /// Are the default FTP Basic Authentication publishing credentials enabled.
        pub ftp_publish_basic_authentication_enabled: pulumi_wasm_rust::Output<bool>,
        /// The ID of the App Service Environment used by App Service.
        pub hosting_environment_id: pulumi_wasm_rust::Output<String>,
        /// Does the Windows Web App require HTTPS connections.
        pub https_only: pulumi_wasm_rust::Output<bool>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// A `identity` block as defined below.
        pub identities: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::appservice::GetWindowsWebAppIdentity>,
        >,
        /// The string representation of the Windows Web App Kind.
        pub kind: pulumi_wasm_rust::Output<String>,
        /// The Azure Region where the Windows Web App exists.
        pub location: pulumi_wasm_rust::Output<String>,
        /// A `logs` block as defined below.
        pub logs: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::appservice::GetWindowsWebAppLog>,
        >,
        /// The name of this Storage Account.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The list of Outbound IP Addresses for this Windows Web App.
        pub outbound_ip_address_lists: pulumi_wasm_rust::Output<Vec<String>>,
        /// A string representation of the list of Outbound IP Addresses for this Windows Web App.
        pub outbound_ip_addresses: pulumi_wasm_rust::Output<String>,
        /// The list of Possible Outbound IP Addresses that could be used by this Windows Web App.
        pub possible_outbound_ip_address_lists: pulumi_wasm_rust::Output<Vec<String>>,
        /// The string representation of the list of Possible Outbound IP Addresses that could be used by this Windows Web App.
        pub possible_outbound_ip_addresses: pulumi_wasm_rust::Output<String>,
        /// Is Public Network Access enabled for the Windows Web App.
        pub public_network_access_enabled: pulumi_wasm_rust::Output<bool>,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The ID of the Service Plan in which this Windows Web App resides.
        pub service_plan_id: pulumi_wasm_rust::Output<String>,
        /// A `site_config` block as defined below.
        pub site_configs: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::appservice::GetWindowsWebAppSiteConfig>,
        >,
        /// A `site_credential` block as defined below.
        pub site_credentials: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::appservice::GetWindowsWebAppSiteCredential>,
        >,
        /// A `sticky_settings` block as defined below.
        pub sticky_settings: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::appservice::GetWindowsWebAppStickySetting>,
        >,
        /// A `storage_account` block as defined below.
        pub storage_accounts: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::appservice::GetWindowsWebAppStorageAccount>,
        >,
        /// A mapping of tags assigned to the Windows Web App.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// The subnet id which the Windows Web App is vNet Integrated with.
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
    pub fn invoke(args: GetWindowsWebAppArgs) -> GetWindowsWebAppResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:appservice/getWindowsWebApp:getWindowsWebApp".into(),
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
                    name: "backups".into(),
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
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetWindowsWebAppResult {
            app_settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("appSettings").unwrap(),
            ),
            auth_settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("authSettings").unwrap(),
            ),
            auth_settings_v2s: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("authSettingsV2s").unwrap(),
            ),
            backups: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("backups").unwrap(),
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
        }
    }
}
