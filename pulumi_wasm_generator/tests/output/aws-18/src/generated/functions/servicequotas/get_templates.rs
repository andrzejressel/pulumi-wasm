pub mod get_templates {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetTemplatesArgs {
        /// AWS Region to which the quota increases apply.
        #[builder(into)]
        pub region: pulumi_wasm_rust::InputOrOutput<String>,
        /// A list of quota increase templates for specified region. See `templates`.
        #[builder(into, default)]
        pub templates: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::super::types::servicequotas::GetTemplatesTemplate>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetTemplatesResult {
        pub id: pulumi_wasm_rust::Output<String>,
        /// AWS Region to which the template applies.
        pub region: pulumi_wasm_rust::Output<String>,
        /// A list of quota increase templates for specified region. See `templates`.
        pub templates: pulumi_wasm_rust::Output<
            Option<Vec<super::super::super::types::servicequotas::GetTemplatesTemplate>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetTemplatesArgs,
    ) -> GetTemplatesResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let region_binding = args.region.get_output(context).get_inner();
        let templates_binding = args.templates.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:servicequotas/getTemplates:getTemplates".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "region".into(),
                    value: &region_binding,
                },
                register_interface::ObjectField {
                    name: "templates".into(),
                    value: &templates_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetTemplatesResult {
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            region: pulumi_wasm_rust::__private::into_domain(o.extract_field("region")),
            templates: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("templates"),
            ),
        }
    }
}
