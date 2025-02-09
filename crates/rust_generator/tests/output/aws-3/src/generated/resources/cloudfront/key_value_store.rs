/// Resource for managing an AWS CloudFront Key Value Store.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = key_value_store::create(
///         "example",
///         KeyValueStoreArgs::builder()
///             .comment("This is an example key value store")
///             .name("ExampleKeyValueStore")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import CloudFront Key Value Store using the `name`. For example:
///
/// ```sh
/// $ pulumi import aws:cloudfront/keyValueStore:KeyValueStore example example_store
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod key_value_store {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct KeyValueStoreArgs {
        /// Comment.
        #[builder(into, default)]
        pub comment: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Unique name for your CloudFront KeyValueStore.
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub timeouts: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::cloudfront::KeyValueStoreTimeouts>,
        >,
    }
    #[allow(dead_code)]
    pub struct KeyValueStoreResult {
        /// Amazon Resource Name (ARN) identifying your CloudFront KeyValueStore.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Comment.
        pub comment: pulumi_gestalt_rust::Output<Option<String>>,
        /// ETag hash of the KeyValueStore.
        pub etag: pulumi_gestalt_rust::Output<String>,
        pub last_modified_time: pulumi_gestalt_rust::Output<String>,
        /// Unique name for your CloudFront KeyValueStore.
        ///
        /// The following arguments are optional:
        pub name: pulumi_gestalt_rust::Output<String>,
        pub timeouts: pulumi_gestalt_rust::Output<
            Option<super::super::types::cloudfront::KeyValueStoreTimeouts>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: KeyValueStoreArgs,
    ) -> KeyValueStoreResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let comment_binding = args.comment.get_output(context);
        let name_binding = args.name.get_output(context);
        let timeouts_binding = args.timeouts.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:cloudfront/keyValueStore:KeyValueStore".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "comment".into(),
                    value: comment_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "timeouts".into(),
                    value: timeouts_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        KeyValueStoreResult {
            arn: o.get_field("arn"),
            comment: o.get_field("comment"),
            etag: o.get_field("etag"),
            last_modified_time: o.get_field("lastModifiedTime"),
            name: o.get_field("name"),
            timeouts: o.get_field("timeouts"),
        }
    }
}
