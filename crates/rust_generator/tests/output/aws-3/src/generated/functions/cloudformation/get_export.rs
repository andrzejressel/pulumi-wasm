#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_export {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetExportArgs {
        /// Name of the export as it appears in the console or from [list-exports](http://docs.aws.amazon.com/cli/latest/reference/cloudformation/list-exports.html)
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetExportResult {
        /// ARN of stack that contains the exported output name and value.
        pub exporting_stack_id: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Value from Cloudformation export identified by the export name found from [list-exports](http://docs.aws.amazon.com/cli/latest/reference/cloudformation/list-exports.html)
        pub value: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetExportArgs,
    ) -> GetExportResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:cloudformation/getExport:getExport".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetExportResult {
            exporting_stack_id: o.get_field("exportingStackId"),
            id: o.get_field("id"),
            name: o.get_field("name"),
            value: o.get_field("value"),
        }
    }
}
