#[allow(clippy::doc_lazy_continuation)]
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
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetAppServiceArgs,
    ) -> GetAppServiceResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:appservice/getAppService:getAppService".into(),
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
        GetAppServiceResult {
            app_service_plan_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("appServicePlanId"),
            ),
            app_settings: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("appSettings"),
            ),
            client_affinity_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("clientAffinityEnabled"),
            ),
            client_cert_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("clientCertEnabled"),
            ),
            connection_strings: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("connectionStrings"),
            ),
            custom_domain_verification_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("customDomainVerificationId"),
            ),
            default_site_hostname: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("defaultSiteHostname"),
            ),
            enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("enabled"),
            ),
            https_only: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("httpsOnly"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            outbound_ip_address_lists: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("outboundIpAddressLists"),
            ),
            outbound_ip_addresses: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("outboundIpAddresses"),
            ),
            possible_outbound_ip_address_lists: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("possibleOutboundIpAddressLists"),
            ),
            possible_outbound_ip_addresses: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("possibleOutboundIpAddresses"),
            ),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            site_configs: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("siteConfigs"),
            ),
            site_credentials: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("siteCredentials"),
            ),
            source_controls: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sourceControls"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
