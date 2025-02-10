#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_linux_web_app {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetLinuxWebAppArgs {
        /// The name of this Linux Web App.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Resource Group where the Linux Web App exists.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetLinuxWebAppResult {
        /// An `app_metadata` block as defined below.
        pub app_metadata: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// An `app_settings` block as defined below.
        pub app_settings: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// An `auth_settings` block as defined below.
        pub auth_settings: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::appservice::GetLinuxWebAppAuthSetting>,
        >,
        /// An `auth_settings_v2` block as defined below.
        pub auth_settings_v2s: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::appservice::GetLinuxWebAppAuthSettingsV2>,
        >,
        /// The current availability state. Possible values are `Normal`, `Limited`, and `DisasterRecoveryMode`.
        pub availability: pulumi_gestalt_rust::Output<String>,
        /// A `backup` block as defined below.
        pub backups: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::appservice::GetLinuxWebAppBackup>,
        >,
        /// Is Client Affinity enabled?
        pub client_affinity_enabled: pulumi_gestalt_rust::Output<bool>,
        /// Are Client Certificates enabled?
        pub client_certificate_enabled: pulumi_gestalt_rust::Output<bool>,
        /// Paths to exclude when using client certificates, separated by ;
        pub client_certificate_exclusion_paths: pulumi_gestalt_rust::Output<String>,
        /// The Client Certificate mode.
        pub client_certificate_mode: pulumi_gestalt_rust::Output<String>,
        /// A `connection_string` block as defined below.
        pub connection_strings: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::appservice::GetLinuxWebAppConnectionString>,
        >,
        /// The identifier used by App Service to perform domain ownership verification via DNS TXT record.
        pub custom_domain_verification_id: pulumi_gestalt_rust::Output<String>,
        /// The default hostname of the Linux Web App.
        pub default_hostname: pulumi_gestalt_rust::Output<String>,
        /// Is the Backup enabled?
        pub enabled: pulumi_gestalt_rust::Output<bool>,
        /// Are the default FTP Basic Authentication publishing credentials enabled.
        pub ftp_publish_basic_authentication_enabled: pulumi_gestalt_rust::Output<bool>,
        /// The ID of the App Service Environment used by App Service.
        pub hosting_environment_id: pulumi_gestalt_rust::Output<String>,
        /// Should the Linux Web App require HTTPS connections.
        pub https_only: pulumi_gestalt_rust::Output<bool>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// A `identity` block as defined below.
        pub identities: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::appservice::GetLinuxWebAppIdentity>,
        >,
        pub key_vault_reference_identity_id: pulumi_gestalt_rust::Output<String>,
        /// The Kind value for this Linux Web App.
        pub kind: pulumi_gestalt_rust::Output<String>,
        /// The Azure Region where the Linux Web App exists.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// A `logs` block as defined below.
        pub logs: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::appservice::GetLinuxWebAppLog>,
        >,
        /// The name of this Storage Account.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A `outbound_ip_address_list` block as defined below.
        pub outbound_ip_address_lists: pulumi_gestalt_rust::Output<Vec<String>>,
        /// A comma separated list of outbound IP addresses - such as `52.23.25.3,52.143.43.12`.
        pub outbound_ip_addresses: pulumi_gestalt_rust::Output<String>,
        /// A `possible_outbound_ip_address_list` block as defined below.
        pub possible_outbound_ip_address_lists: pulumi_gestalt_rust::Output<Vec<String>>,
        /// A comma separated list of outbound IP addresses - such as `52.23.25.3,52.143.43.12,52.143.43.17` - not all of which are necessarily in use. Superset of `outbound_ip_addresses`.
        pub possible_outbound_ip_addresses: pulumi_gestalt_rust::Output<String>,
        /// Is Public Network Access enabled for this Linux Web App.
        pub public_network_access_enabled: pulumi_gestalt_rust::Output<bool>,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Service Plan that this Linux Web App exists in.
        pub service_plan_id: pulumi_gestalt_rust::Output<String>,
        /// A `site_config` block as defined below.
        pub site_configs: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::appservice::GetLinuxWebAppSiteConfig>,
        >,
        /// A `site_credential` block as defined below.
        pub site_credentials: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::appservice::GetLinuxWebAppSiteCredential>,
        >,
        /// A `sticky_settings` block as defined below.
        pub sticky_settings: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::appservice::GetLinuxWebAppStickySetting>,
        >,
        /// A `storage_account` block as defined below.
        pub storage_accounts: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::appservice::GetLinuxWebAppStorageAccount>,
        >,
        /// A mapping of tags assigned to the Linux Web App.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// The current usage state. Possible values are `Normal` and `Exceeded`.
        pub usage: pulumi_gestalt_rust::Output<String>,
        /// The subnet id which the Linux Web App is vNet Integrated with.
        pub virtual_network_subnet_id: pulumi_gestalt_rust::Output<String>,
        /// Are the default WebDeploy Basic Authentication publishing credentials enabled.
        pub webdeploy_publish_basic_authentication_enabled: pulumi_gestalt_rust::Output<
            bool,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetLinuxWebAppArgs,
    ) -> GetLinuxWebAppResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:appservice/getLinuxWebApp:getLinuxWebApp".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetLinuxWebAppResult {
            app_metadata: o.get_field("appMetadata"),
            app_settings: o.get_field("appSettings"),
            auth_settings: o.get_field("authSettings"),
            auth_settings_v2s: o.get_field("authSettingsV2s"),
            availability: o.get_field("availability"),
            backups: o.get_field("backups"),
            client_affinity_enabled: o.get_field("clientAffinityEnabled"),
            client_certificate_enabled: o.get_field("clientCertificateEnabled"),
            client_certificate_exclusion_paths: o
                .get_field("clientCertificateExclusionPaths"),
            client_certificate_mode: o.get_field("clientCertificateMode"),
            connection_strings: o.get_field("connectionStrings"),
            custom_domain_verification_id: o.get_field("customDomainVerificationId"),
            default_hostname: o.get_field("defaultHostname"),
            enabled: o.get_field("enabled"),
            ftp_publish_basic_authentication_enabled: o
                .get_field("ftpPublishBasicAuthenticationEnabled"),
            hosting_environment_id: o.get_field("hostingEnvironmentId"),
            https_only: o.get_field("httpsOnly"),
            id: o.get_field("id"),
            identities: o.get_field("identities"),
            key_vault_reference_identity_id: o.get_field("keyVaultReferenceIdentityId"),
            kind: o.get_field("kind"),
            location: o.get_field("location"),
            logs: o.get_field("logs"),
            name: o.get_field("name"),
            outbound_ip_address_lists: o.get_field("outboundIpAddressLists"),
            outbound_ip_addresses: o.get_field("outboundIpAddresses"),
            possible_outbound_ip_address_lists: o
                .get_field("possibleOutboundIpAddressLists"),
            possible_outbound_ip_addresses: o.get_field("possibleOutboundIpAddresses"),
            public_network_access_enabled: o.get_field("publicNetworkAccessEnabled"),
            resource_group_name: o.get_field("resourceGroupName"),
            service_plan_id: o.get_field("servicePlanId"),
            site_configs: o.get_field("siteConfigs"),
            site_credentials: o.get_field("siteCredentials"),
            sticky_settings: o.get_field("stickySettings"),
            storage_accounts: o.get_field("storageAccounts"),
            tags: o.get_field("tags"),
            usage: o.get_field("usage"),
            virtual_network_subnet_id: o.get_field("virtualNetworkSubnetId"),
            webdeploy_publish_basic_authentication_enabled: o
                .get_field("webdeployPublishBasicAuthenticationEnabled"),
        }
    }
}
