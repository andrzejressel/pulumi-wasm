pub mod nursery {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NurseryArgs {
        /// The sizes of trees available
        #[builder(into, default)]
        pub sizes: pulumi_wasm_rust::InputOrOutput<
            Option<
                std::collections::HashMap<
                    String,
                    super::super::super::types::tree::v1::TreeSize,
                >,
            >,
        >,
        /// The varieties available
        #[builder(into)]
        pub varieties: pulumi_wasm_rust::InputOrOutput<
            Vec<super::super::super::types::tree::v1::RubberTreeVariety>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: NurseryArgs,
    ) {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let sizes_binding = args.sizes.get_output(context).get_inner();
        let varieties_binding = args.varieties.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "plant:tree/v1:Nursery".into(),
            name: name.to_string(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "sizes".into(),
                    value: &sizes_binding,
                },
                register_interface::ObjectField {
                    name: "varieties".into(),
                    value: &varieties_binding,
                },
            ]),
        };
        register_interface::register(context.get_inner(), &request);
    }
}
