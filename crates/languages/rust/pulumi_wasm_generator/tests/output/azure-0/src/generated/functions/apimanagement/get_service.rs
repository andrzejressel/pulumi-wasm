pub mod get_service {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetServiceArgs {
        /// The name of the API Management service.
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The Name of the Resource Group in which the API Management Service exists.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// A mapping of tags assigned to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
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
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetServiceArgs,
    ) -> GetServiceResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
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
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetServiceResult {
            additional_locations: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("additionalLocations"),
            ),
            developer_portal_url: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("developerPortalUrl"),
            ),
            gateway_regional_url: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("gatewayRegionalUrl"),
            ),
            gateway_url: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("gatewayUrl"),
            ),
            hostname_configurations: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("hostnameConfigurations"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            identities: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("identities"),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            management_api_url: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("managementApiUrl"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            notification_sender_email: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("notificationSenderEmail"),
            ),
            portal_url: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("portalUrl"),
            ),
            private_ip_addresses: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("privateIpAddresses"),
            ),
            public_ip_address_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("publicIpAddressId"),
            ),
            public_ip_addresses: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("publicIpAddresses"),
            ),
            publisher_email: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("publisherEmail"),
            ),
            publisher_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("publisherName"),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            scm_url: pulumi_wasm_rust::__private::into_domain(o.extract_field("scmUrl")),
            sku_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("skuName"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            tenant_accesses: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("tenantAccesses"),
            ),
        }
    }
}
