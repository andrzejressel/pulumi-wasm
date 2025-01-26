pub mod get_standard {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetStandardArgs {
        /// The name of this Logic App.
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of the Resource Group where the Logic App exists.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// A `site_config` object as defined below.
        #[builder(into, default)]
        pub site_config: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::super::types::logicapps::GetStandardSiteConfig>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetStandardResult {
        /// The ID of the App Service Plan.
        pub app_service_plan_id: pulumi_wasm_rust::Output<String>,
        /// A map of key-value pairs for [App Settings](https://docs.microsoft.com/azure/azure-functions/functions-app-settings) and custom values.
        pub app_settings: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Controls the allowed range for bundle versions.
        pub bundle_version: pulumi_wasm_rust::Output<String>,
        /// Should the Logic App send session affinity cookies, which route client requests in the same session to the same instance.
        pub client_affinity_enabled: pulumi_wasm_rust::Output<bool>,
        /// The mode of the Logic App's client certificates requirement for incoming requests.
        pub client_certificate_mode: pulumi_wasm_rust::Output<String>,
        /// A `connection_string` block as defined below.
        pub connection_strings: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::logicapps::GetStandardConnectionString>,
        >,
        /// The custom domain verification of the Logic App.
        pub custom_domain_verification_id: pulumi_wasm_rust::Output<String>,
        /// The default hostname of the Logic App.
        pub default_hostname: pulumi_wasm_rust::Output<String>,
        /// Whether the Logic App is enabled.
        pub enabled: pulumi_wasm_rust::Output<bool>,
        /// Whether the Logic App can only be accessed via HTTPS.
        pub https_only: pulumi_wasm_rust::Output<bool>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// An `identity` block as defined below.
        pub identities: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::logicapps::GetStandardIdentity>,
        >,
        /// The kind of the Logic App.
        pub kind: pulumi_wasm_rust::Output<String>,
        /// The Azure location where the Logic App Standard exists.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The name for this IP Restriction.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The outbound IP addresses of the Logic App.
        pub outbound_ip_addresses: pulumi_wasm_rust::Output<String>,
        /// The possible outbound IP addresses of the Logic App.
        pub possible_outbound_ip_addresses: pulumi_wasm_rust::Output<String>,
        /// Whether Public Network Access should be enabled or not.
        pub public_network_access: pulumi_wasm_rust::Output<String>,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A `site_config` object as defined below.
        pub site_config: pulumi_wasm_rust::Output<
            super::super::super::types::logicapps::GetStandardSiteConfig,
        >,
        /// A `site_credential` block as defined below, which contains the site-level credentials used to publish to this Logic App.
        pub site_credentials: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::logicapps::GetStandardSiteCredential>,
        >,
        /// The access key which will be used to access the backend storage account for the Logic App.
        pub storage_account_access_key: pulumi_wasm_rust::Output<String>,
        /// The backend storage account name which will be used by this Logic App (e.g. for Stateful workflows data).
        pub storage_account_name: pulumi_wasm_rust::Output<String>,
        /// The name of the share used by the logic app.
        pub storage_account_share_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags assigned to the resource.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// Whether the logic app should use the bundled extension package.
        pub use_extension_bundle: pulumi_wasm_rust::Output<bool>,
        /// The runtime version associated with the Logic App.
        pub version: pulumi_wasm_rust::Output<String>,
        /// The Virtual Network Subnet ID used for this IP Restriction.
        pub virtual_network_subnet_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetStandardArgs,
    ) -> GetStandardResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let site_config_binding = args.site_config.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:logicapps/getStandard:getStandard".into(),
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
                register_interface::ObjectField {
                    name: "siteConfig".into(),
                    value: &site_config_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetStandardResult {
            app_service_plan_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("appServicePlanId"),
            ),
            app_settings: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("appSettings"),
            ),
            bundle_version: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("bundleVersion"),
            ),
            client_affinity_enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("clientAffinityEnabled"),
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
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            outbound_ip_addresses: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("outboundIpAddresses"),
            ),
            possible_outbound_ip_addresses: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("possibleOutboundIpAddresses"),
            ),
            public_network_access: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("publicNetworkAccess"),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            site_config: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("siteConfig"),
            ),
            site_credentials: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("siteCredentials"),
            ),
            storage_account_access_key: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("storageAccountAccessKey"),
            ),
            storage_account_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("storageAccountName"),
            ),
            storage_account_share_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("storageAccountShareName"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            use_extension_bundle: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("useExtensionBundle"),
            ),
            version: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("version"),
            ),
            virtual_network_subnet_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("virtualNetworkSubnetId"),
            ),
        }
    }
}
