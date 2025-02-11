#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_service {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetServiceArgs {
        /// The name of the API Management service.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Name of the Resource Group in which the API Management Service exists.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A mapping of tags assigned to the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetServiceResult {
        /// Zero or more `additional_location` blocks as defined below
        pub additional_locations: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::apimanagement::GetServiceAdditionalLocation>,
        >,
        /// The URL for the Developer Portal associated with this API Management service.
        pub developer_portal_url: pulumi_gestalt_rust::Output<String>,
        /// Gateway URL of the API Management service in the Region.
        pub gateway_regional_url: pulumi_gestalt_rust::Output<String>,
        /// The URL for the API Management Service's Gateway.
        pub gateway_url: pulumi_gestalt_rust::Output<String>,
        /// A `hostname_configuration` block as defined below.
        pub hostname_configurations: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::apimanagement::GetServiceHostnameConfiguration,
            >,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// (Optional) An `identity` block as defined below.
        pub identities: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::apimanagement::GetServiceIdentity>,
        >,
        /// The location name of the additional region among Azure Data center regions.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The URL for the Management API.
        pub management_api_url: pulumi_gestalt_rust::Output<String>,
        /// Specifies the plan's pricing tier.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The email address from which the notification will be sent.
        pub notification_sender_email: pulumi_gestalt_rust::Output<String>,
        /// The URL of the Publisher Portal.
        pub portal_url: pulumi_gestalt_rust::Output<String>,
        /// Private IP addresses of the API Management service in the additional location, for instances using virtual network mode.
        pub private_ip_addresses: pulumi_gestalt_rust::Output<Vec<String>>,
        /// ID of the standard SKU IPv4 Public IP. Available only for Premium SKU deployed in a virtual network.
        pub public_ip_address_id: pulumi_gestalt_rust::Output<String>,
        /// Public Static Load Balanced IP addresses of the API Management service in the additional location. Available only for Basic, Standard and Premium SKU.
        pub public_ip_addresses: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The email of Publisher/Company of the API Management Service.
        pub publisher_email: pulumi_gestalt_rust::Output<String>,
        /// The name of the Publisher/Company of the API Management Service.
        pub publisher_name: pulumi_gestalt_rust::Output<String>,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The SCM (Source Code Management) endpoint.
        pub scm_url: pulumi_gestalt_rust::Output<String>,
        pub sku_name: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags assigned to the resource.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A `tenant_access` block as defined below.
        pub tenant_accesses: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::apimanagement::GetServiceTenantAccess>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetServiceArgs,
    ) -> GetServiceResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:apimanagement/getService:getService".into(),
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
        GetServiceResult {
            additional_locations: o.get_field("additionalLocations"),
            developer_portal_url: o.get_field("developerPortalUrl"),
            gateway_regional_url: o.get_field("gatewayRegionalUrl"),
            gateway_url: o.get_field("gatewayUrl"),
            hostname_configurations: o.get_field("hostnameConfigurations"),
            id: o.get_field("id"),
            identities: o.get_field("identities"),
            location: o.get_field("location"),
            management_api_url: o.get_field("managementApiUrl"),
            name: o.get_field("name"),
            notification_sender_email: o.get_field("notificationSenderEmail"),
            portal_url: o.get_field("portalUrl"),
            private_ip_addresses: o.get_field("privateIpAddresses"),
            public_ip_address_id: o.get_field("publicIpAddressId"),
            public_ip_addresses: o.get_field("publicIpAddresses"),
            publisher_email: o.get_field("publisherEmail"),
            publisher_name: o.get_field("publisherName"),
            resource_group_name: o.get_field("resourceGroupName"),
            scm_url: o.get_field("scmUrl"),
            sku_name: o.get_field("skuName"),
            tags: o.get_field("tags"),
            tenant_accesses: o.get_field("tenantAccesses"),
        }
    }
}
