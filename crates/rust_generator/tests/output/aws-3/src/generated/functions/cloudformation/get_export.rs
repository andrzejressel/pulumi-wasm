pub mod get_export {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetExportArgs {
        /// Name of the export as it appears in the console or from [list-exports](http://docs.aws.amazon.com/cli/latest/reference/cloudformation/list-exports.html)
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetExportResult {
        /// ARN of stack that contains the exported output name and value.
        pub exporting_stack_id: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        /// Value from Cloudformation export identified by the export name found from [list-exports](http://docs.aws.amazon.com/cli/latest/reference/cloudformation/list-exports.html)
        pub value: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetExportArgs,
    ) -> GetExportResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:cloudformation/getExport:getExport".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetExportResult {
            exporting_stack_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("exportingStackId"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            value: pulumi_wasm_rust::__private::into_domain(o.extract_field("value")),
        }
    }
}
