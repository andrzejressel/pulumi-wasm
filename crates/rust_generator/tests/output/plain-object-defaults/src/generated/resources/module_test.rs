pub mod module_test {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct moduleTestArgs {
        #[builder(into, default)]
        pub mod1: pulumi_gestalt_rust::InputOrOutput<Option<super::types::mod1::Typ>>,
        #[builder(into, default)]
        pub val: pulumi_gestalt_rust::InputOrOutput<Option<super::types::Typ>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: moduleTestArgs,
    ) {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let mod1_binding = args.mod1.get_output(context).get_inner();
        let val_binding = args.val.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "example:index:moduleTest".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "mod1".into(),
                    value: &mod1_binding,
                },
                register_interface::ObjectField {
                    name: "val".into(),
                    value: &val_binding,
                },
            ]),
        };
        register_interface::register(context.get_inner(), &request);
    }
}
