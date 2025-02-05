pub mod get_windows_web_app {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetWindowsWebAppArgs {
        /// The name of this Windows Web App.
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of the Resource Group where the Windows Web App exists.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
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
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetWindowsWebAppArgs,
    ) -> GetWindowsWebAppResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:appservice/getWindowsWebApp:getWindowsWebApp".into(),
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
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetWindowsWebAppResult {
            app_settings: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("appSettings"),
            ),
            auth_settings: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("authSettings"),
            ),
            auth_settings_v2s: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("authSettingsV2s"),
            ),
            backups: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("backups"),
            ),
            client_affinity_enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("clientAffinityEnabled"),
            ),
            client_certificate_enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("clientCertificateEnabled"),
            ),
            client_certificate_exclusion_paths: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("clientCertificateExclusionPaths"),
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
            ftp_publish_basic_authentication_enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("ftpPublishBasicAuthenticationEnabled"),
            ),
            hosting_environment_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("hostingEnvironmentId"),
            ),
            https_only: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("httpsOnly"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            identities: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("identities"),
            ),
            kind: pulumi_wasm_rust::__private::into_domain(o.extract_field("kind")),
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            logs: pulumi_wasm_rust::__private::into_domain(o.extract_field("logs")),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            outbound_ip_address_lists: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("outboundIpAddressLists"),
            ),
            outbound_ip_addresses: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("outboundIpAddresses"),
            ),
            possible_outbound_ip_address_lists: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("possibleOutboundIpAddressLists"),
            ),
            possible_outbound_ip_addresses: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("possibleOutboundIpAddresses"),
            ),
            public_network_access_enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("publicNetworkAccessEnabled"),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            service_plan_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("servicePlanId"),
            ),
            site_configs: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("siteConfigs"),
            ),
            site_credentials: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("siteCredentials"),
            ),
            sticky_settings: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("stickySettings"),
            ),
            storage_accounts: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("storageAccounts"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            virtual_network_subnet_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("virtualNetworkSubnetId"),
            ),
            webdeploy_publish_basic_authentication_enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("webdeployPublishBasicAuthenticationEnabled"),
            ),
        }
    }
}
