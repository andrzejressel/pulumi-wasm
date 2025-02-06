pub mod get_regions {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetRegionsArgs {
        /// If true the source will query all regions regardless of availability.
        #[builder(into, default)]
        pub all_regions: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Configuration block(s) to use as filters. Detailed below.
        #[builder(into, default)]
        pub filters: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::GetRegionsFilter>>,
        >,
        /// Identifier of the current partition (e.g., `aws` in AWS Commercial, `aws-cn` in AWS China).
        #[builder(into, default)]
        pub id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetRegionsResult {
        pub all_regions: pulumi_wasm_rust::Output<Option<bool>>,
        pub filters: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::GetRegionsFilter>>,
        >,
        /// Identifier of the current partition (e.g., `aws` in AWS Commercial, `aws-cn` in AWS China).
        pub id: pulumi_wasm_rust::Output<String>,
        /// Names of regions that meets the criteria.
        pub names: pulumi_wasm_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetRegionsArgs,
    ) -> GetRegionsResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let all_regions_binding = args.all_regions.get_output(context).get_inner();
        let filters_binding = args.filters.get_output(context).get_inner();
        let id_binding = args.id.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:index/getRegions:getRegions".into(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "allRegions".into(),
                    value: &all_regions_binding,
                },
                register_interface::ObjectField {
                    name: "filters".into(),
                    value: &filters_binding,
                },
                register_interface::ObjectField {
                    name: "id".into(),
                    value: &id_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetRegionsResult {
            all_regions: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("allRegions"),
            ),
            filters: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("filters"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            names: pulumi_wasm_rust::__private::into_domain(o.extract_field("names")),
        }
    }
}
