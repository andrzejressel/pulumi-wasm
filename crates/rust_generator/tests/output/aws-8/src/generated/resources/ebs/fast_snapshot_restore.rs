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
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: FastSnapshotRestoreArgs,
    ) -> FastSnapshotRestoreResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let availability_zone_binding = args
            .availability_zone
            .get_output(context)
            .get_inner();
        let snapshot_id_binding = args.snapshot_id.get_output(context).get_inner();
        let timeouts_binding = args.timeouts.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ebs/fastSnapshotRestore:FastSnapshotRestore".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "availabilityZone".into(),
                    value: &availability_zone_binding,
                },
                register_interface::ObjectField {
                    name: "snapshotId".into(),
                    value: &snapshot_id_binding,
                },
                register_interface::ObjectField {
                    name: "timeouts".into(),
                    value: &timeouts_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        FastSnapshotRestoreResult {
            availability_zone: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("availabilityZone"),
            ),
            snapshot_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("snapshotId"),
            ),
            state: pulumi_gestalt_rust::__private::into_domain(o.extract_field("state")),
            timeouts: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("timeouts"),
            ),
        }
    }
}
