pub mod get_service_tags {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetServiceTagsArgs {
        /// The Azure Region where the Service Tags exists. This value is not used to filter the results but for specifying the region to request. For filtering by region use `location_filter` instead.  More information can be found here: [Service Tags URL parameters](https://docs.microsoft.com/rest/api/virtualnetwork/servicetags/list#uri-parameters).
        #[builder(into)]
        pub location: pulumi_wasm_rust::InputOrOutput<String>,
        /// Changes the scope of the service tags. Can be any value that is also valid for `location`. If this field is empty then all address prefixes are considered instead of only location specific ones.
        #[builder(into, default)]
        pub location_filter: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The type of the service for which address prefixes will be fetched. Available service tags can be found here: [Available service tags](https://docs.microsoft.com/azure/virtual-network/service-tags-overview#available-service-tags).
        #[builder(into)]
        pub service: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetServiceTagsResult {
        /// List of address prefixes for the service type (and optionally a specific region).
        pub address_prefixes: pulumi_wasm_rust::Output<Vec<String>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// List of IPv4 addresses for the service type (and optionally a specific region)
        pub ipv4_cidrs: pulumi_wasm_rust::Output<Vec<String>>,
        /// List of IPv6 addresses for the service type (and optionally a specific region)
        pub ipv6_cidrs: pulumi_wasm_rust::Output<Vec<String>>,
        pub location: pulumi_wasm_rust::Output<String>,
        pub location_filter: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of this Service Tags block.
        pub name: pulumi_wasm_rust::Output<String>,
        pub service: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetServiceTagsArgs,
    ) -> GetServiceTagsResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let location_binding = args.location.get_output(context).get_inner();
        let location_filter_binding = args
            .location_filter
            .get_output(context)
            .get_inner();
        let service_binding = args.service.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:network/getServiceTags:getServiceTags".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "locationFilter".into(),
                    value: &location_filter_binding,
                },
                register_interface::ObjectField {
                    name: "service".into(),
                    value: &service_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetServiceTagsResult {
            address_prefixes: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("addressPrefixes"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            ipv4_cidrs: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("ipv4Cidrs"),
            ),
            ipv6_cidrs: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("ipv6Cidrs"),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            location_filter: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("locationFilter"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            service: pulumi_wasm_rust::__private::into_domain(o.extract_field("service")),
        }
    }
}
