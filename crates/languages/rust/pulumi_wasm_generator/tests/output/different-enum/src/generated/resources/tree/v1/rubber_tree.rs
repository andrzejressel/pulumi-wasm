pub mod rubber_tree {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RubberTreeArgs {
        #[builder(into, default)]
        pub container: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::super::types::Container>,
        >,
        #[builder(into)]
        pub diameter: pulumi_wasm_rust::InputOrOutput<
            super::super::super::types::tree::v1::Diameter,
        >,
        #[builder(into, default)]
        pub farm: pulumi_wasm_rust::InputOrOutput<
            Option<
                pulumi_wasm_rust::OneOf2<
                    super::super::super::types::tree::v1::Farm,
                    String,
                >,
            >,
        >,
        #[builder(into, default)]
        pub size: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::super::types::tree::v1::TreeSize>,
        >,
        #[builder(into)]
        pub type_: pulumi_wasm_rust::InputOrOutput<
            super::super::super::types::tree::v1::RubberTreeVariety,
        >,
    }
    #[allow(dead_code)]
    pub struct RubberTreeResult {
        pub container: pulumi_wasm_rust::Output<
            Option<super::super::super::types::Container>,
        >,
        pub diameter: pulumi_wasm_rust::Output<
            super::super::super::types::tree::v1::Diameter,
        >,
        pub farm: pulumi_wasm_rust::Output<
            Option<
                pulumi_wasm_rust::OneOf2<
                    super::super::super::types::tree::v1::Farm,
                    String,
                >,
            >,
        >,
        pub size: pulumi_wasm_rust::Output<
            Option<super::super::super::types::tree::v1::TreeSize>,
        >,
        pub type_: pulumi_wasm_rust::Output<
            super::super::super::types::tree::v1::RubberTreeVariety,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: RubberTreeArgs,
    ) -> RubberTreeResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let container_binding = args.container.get_output(context).get_inner();
        let diameter_binding = args.diameter.get_output(context).get_inner();
        let farm_binding = args.farm.get_output(context).get_inner();
        let size_binding = args.size.get_output(context).get_inner();
        let type__binding = args.type_.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "plant:tree/v1:RubberTree".into(),
            name: name.to_string(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "container".into(),
                    value: &container_binding,
                },
                register_interface::ObjectField {
                    name: "diameter".into(),
                    value: &diameter_binding,
                },
                register_interface::ObjectField {
                    name: "farm".into(),
                    value: &farm_binding,
                },
                register_interface::ObjectField {
                    name: "size".into(),
                    value: &size_binding,
                },
                register_interface::ObjectField {
                    name: "type".into(),
                    value: &type__binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        RubberTreeResult {
            container: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("container"),
            ),
            diameter: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("diameter"),
            ),
            farm: pulumi_wasm_rust::__private::into_domain(o.extract_field("farm")),
            size: pulumi_wasm_rust::__private::into_domain(o.extract_field("size")),
            type_: pulumi_wasm_rust::__private::into_domain(o.extract_field("type")),
        }
    }
}
