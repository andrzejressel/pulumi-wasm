#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_access_points {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetAccessPointsArgs {
        /// EFS File System identifier.
        #[builder(into)]
        pub file_system_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetAccessPointsResult {
        /// Set of Amazon Resource Names (ARNs).
        pub arns: pulumi_gestalt_rust::Output<Vec<String>>,
        pub file_system_id: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Set of identifiers.
        pub ids: pulumi_gestalt_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetAccessPointsArgs,
    ) -> GetAccessPointsResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let file_system_id_binding = args.file_system_id.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:efs/getAccessPoints:getAccessPoints".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "fileSystemId".into(),
                    value: file_system_id_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetAccessPointsResult {
            arns: o.get_field("arns"),
            file_system_id: o.get_field("fileSystemId"),
            id: o.get_field("id"),
            ids: o.get_field("ids"),
        }
    }
}
