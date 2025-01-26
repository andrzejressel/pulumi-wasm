/// Resource for managing an AWS Location Tracker Association.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod tracker_association {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TrackerAssociationArgs {
        /// The Amazon Resource Name (ARN) for the geofence collection to be associated to tracker resource. Used when you need to specify a resource across all AWS.
        #[builder(into)]
        pub consumer_arn: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of the tracker resource to be associated with a geofence collection.
        #[builder(into)]
        pub tracker_name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct TrackerAssociationResult {
        /// The Amazon Resource Name (ARN) for the geofence collection to be associated to tracker resource. Used when you need to specify a resource across all AWS.
        pub consumer_arn: pulumi_wasm_rust::Output<String>,
        /// The name of the tracker resource to be associated with a geofence collection.
        pub tracker_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: TrackerAssociationArgs,
    ) -> TrackerAssociationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let consumer_arn_binding = args.consumer_arn.get_output(context).get_inner();
        let tracker_name_binding = args.tracker_name.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:location/trackerAssociation:TrackerAssociation".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "consumerArn".into(),
                    value: &consumer_arn_binding,
                },
                register_interface::ObjectField {
                    name: "trackerName".into(),
                    value: &tracker_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "consumerArn".into(),
                },
                register_interface::ResultField {
                    name: "trackerName".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        TrackerAssociationResult {
            consumer_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("consumerArn").unwrap(),
            ),
            tracker_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("trackerName").unwrap(),
            ),
        }
    }
}
