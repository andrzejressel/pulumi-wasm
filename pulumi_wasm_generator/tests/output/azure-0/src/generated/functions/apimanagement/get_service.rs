pub mod get_service {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetServiceArgs {
        /// The name of the API Management service.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// The Name of the Resource Group in which the API Management Service exists.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags assigned to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetServiceResult {
        /// Zero or more `additional_location` blocks as defined below
        pub additional_locations: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::apimanagement::GetServiceAdditionalLocation>,
        >,
        /// The URL for the Developer Portal associated with this API Management service.
        pub developer_portal_url: pulumi_wasm_rust::Output<String>,
        /// Gateway URL of the API Management service in the Region.
        pub gateway_regional_url: pulumi_wasm_rust::Output<String>,
        /// The URL for the API Management Service's Gateway.
        pub gateway_url: pulumi_wasm_rust::Output<String>,
        /// A `hostname_configuration` block as defined below.
        pub hostname_configurations: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::apimanagement::GetServiceHostnameConfiguration,
            >,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// (Optional) An `identity` block as defined below.
        pub identities: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::apimanagement::GetServiceIdentity>,
        >,
        /// The location name of the additional region among Azure Data center regions.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The URL for the Management API.
        pub management_api_url: pulumi_wasm_rust::Output<String>,
        /// Specifies the plan's pricing tier.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The email address from which the notification will be sent.
        pub notification_sender_email: pulumi_wasm_rust::Output<String>,
        /// The URL of the Publisher Portal.
        pub portal_url: pulumi_wasm_rust::Output<String>,
        /// Private IP addresses of the API Management service in the additional location, for instances using virtual network mode.
        pub private_ip_addresses: pulumi_wasm_rust::Output<Vec<String>>,
        /// ID of the standard SKU IPv4 Public IP. Available only for Premium SKU deployed in a virtual network.
        pub public_ip_address_id: pulumi_wasm_rust::Output<String>,
        /// Public Static Load Balanced IP addresses of the API Management service in the additional location. Available only for Basic, Standard and Premium SKU.
        pub public_ip_addresses: pulumi_wasm_rust::Output<Vec<String>>,
        /// The email of Publisher/Company of the API Management Service.
        pub publisher_email: pulumi_wasm_rust::Output<String>,
        /// The name of the Publisher/Company of the API Management Service.
        pub publisher_name: pulumi_wasm_rust::Output<String>,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The SCM (Source Code Management) endpoint.
        pub scm_url: pulumi_wasm_rust::Output<String>,
        pub sku_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags assigned to the resource.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A `tenant_access` block as defined below.
        pub tenant_accesses: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::apimanagement::GetServiceTenantAccess>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetServiceArgs) -> GetServiceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:apimanagement/getService:getService".into(),
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
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "additionalLocations".into(),
                },
                register_interface::ResultField {
                    name: "developerPortalUrl".into(),
                },
                register_interface::ResultField {
                    name: "gatewayRegionalUrl".into(),
                },
                register_interface::ResultField {
                    name: "gatewayUrl".into(),
                },
                register_interface::ResultField {
                    name: "hostnameConfigurations".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "identities".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "managementApiUrl".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "notificationSenderEmail".into(),
                },
                register_interface::ResultField {
                    name: "portalUrl".into(),
                },
                register_interface::ResultField {
                    name: "privateIpAddresses".into(),
                },
                register_interface::ResultField {
                    name: "publicIpAddressId".into(),
                },
                register_interface::ResultField {
                    name: "publicIpAddresses".into(),
                },
                register_interface::ResultField {
                    name: "publisherEmail".into(),
                },
                register_interface::ResultField {
                    name: "publisherName".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "scmUrl".into(),
                },
                register_interface::ResultField {
                    name: "skuName".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tenantAccesses".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetServiceResult {
            additional_locations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("additionalLocations").unwrap(),
            ),
            developer_portal_url: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("developerPortalUrl").unwrap(),
            ),
            gateway_regional_url: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("gatewayRegionalUrl").unwrap(),
            ),
            gateway_url: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("gatewayUrl").unwrap(),
            ),
            hostname_configurations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hostnameConfigurations").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            identities: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("identities").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            management_api_url: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("managementApiUrl").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            notification_sender_email: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("notificationSenderEmail").unwrap(),
            ),
            portal_url: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("portalUrl").unwrap(),
            ),
            private_ip_addresses: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("privateIpAddresses").unwrap(),
            ),
            public_ip_address_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("publicIpAddressId").unwrap(),
            ),
            public_ip_addresses: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("publicIpAddresses").unwrap(),
            ),
            publisher_email: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("publisherEmail").unwrap(),
            ),
            publisher_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("publisherName").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            scm_url: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("scmUrl").unwrap(),
            ),
            sku_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("skuName").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tenant_accesses: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tenantAccesses").unwrap(),
            ),
        }
    }
}
