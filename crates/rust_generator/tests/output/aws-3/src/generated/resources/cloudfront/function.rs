/// Provides a CloudFront Function resource. With CloudFront Functions in Amazon CloudFront, you can write lightweight functions in JavaScript for high-scale, latency-sensitive CDN customizations.
///
/// See [CloudFront Functions](https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/cloudfront-functions.html)
///
/// > **NOTE:** You cannot delete a function if it’s associated with a cache behavior. First, update your distributions to remove the function association from all cache behaviors, then delete the function.
///
/// ## Example Usage
///
/// ## Import
///
/// Using `pulumi import`, import CloudFront Functions using the `name`. For example:
///
/// ```sh
/// $ pulumi import aws:cloudfront/function:Function test my_test_function
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod function {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FunctionArgs {
        /// Source code of the function
        #[builder(into)]
        pub code: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Comment.
        #[builder(into, default)]
        pub comment: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// List of `aws.cloudfront.KeyValueStore` ARNs to be associated to the function. AWS limits associations to on key value store per function.
        #[builder(into, default)]
        pub key_value_store_associations: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// Unique name for your CloudFront Function.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Whether to publish creation/change as Live CloudFront Function Version. Defaults to `true`.
        #[builder(into, default)]
        pub publish: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Identifier of the function's runtime. Valid values are `cloudfront-js-1.0` and `cloudfront-js-2.0`.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub runtime: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct FunctionResult {
        /// Amazon Resource Name (ARN) identifying your CloudFront Function.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Source code of the function
        pub code: pulumi_gestalt_rust::Output<String>,
        /// Comment.
        pub comment: pulumi_gestalt_rust::Output<Option<String>>,
        /// ETag hash of the function. This is the value for the `DEVELOPMENT` stage of the function.
        pub etag: pulumi_gestalt_rust::Output<String>,
        /// List of `aws.cloudfront.KeyValueStore` ARNs to be associated to the function. AWS limits associations to on key value store per function.
        pub key_value_store_associations: pulumi_gestalt_rust::Output<
            Option<Vec<String>>,
        >,
        /// ETag hash of any `LIVE` stage of the function.
        pub live_stage_etag: pulumi_gestalt_rust::Output<String>,
        /// Unique name for your CloudFront Function.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Whether to publish creation/change as Live CloudFront Function Version. Defaults to `true`.
        pub publish: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Identifier of the function's runtime. Valid values are `cloudfront-js-1.0` and `cloudfront-js-2.0`.
        ///
        /// The following arguments are optional:
        pub runtime: pulumi_gestalt_rust::Output<String>,
        /// Status of the function. Can be `UNPUBLISHED`, `UNASSOCIATED` or `ASSOCIATED`.
        pub status: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: FunctionArgs,
    ) -> FunctionResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let code_binding = args.code.get_output(context);
        let comment_binding = args.comment.get_output(context);
        let key_value_store_associations_binding = args
            .key_value_store_associations
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let publish_binding = args.publish.get_output(context);
        let runtime_binding = args.runtime.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:cloudfront/function:Function".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "code".into(),
                    value: &code_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "comment".into(),
                    value: &comment_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "keyValueStoreAssociations".into(),
                    value: &key_value_store_associations_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "publish".into(),
                    value: &publish_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "runtime".into(),
                    value: &runtime_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        FunctionResult {
            arn: o.get_field("arn"),
            code: o.get_field("code"),
            comment: o.get_field("comment"),
            etag: o.get_field("etag"),
            key_value_store_associations: o.get_field("keyValueStoreAssociations"),
            live_stage_etag: o.get_field("liveStageEtag"),
            name: o.get_field("name"),
            publish: o.get_field("publish"),
            runtime: o.get_field("runtime"),
            status: o.get_field("status"),
        }
    }
}
