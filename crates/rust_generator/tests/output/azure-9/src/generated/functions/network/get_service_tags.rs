#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_service_tags {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetServiceTagsArgs {
        /// The Azure Region where the Service Tags exists. This value is not used to filter the results but for specifying the region to request. For filtering by region use `location_filter` instead.  More information can be found here: [Service Tags URL parameters](https://docs.microsoft.com/rest/api/virtualnetwork/servicetags/list#uri-parameters).
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Changes the scope of the service tags. Can be any value that is also valid for `location`. If this field is empty then all address prefixes are considered instead of only location specific ones.
        #[builder(into, default)]
        pub location_filter: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The type of the service for which address prefixes will be fetched. Available service tags can be found here: [Available service tags](https://docs.microsoft.com/azure/virtual-network/service-tags-overview#available-service-tags).
        #[builder(into)]
        pub service: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetServiceTagsResult {
        /// List of address prefixes for the service type (and optionally a specific region).
        pub address_prefixes: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// List of IPv4 addresses for the service type (and optionally a specific region)
        pub ipv4_cidrs: pulumi_gestalt_rust::Output<Vec<String>>,
        /// List of IPv6 addresses for the service type (and optionally a specific region)
        pub ipv6_cidrs: pulumi_gestalt_rust::Output<Vec<String>>,
        pub location: pulumi_gestalt_rust::Output<String>,
        pub location_filter: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of this Service Tags block.
        pub name: pulumi_gestalt_rust::Output<String>,
        pub service: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetServiceTagsArgs,
    ) -> GetServiceTagsResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let location_binding = args.location.get_output(context);
        let location_filter_binding = args.location_filter.get_output(context);
        let service_binding = args.service.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:network/getServiceTags:getServiceTags".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "locationFilter".into(),
                    value: location_filter_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "service".into(),
                    value: service_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetServiceTagsResult {
            address_prefixes: o.get_field("addressPrefixes"),
            id: o.get_field("id"),
            ipv4_cidrs: o.get_field("ipv4Cidrs"),
            ipv6_cidrs: o.get_field("ipv6Cidrs"),
            location: o.get_field("location"),
            location_filter: o.get_field("locationFilter"),
            name: o.get_field("name"),
            service: o.get_field("service"),
        }
    }
}
