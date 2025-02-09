/// Provides a Location Service Tracker.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = tracker::create(
///         "example",
///         TrackerArgs::builder().tracker_name("example").build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_location_tracker` resources using the tracker name. For example:
///
/// ```sh
/// $ pulumi import aws:location/tracker:Tracker example example
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod tracker {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TrackerArgs {
        /// The optional description for the tracker resource.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A key identifier for an AWS KMS customer managed key assigned to the Amazon Location resource.
        #[builder(into, default)]
        pub kms_key_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The position filtering method of the tracker resource. Valid values: `TimeBased`, `DistanceBased`, `AccuracyBased`. Default: `TimeBased`.
        #[builder(into, default)]
        pub position_filtering: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Key-value tags for the tracker. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The name of the tracker resource.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub tracker_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct TrackerResult {
        /// The timestamp for when the tracker resource was created in ISO 8601 format.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// The optional description for the tracker resource.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// A key identifier for an AWS KMS customer managed key assigned to the Amazon Location resource.
        pub kms_key_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The position filtering method of the tracker resource. Valid values: `TimeBased`, `DistanceBased`, `AccuracyBased`. Default: `TimeBased`.
        pub position_filtering: pulumi_gestalt_rust::Output<Option<String>>,
        /// Key-value tags for the tracker. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The Amazon Resource Name (ARN) for the tracker resource. Used when you need to specify a resource across all AWS.
        pub tracker_arn: pulumi_gestalt_rust::Output<String>,
        /// The name of the tracker resource.
        ///
        /// The following arguments are optional:
        pub tracker_name: pulumi_gestalt_rust::Output<String>,
        /// The timestamp for when the tracker resource was last updated in ISO 8601 format.
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: TrackerArgs,
    ) -> TrackerResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let kms_key_id_binding = args.kms_key_id.get_output(context);
        let position_filtering_binding = args.position_filtering.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let tracker_name_binding = args.tracker_name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:location/tracker:Tracker".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "kmsKeyId".into(),
                    value: kms_key_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "positionFiltering".into(),
                    value: position_filtering_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "trackerName".into(),
                    value: tracker_name_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        TrackerResult {
            create_time: o.get_field("createTime"),
            description: o.get_field("description"),
            kms_key_id: o.get_field("kmsKeyId"),
            position_filtering: o.get_field("positionFiltering"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            tracker_arn: o.get_field("trackerArn"),
            tracker_name: o.get_field("trackerName"),
            update_time: o.get_field("updateTime"),
        }
    }
}
