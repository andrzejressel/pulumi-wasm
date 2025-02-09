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
        context: &pulumi_gestalt_rust::Context,
        args: GetZonesArgs,
    ) -> GetZonesResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let filter_binding = args.filter.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "cloudflare:index/getZones:getZones".into(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "filter".into(),
                    value: filter_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetZonesResult {
            filter: o.get_field("filter"),
            id: o.get_field("id"),
            zones: o.get_field("zones"),
        }
    }
}
