#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_tracker {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetTrackerArgs {
        /// Key-value map of resource tags for the tracker.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Name of the tracker resource.
        #[builder(into)]
        pub tracker_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetTrackerResult {
        /// Timestamp for when the tracker resource was created in ISO 8601 format.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// Optional description for the tracker resource.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Key identifier for an AWS KMS customer managed key assigned to the Amazon Location resource.
        pub kms_key_id: pulumi_gestalt_rust::Output<String>,
        /// Position filtering method of the tracker resource.
        pub position_filtering: pulumi_gestalt_rust::Output<String>,
        /// Key-value map of resource tags for the tracker.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// ARN for the tracker resource. Used when you need to specify a resource across all AWS.
        pub tracker_arn: pulumi_gestalt_rust::Output<String>,
        pub tracker_name: pulumi_gestalt_rust::Output<String>,
        /// Timestamp for when the tracker resource was last updated in ISO 8601 format.
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetTrackerArgs,
    ) -> GetTrackerResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let tags_binding = args.tags.get_output(context);
        let tracker_name_binding = args.tracker_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:location/getTracker:getTracker".into(),
            version: super::super::super::get_version(),
            object: &[
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
        let o = context.invoke_resource(request);
        GetTrackerResult {
            create_time: o.get_field("createTime"),
            description: o.get_field("description"),
            id: o.get_field("id"),
            kms_key_id: o.get_field("kmsKeyId"),
            position_filtering: o.get_field("positionFiltering"),
            tags: o.get_field("tags"),
            tracker_arn: o.get_field("trackerArn"),
            tracker_name: o.get_field("trackerName"),
            update_time: o.get_field("updateTime"),
        }
    }
}
