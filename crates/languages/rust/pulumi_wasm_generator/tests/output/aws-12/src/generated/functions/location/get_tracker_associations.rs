pub mod get_tracker_associations {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetTrackerAssociationsArgs {
        /// Name of the tracker resource associated with a geofence collection.
        #[builder(into)]
        pub tracker_name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetTrackerAssociationsResult {
        /// List of geofence collection ARNs associated to the tracker resource.
        pub consumer_arns: pulumi_wasm_rust::Output<Vec<String>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub tracker_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetTrackerAssociationsArgs,
    ) -> GetTrackerAssociationsResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let tracker_name_binding = args.tracker_name.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:location/getTrackerAssociations:getTrackerAssociations".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "trackerName".into(),
                    value: &tracker_name_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetTrackerAssociationsResult {
            consumer_arns: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("consumerArns"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            tracker_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("trackerName"),
            ),
        }
    }
}
