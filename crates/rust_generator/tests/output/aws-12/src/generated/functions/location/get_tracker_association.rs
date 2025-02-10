#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_tracker_association {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetTrackerAssociationArgs {
        /// ARN of the geofence collection associated to tracker resource.
        #[builder(into)]
        pub consumer_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Name of the tracker resource associated with a geofence collection.
        #[builder(into)]
        pub tracker_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetTrackerAssociationResult {
        pub consumer_arn: pulumi_gestalt_rust::Output<String>,
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
        args: GetTrackerAssociationArgs,
    ) -> GetTrackerAssociationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let consumer_arn_binding = args.consumer_arn.get_output(context);
        let tracker_name_binding = args.tracker_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:location/getTrackerAssociation:getTrackerAssociation".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "consumerArn".into(),
                    value: consumer_arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "trackerName".into(),
                    value: tracker_name_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetTrackerAssociationResult {
            consumer_arn: o.get_field("consumerArn"),
            id: o.get_field("id"),
            tracker_name: o.get_field("trackerName"),
        }
    }
}
