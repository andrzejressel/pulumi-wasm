#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_tracker_associations {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetTrackerAssociationsArgs {
        /// Name of the tracker resource associated with a geofence collection.
        #[builder(into)]
        pub tracker_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetTrackerAssociationsResult {
        /// List of geofence collection ARNs associated to the tracker resource.
        pub consumer_arns: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub tracker_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetTrackerAssociationsArgs,
    ) -> GetTrackerAssociationsResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let tracker_name_binding = args.tracker_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:location/getTrackerAssociations:getTrackerAssociations".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "trackerName".into(),
                    value: tracker_name_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetTrackerAssociationsResult {
            consumer_arns: o.get_field("consumerArns"),
            id: o.get_field("id"),
            tracker_name: o.get_field("trackerName"),
        }
    }
}
