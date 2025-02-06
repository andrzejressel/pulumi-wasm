pub mod get_subnetworks {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetSubnetworksArgs {
        /// A string filter as defined in the [REST API](https://cloud.google.com/compute/docs/reference/rest/v1/subnetworks/list#query-parameters).
        #[builder(into, default)]
        pub filter: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The region this subnetwork has been created in. If
        /// unspecified, this defaults to the region configured in the provider.
        #[builder(into, default)]
        pub region: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetSubnetworksResult {
        pub filter: pulumi_wasm_rust::Output<Option<String>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        pub region: pulumi_wasm_rust::Output<Option<String>>,
        /// A list of all retrieved GCE subnetworks. Structure is defined below.
        pub subnetworks: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::compute::GetSubnetworksSubnetwork>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetSubnetworksArgs,
    ) -> GetSubnetworksResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let filter_binding = args.filter.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let region_binding = args.region.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:compute/getSubnetworks:getSubnetworks".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "filter".into(),
                    value: &filter_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "region".into(),
                    value: &region_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetSubnetworksResult {
            filter: pulumi_wasm_rust::__private::into_domain(o.extract_field("filter")),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            project: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            region: pulumi_wasm_rust::__private::into_domain(o.extract_field("region")),
            subnetworks: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("subnetworks"),
            ),
        }
    }
}
