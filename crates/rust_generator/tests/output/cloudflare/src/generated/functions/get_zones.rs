#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_zones {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetZonesArgs {
        /// One or more values used to look up zone records. If more than one value is given all values must match in order to be included.
        #[builder(into)]
        pub filter: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::GetZonesFilter,
        >,
    }
    #[allow(dead_code)]
    pub struct GetZonesResult {
        /// One or more values used to look up zone records. If more than one value is given all values must match in order to be included.
        pub filter: pulumi_gestalt_rust::Output<super::super::types::GetZonesFilter>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// A list of zone objects.
        pub zones: pulumi_gestalt_rust::Output<Vec<super::super::types::GetZonesZone>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetZonesArgs,
    ) -> GetZonesResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let filter_binding_1 = args.filter.get_output(context);
        let filter_binding = filter_binding_1.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "cloudflare:index/getZones:getZones".into(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "filter".into(),
                    value: &filter_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetZonesResult {
            filter: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("filter"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            zones: pulumi_gestalt_rust::__private::into_domain(o.extract_field("zones")),
        }
    }
}
