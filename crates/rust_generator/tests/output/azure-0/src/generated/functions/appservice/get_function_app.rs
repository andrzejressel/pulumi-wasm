#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_function_app {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetFunctionAppArgs {
        /// The name of the Function App resource.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Resource Group where the Function App exists.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetFunctionAppResult {
        /// The ID of the App Service Plan within which to create this Function App.
        pub app_service_plan_id: pulumi_gestalt_rust::Output<String>,
        /// A key-value pair of App Settings.
        pub app_settings: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The mode of the Function App's client certificates requirement for incoming requests.
        pub client_cert_mode: pulumi_gestalt_rust::Output<String>,
        /// An `connection_string` block as defined below.
        pub connection_strings: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::appservice::GetFunctionAppConnectionString>,
        >,
        /// An identifier used by App Service to perform domain ownership verification via DNS TXT record.
        pub custom_domain_verification_id: pulumi_gestalt_rust::Output<String>,
        /// The default hostname associated with the Function App.
        pub default_hostname: pulumi_gestalt_rust::Output<String>,
        /// Is the Function App enabled?
        pub enabled: pulumi_gestalt_rust::Output<bool>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// A `identity` block as defined below.
        pub identities: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::appservice::GetFunctionAppIdentity>,
        >,
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The name for this IP Restriction.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A string indicating the Operating System type for this function app.
        pub os_type: pulumi_gestalt_rust::Output<String>,
        /// A comma separated list of outbound IP addresses.
        pub outbound_ip_addresses: pulumi_gestalt_rust::Output<String>,
        /// A comma separated list of outbound IP addresses, not all of which are necessarily in use. Superset of `outbound_ip_addresses`.
        pub possible_outbound_ip_addresses: pulumi_gestalt_rust::Output<String>,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        pub site_configs: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::appservice::GetFunctionAppSiteConfig>,
        >,
        /// A `site_credential` block as defined below, which contains the site-level credentials used to publish to this App Service.
        pub site_credentials: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::appservice::GetFunctionAppSiteCredential>,
        >,
        /// A `source_control` block as defined below.
        pub source_controls: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::appservice::GetFunctionAppSourceControl>,
        >,
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetFunctionAppArgs,
    ) -> GetFunctionAppResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:appservice/getFunctionApp:getFunctionApp".into(),
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
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetFunctionAppResult {
            app_service_plan_id: o.get_field("appServicePlanId"),
            app_settings: o.get_field("appSettings"),
            client_cert_mode: o.get_field("clientCertMode"),
            connection_strings: o.get_field("connectionStrings"),
            custom_domain_verification_id: o.get_field("customDomainVerificationId"),
            default_hostname: o.get_field("defaultHostname"),
            enabled: o.get_field("enabled"),
            id: o.get_field("id"),
            identities: o.get_field("identities"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            os_type: o.get_field("osType"),
            outbound_ip_addresses: o.get_field("outboundIpAddresses"),
            possible_outbound_ip_addresses: o.get_field("possibleOutboundIpAddresses"),
            resource_group_name: o.get_field("resourceGroupName"),
            site_configs: o.get_field("siteConfigs"),
            site_credentials: o.get_field("siteCredentials"),
            source_controls: o.get_field("sourceControls"),
            tags: o.get_field("tags"),
        }
    }
}
