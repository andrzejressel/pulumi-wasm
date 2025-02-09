#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_function {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetFunctionArgs {
        /// Name of the CloudFront function.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Function’s stage, either `DEVELOPMENT` or `LIVE`.
        #[builder(into)]
        pub stage: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetFunctionResult {
        /// ARN identifying your CloudFront Function.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Source code of the function
        pub code: pulumi_gestalt_rust::Output<String>,
        /// Comment.
        pub comment: pulumi_gestalt_rust::Output<String>,
        /// ETag hash of the function
        pub etag: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// List of `aws.cloudfront.KeyValueStore` ARNs associated to the function.
        pub key_value_store_associations: pulumi_gestalt_rust::Output<Vec<String>>,
        /// When this resource was last modified.
        pub last_modified_time: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Identifier of the function's runtime.
        pub runtime: pulumi_gestalt_rust::Output<String>,
        pub stage: pulumi_gestalt_rust::Output<String>,
        /// Status of the function. Can be `UNPUBLISHED`, `UNASSOCIATED` or `ASSOCIATED`.
        pub status: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetFunctionArgs,
    ) -> GetFunctionResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let stage_binding = args.stage.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:cloudfront/getFunction:getFunction".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "stage".into(),
                    value: stage_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetFunctionResult {
            arn: o.get_field("arn"),
            code: o.get_field("code"),
            comment: o.get_field("comment"),
            etag: o.get_field("etag"),
            id: o.get_field("id"),
            key_value_store_associations: o.get_field("keyValueStoreAssociations"),
            last_modified_time: o.get_field("lastModifiedTime"),
            name: o.get_field("name"),
            runtime: o.get_field("runtime"),
            stage: o.get_field("stage"),
            status: o.get_field("status"),
        }
    }
}
