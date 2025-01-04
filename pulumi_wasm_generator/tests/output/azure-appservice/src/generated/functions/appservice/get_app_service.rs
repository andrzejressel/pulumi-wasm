pub mod get_app_service {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetAppServiceArgs {
        /// The name of the App Service.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// The Name of the Resource Group where the App Service exists.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetAppServiceResult {
        /// The ID of the App Service Plan within which the App Service exists.
        pub app_service_plan_id: pulumi_wasm_rust::Output<String>,
        /// A key-value pair of App Settings for the App Service.
        pub app_settings: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Does the App Service send session affinity cookies, which route client requests in the same session to the same instance?
        pub client_affinity_enabled: pulumi_wasm_rust::Output<bool>,
        /// Does the App Service require client certificates for incoming requests?
        pub client_cert_enabled: pulumi_wasm_rust::Output<bool>,
        /// An `connection_string` block as defined below.
        pub connection_strings: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::appservice::GetAppServiceConnectionString>,
        >,
        /// An identifier used by App Service to perform domain ownership verification via DNS TXT record.
        pub custom_domain_verification_id: pulumi_wasm_rust::Output<String>,
        /// The Default Hostname associated with the App Service - such as `mysite.azurewebsites.net`
        pub default_site_hostname: pulumi_wasm_rust::Output<String>,
        /// Is the App Service Enabled?
        pub enabled: pulumi_wasm_rust::Output<bool>,
        /// Can the App Service only be accessed via HTTPS?
        pub https_only: pulumi_wasm_rust::Output<bool>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The Azure location where the App Service exists.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The name for this IP Restriction.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A list of outbound IP addresses - such as `["52.23.25.3", "52.143.43.12"]`
        pub outbound_ip_address_lists: pulumi_wasm_rust::Output<Vec<String>>,
        /// A comma separated list of outbound IP addresses - such as `52.23.25.3,52.143.43.12`
        pub outbound_ip_addresses: pulumi_wasm_rust::Output<String>,
        /// A list of outbound IP addresses - such as `["52.23.25.3", "52.143.43.12", "52.143.43.17"]` - not all of which are necessarily in use. Superset of `outbound_ip_address_list`.
        pub possible_outbound_ip_address_lists: pulumi_wasm_rust::Output<Vec<String>>,
        /// A comma separated list of outbound IP addresses - such as `52.23.25.3,52.143.43.12,52.143.43.17` - not all of which are necessarily in use. Superset of `outbound_ip_addresses`.
        pub possible_outbound_ip_addresses: pulumi_wasm_rust::Output<String>,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A `site_config` block as defined below.
        pub site_configs: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::appservice::GetAppServiceSiteConfig>,
        >,
        pub site_credentials: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::appservice::GetAppServiceSiteCredential>,
        >,
        /// A `source_control` block as defined below.
        pub source_controls: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::appservice::GetAppServiceSourceControl>,
        >,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetAppServiceArgs) -> GetAppServiceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:appservice/getAppService:getAppService".into(),
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
                    name: "appServicePlanId".into(),
                },
                register_interface::ResultField {
                    name: "appSettings".into(),
                },
                register_interface::ResultField {
                    name: "clientAffinityEnabled".into(),
                },
                register_interface::ResultField {
                    name: "clientCertEnabled".into(),
                },
                register_interface::ResultField {
                    name: "connectionStrings".into(),
                },
                register_interface::ResultField {
                    name: "customDomainVerificationId".into(),
                },
                register_interface::ResultField {
                    name: "defaultSiteHostname".into(),
                },
                register_interface::ResultField {
                    name: "enabled".into(),
                },
                register_interface::ResultField {
                    name: "httpsOnly".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
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
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "siteConfigs".into(),
                },
                register_interface::ResultField {
                    name: "siteCredentials".into(),
                },
                register_interface::ResultField {
                    name: "sourceControls".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetAppServiceResult {
            app_service_plan_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("appServicePlanId").unwrap(),
            ),
            app_settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("appSettings").unwrap(),
            ),
            client_affinity_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clientAffinityEnabled").unwrap(),
            ),
            client_cert_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clientCertEnabled").unwrap(),
            ),
            connection_strings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("connectionStrings").unwrap(),
            ),
            custom_domain_verification_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customDomainVerificationId").unwrap(),
            ),
            default_site_hostname: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultSiteHostname").unwrap(),
            ),
            enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enabled").unwrap(),
            ),
            https_only: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("httpsOnly").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
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
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            site_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("siteConfigs").unwrap(),
            ),
            site_credentials: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("siteCredentials").unwrap(),
            ),
            source_controls: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceControls").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}
