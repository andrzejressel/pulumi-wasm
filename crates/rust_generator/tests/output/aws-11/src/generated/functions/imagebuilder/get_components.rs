pub mod get_components {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetComponentsArgs {
        /// Configuration block(s) for filtering. Detailed below.
        #[builder(into, default)]
        pub filters: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::super::types::imagebuilder::GetComponentsFilter>>,
        >,
        /// Owner of the image recipes. Valid values are `Self`, `Shared`, `Amazon` and `ThirdParty`. Defaults to `Self`.
        #[builder(into, default)]
        pub owner: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetComponentsResult {
        /// Set of ARNs of the matched Image Builder Components.
        pub arns: pulumi_wasm_rust::Output<Vec<String>>,
        pub filters: pulumi_wasm_rust::Output<
            Option<Vec<super::super::super::types::imagebuilder::GetComponentsFilter>>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// Set of names of the matched Image Builder Components.
        pub names: pulumi_wasm_rust::Output<Vec<String>>,
        pub owner: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetComponentsArgs,
    ) -> GetComponentsResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let filters_binding = args.filters.get_output(context).get_inner();
        let owner_binding = args.owner.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:imagebuilder/getComponents:getComponents".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "filters".into(),
                    value: &filters_binding,
                },
                register_interface::ObjectField {
                    name: "owner".into(),
                    value: &owner_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetComponentsResult {
            arns: pulumi_wasm_rust::__private::into_domain(o.extract_field("arns")),
            filters: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("filters"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            names: pulumi_wasm_rust::__private::into_domain(o.extract_field("names")),
            owner: pulumi_wasm_rust::__private::into_domain(o.extract_field("owner")),
        }
    }
}
