/// Provides a CloudWatch Evidently Segment resource.
///
/// ## Example Usage
///
/// ### Basic
///
/// ```yaml
/// resources:
///   example:
///     type: aws:evidently:Segment
///     properties:
///       name: example
///       pattern: '{"Price":[{"numeric":[">",10,"<=",20]}]}'
///       tags:
///         Key1: example Segment
/// ```
///
/// ### With JSON object in pattern
///
/// ```yaml
/// resources:
///   example:
///     type: aws:evidently:Segment
///     properties:
///       name: example
///       pattern: |2
///           {
///             "Price": [
///               {
///                 "numeric": [">",10,"<=",20]
///               }
///             ]
///           }
///       tags:
///         Key1: example Segment
/// ```
///
/// ### With Description
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = segment::create(
///         "example",
///         SegmentArgs::builder()
///             .description("example")
///             .name("example")
///             .pattern("{\"Price\":[{\"numeric\":[\">\",10,\"<=\",20]}]}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import CloudWatch Evidently Segment using the `arn`. For example:
///
/// ```sh
/// $ pulumi import aws:evidently/segment:Segment example arn:aws:evidently:us-west-2:123456789012:segment/example
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod segment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SegmentArgs {
        /// Specifies the description of the segment.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A name for the segment.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The pattern to use for the segment. For more information about pattern syntax, see [Segment rule pattern syntax](https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/CloudWatch-Evidently-segments.html#CloudWatch-Evidently-segments-syntax.html).
        #[builder(into)]
        pub pattern: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Tags to apply to the segment. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct SegmentResult {
        /// The ARN of the segment.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The date and time that the segment is created.
        pub created_time: pulumi_gestalt_rust::Output<String>,
        /// Specifies the description of the segment.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The number of experiments that this segment is used in. This count includes all current experiments, not just those that are currently running.
        pub experiment_count: pulumi_gestalt_rust::Output<i32>,
        /// The date and time that this segment was most recently updated.
        pub last_updated_time: pulumi_gestalt_rust::Output<String>,
        /// The number of launches that this segment is used in. This count includes all current launches, not just those that are currently running.
        pub launch_count: pulumi_gestalt_rust::Output<i32>,
        /// A name for the segment.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The pattern to use for the segment. For more information about pattern syntax, see [Segment rule pattern syntax](https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/CloudWatch-Evidently-segments.html#CloudWatch-Evidently-segments-syntax.html).
        pub pattern: pulumi_gestalt_rust::Output<String>,
        /// Tags to apply to the segment. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
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
        args: SegmentArgs,
    ) -> SegmentResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let name_binding = args.name.get_output(context);
        let pattern_binding = args.pattern.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:evidently/segment:Segment".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "pattern".into(),
                    value: pattern_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        SegmentResult {
            arn: o.get_field("arn"),
            created_time: o.get_field("createdTime"),
            description: o.get_field("description"),
            experiment_count: o.get_field("experimentCount"),
            last_updated_time: o.get_field("lastUpdatedTime"),
            launch_count: o.get_field("launchCount"),
            name: o.get_field("name"),
            pattern: o.get_field("pattern"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
