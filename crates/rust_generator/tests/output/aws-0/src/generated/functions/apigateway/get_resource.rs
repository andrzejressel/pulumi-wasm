#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_resource {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetResourceArgs {
        /// Full path of the resource.  If no path is found, an error will be returned.
        #[builder(into)]
        pub path: pulumi_gestalt_rust::InputOrOutput<String>,
        /// REST API id that owns the resource. If no REST API is found, an error will be returned.
        #[builder(into)]
        pub rest_api_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetResourceResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Set to the ID of the parent Resource.
        pub parent_id: pulumi_gestalt_rust::Output<String>,
        pub path: pulumi_gestalt_rust::Output<String>,
        /// Set to the path relative to the parent Resource.
        pub path_part: pulumi_gestalt_rust::Output<String>,
        pub rest_api_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetResourceArgs,
    ) -> GetResourceResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let path_binding = args.path.get_output(context);
        let rest_api_id_binding = args.rest_api_id.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:apigateway/getResource:getResource".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "path".into(),
                    value: &path_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "restApiId".into(),
                    value: &rest_api_id_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetResourceResult {
            id: o.get_field("id"),
            parent_id: o.get_field("parentId"),
            path: o.get_field("path"),
            path_part: o.get_field("pathPart"),
            rest_api_id: o.get_field("restApiId"),
        }
    }
}
