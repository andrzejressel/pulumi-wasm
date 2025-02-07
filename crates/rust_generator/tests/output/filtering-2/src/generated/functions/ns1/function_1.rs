pub mod function_1 {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct Function1Args {
        #[builder(into, default)]
        pub common_type: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::super::types::common::CommonType>,
        >,
        #[builder(into, default)]
        pub type1: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::super::types::ns1::Type1>,
        >,
    }
    #[allow(dead_code)]
    pub struct Function1Result {
        pub result: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: Function1Args,
    ) -> Function1Result {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let common_type_binding = args.common_type.get_output(context).get_inner();
        let type1_binding = args.type1.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "example:ns1:Function1".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "commonType".into(),
                    value: &common_type_binding,
                },
                register_interface::ObjectField {
                    name: "type1".into(),
                    value: &type1_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        Function1Result {
            result: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("result"),
            ),
        }
    }
}
