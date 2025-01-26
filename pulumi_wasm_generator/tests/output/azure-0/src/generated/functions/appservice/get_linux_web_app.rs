pub mod get_linux_web_app {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetLinuxWebAppArgs {
        /// The name of this Linux Web App.
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of the Resource Group where the Linux Web App exists.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetLinuxWebAppResult {
        /// An `app_metadata` block as defined below.
        pub app_metadata: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// An `app_settings` block as defined below.
        pub app_settings: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// An `auth_settings` block as defined below.
        pub auth_settings: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::appservice::GetLinuxWebAppAuthSetting>,
        >,
        /// An `auth_settings_v2` block as defined below.
        pub auth_settings_v2s: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::appservice::GetLinuxWebAppAuthSettingsV2>,
        >,
        /// The current availability state. Possible values are `Normal`, `Limited`, and `DisasterRecoveryMode`.
        pub availability: pulumi_wasm_rust::Output<String>,
        /// A `backup` block as defined below.
        pub backups: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::appservice::GetLinuxWebAppBackup>,
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
            Vec<super::super::super::types::appservice::GetLinuxWebAppConnectionString>,
        >,
        /// The identifier used by App Service to perform domain ownership verification via DNS TXT record.
        pub custom_domain_verification_id: pulumi_wasm_rust::Output<String>,
        /// The default hostname of the Linux Web App.
        pub default_hostname: pulumi_wasm_rust::Output<String>,
        /// Is the Backup enabled?
        pub enabled: pulumi_wasm_rust::Output<bool>,
        /// Are the default FTP Basic Authentication publishing credentials enabled.
        pub ftp_publish_basic_authentication_enabled: pulumi_wasm_rust::Output<bool>,
        /// The ID of the App Service Environment used by App Service.
        pub hosting_environment_id: pulumi_wasm_rust::Output<String>,
        /// Should the Linux Web App require HTTPS connections.
        pub https_only: pulumi_wasm_rust::Output<bool>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// A `identity` block as defined below.
        pub identities: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::appservice::GetLinuxWebAppIdentity>,
        >,
        pub key_vault_reference_identity_id: pulumi_wasm_rust::Output<String>,
        /// The Kind value for this Linux Web App.
        pub kind: pulumi_wasm_rust::Output<String>,
        /// The Azure Region where the Linux Web App exists.
        pub location: pulumi_wasm_rust::Output<String>,
        /// A `logs` block as defined below.
        pub logs: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::appservice::GetLinuxWebAppLog>,
        >,
        /// The name of this Storage Account.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A `outbound_ip_address_list` block as defined below.
        pub outbound_ip_address_lists: pulumi_wasm_rust::Output<Vec<String>>,
        /// A comma separated list of outbound IP addresses - such as `52.23.25.3,52.143.43.12`.
        pub outbound_ip_addresses: pulumi_wasm_rust::Output<String>,
        /// A `possible_outbound_ip_address_list` block as defined below.
        pub possible_outbound_ip_address_lists: pulumi_wasm_rust::Output<Vec<String>>,
        /// A comma separated list of outbound IP addresses - such as `52.23.25.3,52.143.43.12,52.143.43.17` - not all of which are necessarily in use. Superset of `outbound_ip_addresses`.
        pub possible_outbound_ip_addresses: pulumi_wasm_rust::Output<String>,
        /// Is Public Network Access enabled for this Linux Web App.
        pub public_network_access_enabled: pulumi_wasm_rust::Output<bool>,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The ID of the Service Plan that this Linux Web App exists in.
        pub service_plan_id: pulumi_wasm_rust::Output<String>,
        /// A `site_config` block as defined below.
        pub site_configs: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::appservice::GetLinuxWebAppSiteConfig>,
        >,
        /// A `site_credential` block as defined below.
        pub site_credentials: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::appservice::GetLinuxWebAppSiteCredential>,
        >,
        /// A `sticky_settings` block as defined below.
        pub sticky_settings: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::appservice::GetLinuxWebAppStickySetting>,
        >,
        /// A `storage_account` block as defined below.
        pub storage_accounts: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::appservice::GetLinuxWebAppStorageAccount>,
        >,
        /// A mapping of tags assigned to the Linux Web App.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// The current usage state. Possible values are `Normal` and `Exceeded`.
        pub usage: pulumi_wasm_rust::Output<String>,
        /// The subnet id which the Linux Web App is vNet Integrated with.
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
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetLinuxWebAppArgs,
    ) -> GetLinuxWebAppResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:appservice/getLinuxWebApp:getLinuxWebApp".into(),
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
                    name: "appMetadata".into(),
                },
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
                    name: "keyVaultReferenceIdentityId".into(),
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
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetLinuxWebAppResult {
            app_metadata: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("appMetadata").unwrap(),
            ),
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
            key_vault_reference_identity_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("keyVaultReferenceIdentityId").unwrap(),
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
