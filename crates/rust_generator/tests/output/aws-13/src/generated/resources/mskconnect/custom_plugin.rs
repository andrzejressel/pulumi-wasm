/// Provides an Amazon MSK Connect Custom Plugin Resource.
///
/// ## Example Usage
///
/// ### Basic configuration
///
/// ```yaml
/// resources:
///   example:
///     type: aws:s3:BucketV2
///     properties:
///       bucket: example
///   exampleBucketObjectv2:
///     type: aws:s3:BucketObjectv2
///     name: example
///     properties:
///       bucket: ${example.id}
///       key: debezium.zip
///       source:
///         fn::FileAsset: debezium.zip
///   exampleCustomPlugin:
///     type: aws:mskconnect:CustomPlugin
///     name: example
///     properties:
///       name: debezium-example
///       contentType: ZIP
///       location:
///         s3:
///           bucketArn: ${example.arn}
///           fileKey: ${exampleBucketObjectv2.key}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import MSK Connect Custom Plugin using the plugin's `arn`. For example:
///
/// ```sh
/// $ pulumi import aws:mskconnect/customPlugin:CustomPlugin example 'arn:aws:kafkaconnect:eu-central-1:123456789012:custom-plugin/debezium-example/abcdefgh-1234-5678-9abc-defghijklmno-4'
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod custom_plugin {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CustomPluginArgs {
        /// The type of the plugin file. Allowed values are `ZIP` and `JAR`.
        #[builder(into)]
        pub content_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A summary description of the custom plugin.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Information about the location of a custom plugin. See `location` Block for details.
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::mskconnect::CustomPluginLocation,
        >,
        /// The name of the custom plugin..
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct CustomPluginResult {
        /// the Amazon Resource Name (ARN) of the custom plugin.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The type of the plugin file. Allowed values are `ZIP` and `JAR`.
        pub content_type: pulumi_gestalt_rust::Output<String>,
        /// A summary description of the custom plugin.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// an ID of the latest successfully created revision of the custom plugin.
        pub latest_revision: pulumi_gestalt_rust::Output<i32>,
        /// Information about the location of a custom plugin. See `location` Block for details.
        pub location: pulumi_gestalt_rust::Output<
            super::super::types::mskconnect::CustomPluginLocation,
        >,
        /// The name of the custom plugin..
        pub name: pulumi_gestalt_rust::Output<String>,
        /// the state of the custom plugin.
        pub state: pulumi_gestalt_rust::Output<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        ///
        /// The following arguments are optional:
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: CustomPluginArgs,
    ) -> CustomPluginResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let content_type_binding = args.content_type.get_output(context);
        let description_binding = args.description.get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:mskconnect/customPlugin:CustomPlugin".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "contentType".into(),
                    value: &content_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        CustomPluginResult {
            arn: o.get_field("arn"),
            content_type: o.get_field("contentType"),
            description: o.get_field("description"),
            latest_revision: o.get_field("latestRevision"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            state: o.get_field("state"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
