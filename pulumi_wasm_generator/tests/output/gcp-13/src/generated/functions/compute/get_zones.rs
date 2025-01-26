pub mod get_zones {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetZonesArgs {
        /// Project from which to list available zones. Defaults to project declared in the provider.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Region from which to list available zones. Defaults to region declared in the provider.
        #[builder(into, default)]
        pub region: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Allows to filter list of zones based on their current status. Status can be either `UP` or `DOWN`.
        /// Defaults to no filtering (all available zones - both `UP` and `DOWN`).
        #[builder(into, default)]
        pub status: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetZonesResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// A list of zones available in the given region
        pub names: pulumi_wasm_rust::Output<Vec<String>>,
        pub project: pulumi_wasm_rust::Output<String>,
        pub region: pulumi_wasm_rust::Output<Option<String>>,
        pub status: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetZonesArgs,
    ) -> GetZonesResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let project_binding = args.project.get_output(context).get_inner();
        let region_binding = args.region.get_output(context).get_inner();
        let status_binding = args.status.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:compute/getZones:getZones".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "region".into(),
                    value: &region_binding,
                },
                register_interface::ObjectField {
                    name: "status".into(),
                    value: &status_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetZonesResult {
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            names: pulumi_wasm_rust::__private::into_domain(o.extract_field("names")),
            project: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            region: pulumi_wasm_rust::__private::into_domain(o.extract_field("region")),
            status: pulumi_wasm_rust::__private::into_domain(o.extract_field("status")),
        }
    }
}
