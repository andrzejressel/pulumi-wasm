#[allow(clippy::doc_lazy_continuation)]
pub mod resource {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ResourceArgs {
        #[builder(into, default)]
        pub baz: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ResourceResult {
        pub baz: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ResourceArgs,
    ) -> ResourceResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let baz_binding = args.baz.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "foo-bar:deeply/nested/module:Resource".into(),
            name: name.to_string(),
            version: super::super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "baz".into(),
                    value: &baz_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ResourceResult {
            baz: pulumi_gestalt_rust::__private::into_domain(o.extract_field("baz")),
        }
    }
}
