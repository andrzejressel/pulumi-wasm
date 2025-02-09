#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod func_with_const_input {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FuncWithConstInputArgs {
        #[builder(into, default)]
        pub plain_input: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::constants::ConstStringFixed>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: FuncWithConstInputArgs,
    ) {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let plain_input_binding_1 = args.plain_input.get_output(context);
        let plain_input_binding = plain_input_binding_1.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "mypkg::funcWithConstInput".into(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "plainInput".into(),
                    value: &plain_input_binding,
                },
            ]),
        };
        register_interface::invoke(context.get_inner(), &request);
    }
}
