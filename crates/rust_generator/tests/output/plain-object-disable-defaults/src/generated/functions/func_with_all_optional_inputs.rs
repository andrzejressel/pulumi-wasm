#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod func_with_all_optional_inputs {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FuncWithAllOptionalInputsArgs {
        /// Property A
        #[builder(into, default)]
        pub a: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::HelmReleaseSettings>,
        >,
        /// Property B
        #[builder(into, default)]
        pub b: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct FuncWithAllOptionalInputsResult {
        pub r: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: FuncWithAllOptionalInputsArgs,
    ) -> FuncWithAllOptionalInputsResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let a_binding_1 = args.a.get_output(context);
        let a_binding = a_binding_1.get_inner();
        let b_binding_1 = args.b.get_output(context);
        let b_binding = b_binding_1.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "mypkg::funcWithAllOptionalInputs".into(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "a".into(),
                    value: &a_binding,
                },
                register_interface::ObjectField {
                    name: "b".into(),
                    value: &b_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        FuncWithAllOptionalInputsResult {
            r: pulumi_gestalt_rust::__private::into_domain(o.extract_field("r")),
        }
    }
}
