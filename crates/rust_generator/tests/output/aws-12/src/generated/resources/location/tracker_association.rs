/// Resource for managing an AWS Location Tracker Association.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = geofence_collection::create(
///         "example",
///         GeofenceCollectionArgs::builder().collection_name("example").build_struct(),
///     );
///     let exampleTracker = tracker::create(
///         "exampleTracker",
///         TrackerArgs::builder().tracker_name("example").build_struct(),
///     );
///     let exampleTrackerAssociation = tracker_association::create(
///         "exampleTrackerAssociation",
///         TrackerAssociationArgs::builder()
///             .consumer_arn("${example.collectionArn}")
///             .tracker_name("${exampleTracker.trackerName}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Location Tracker Association using the `tracker_name|consumer_arn`. For example:
///
/// ```sh
/// $ pulumi import aws:location/trackerAssociation:TrackerAssociation example "tracker_name|consumer_arn"
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod tracker_association {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TrackerAssociationArgs {
        /// The Amazon Resource Name (ARN) for the geofence collection to be associated to tracker resource. Used when you need to specify a resource across all AWS.
        #[builder(into)]
        pub consumer_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the tracker resource to be associated with a geofence collection.
        #[builder(into)]
        pub tracker_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct TrackerAssociationResult {
        /// The Amazon Resource Name (ARN) for the geofence collection to be associated to tracker resource. Used when you need to specify a resource across all AWS.
        pub consumer_arn: pulumi_gestalt_rust::Output<String>,
        /// The name of the tracker resource to be associated with a geofence collection.
        pub tracker_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: TrackerAssociationArgs,
    ) -> TrackerAssociationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let consumer_arn_binding = args.consumer_arn.get_output(context);
        let tracker_name_binding = args.tracker_name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:location/trackerAssociation:TrackerAssociation".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "consumerArn".into(),
                    value: &consumer_arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "trackerName".into(),
                    value: &tracker_name_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        TrackerAssociationResult {
            consumer_arn: o.get_field("consumerArn"),
            tracker_name: o.get_field("trackerName"),
        }
    }
}
