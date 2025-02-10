/// Resource for managing an EBS (Elastic Block Storage) Fast Snapshot Restore.
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
///     let example = fast_snapshot_restore::create(
///         "example",
///         FastSnapshotRestoreArgs::builder()
///             .availability_zone("us-west-2a")
///             .snapshot_id("${exampleAwsEbsSnapshot.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import EC2 (Elastic Compute Cloud) EBS Fast Snapshot Restore using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:ebs/fastSnapshotRestore:FastSnapshotRestore example us-west-2a,snap-abcdef123456
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod fast_snapshot_restore {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FastSnapshotRestoreArgs {
        /// Availability zone in which to enable fast snapshot restores.
        #[builder(into)]
        pub availability_zone: pulumi_gestalt_rust::InputOrOutput<String>,
        /// ID of the snapshot.
        #[builder(into)]
        pub snapshot_id: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub timeouts: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::ebs::FastSnapshotRestoreTimeouts>,
        >,
    }
    #[allow(dead_code)]
    pub struct FastSnapshotRestoreResult {
        /// Availability zone in which to enable fast snapshot restores.
        pub availability_zone: pulumi_gestalt_rust::Output<String>,
        /// ID of the snapshot.
        pub snapshot_id: pulumi_gestalt_rust::Output<String>,
        /// State of fast snapshot restores. Valid values are `enabling`, `optimizing`, `enabled`, `disabling`, `disabled`.
        pub state: pulumi_gestalt_rust::Output<String>,
        pub timeouts: pulumi_gestalt_rust::Output<
            Option<super::super::types::ebs::FastSnapshotRestoreTimeouts>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: FastSnapshotRestoreArgs,
    ) -> FastSnapshotRestoreResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let availability_zone_binding = args.availability_zone.get_output(context);
        let snapshot_id_binding = args.snapshot_id.get_output(context);
        let timeouts_binding = args.timeouts.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ebs/fastSnapshotRestore:FastSnapshotRestore".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "availabilityZone".into(),
                    value: availability_zone_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "snapshotId".into(),
                    value: snapshot_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "timeouts".into(),
                    value: timeouts_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        FastSnapshotRestoreResult {
            availability_zone: o.get_field("availabilityZone"),
            snapshot_id: o.get_field("snapshotId"),
            state: o.get_field("state"),
            timeouts: o.get_field("timeouts"),
        }
    }
}
