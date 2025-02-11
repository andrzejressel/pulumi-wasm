#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_app_service {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetAppServiceArgs {
        /// The name of the App Service.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Name of the Resource Group where the App Service exists.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetAppServiceResult {
        /// The ID of the App Service Plan within which the App Service exists.
        pub app_service_plan_id: pulumi_gestalt_rust::Output<String>,
        /// A key-value pair of App Settings for the App Service.
        pub app_settings: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Does the App Service send session affinity cookies, which route client requests in the same session to the same instance?
        pub client_affinity_enabled: pulumi_gestalt_rust::Output<bool>,
        /// Does the App Service require client certificates for incoming requests?
        pub client_cert_enabled: pulumi_gestalt_rust::Output<bool>,
        /// An `connection_string` block as defined below.
        pub connection_strings: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::appservice::GetAppServiceConnectionString>,
        >,
        /// An identifier used by App Service to perform domain ownership verification via DNS TXT record.
        pub custom_domain_verification_id: pulumi_gestalt_rust::Output<String>,
        /// The Default Hostname associated with the App Service - such as `mysite.azurewebsites.net`
        pub default_site_hostname: pulumi_gestalt_rust::Output<String>,
        /// Is the App Service Enabled?
        pub enabled: pulumi_gestalt_rust::Output<bool>,
        /// Can the App Service only be accessed via HTTPS?
        pub https_only: pulumi_gestalt_rust::Output<bool>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The Azure location where the App Service exists.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The name for this IP Restriction.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A list of outbound IP addresses - such as `["52.23.25.3", "52.143.43.12"]`
        pub outbound_ip_address_lists: pulumi_gestalt_rust::Output<Vec<String>>,
        /// A comma separated list of outbound IP addresses - such as `52.23.25.3,52.143.43.12`
        pub outbound_ip_addresses: pulumi_gestalt_rust::Output<String>,
        /// A list of outbound IP addresses - such as `["52.23.25.3", "52.143.43.12", "52.143.43.17"]` - not all of which are necessarily in use. Superset of `outbound_ip_address_list`.
        pub possible_outbound_ip_address_lists: pulumi_gestalt_rust::Output<Vec<String>>,
        /// A comma separated list of outbound IP addresses - such as `52.23.25.3,52.143.43.12,52.143.43.17` - not all of which are necessarily in use. Superset of `outbound_ip_addresses`.
        pub possible_outbound_ip_addresses: pulumi_gestalt_rust::Output<String>,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A `site_config` block as defined below.
        pub site_configs: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::appservice::GetAppServiceSiteConfig>,
        >,
        pub site_credentials: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::appservice::GetAppServiceSiteCredential>,
        >,
        /// A `source_control` block as defined below.
        pub source_controls: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::appservice::GetAppServiceSourceControl>,
        >,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetAppServiceArgs,
    ) -> GetAppServiceResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:appservice/getAppService:getAppService".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetAppServiceResult {
            app_service_plan_id: o.get_field("appServicePlanId"),
            app_settings: o.get_field("appSettings"),
            client_affinity_enabled: o.get_field("clientAffinityEnabled"),
            client_cert_enabled: o.get_field("clientCertEnabled"),
            connection_strings: o.get_field("connectionStrings"),
            custom_domain_verification_id: o.get_field("customDomainVerificationId"),
            default_site_hostname: o.get_field("defaultSiteHostname"),
            enabled: o.get_field("enabled"),
            https_only: o.get_field("httpsOnly"),
            id: o.get_field("id"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            outbound_ip_address_lists: o.get_field("outboundIpAddressLists"),
            outbound_ip_addresses: o.get_field("outboundIpAddresses"),
            possible_outbound_ip_address_lists: o
                .get_field("possibleOutboundIpAddressLists"),
            possible_outbound_ip_addresses: o.get_field("possibleOutboundIpAddresses"),
            resource_group_name: o.get_field("resourceGroupName"),
            site_configs: o.get_field("siteConfigs"),
            site_credentials: o.get_field("siteCredentials"),
            source_controls: o.get_field("sourceControls"),
            tags: o.get_field("tags"),
        }
    }
}
