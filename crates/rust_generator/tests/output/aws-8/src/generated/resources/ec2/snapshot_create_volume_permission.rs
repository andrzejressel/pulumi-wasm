/// Adds permission to create volumes off of a given EBS Snapshot.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = volume::create(
///         "example",
///         VolumeArgs::builder().availability_zone("us-west-2a").size(40).build_struct(),
///     );
///     let examplePerm = snapshot_create_volume_permission::create(
///         "examplePerm",
///         SnapshotCreateVolumePermissionArgs::builder()
///             .account_id("12345678")
///             .snapshot_id("${exampleSnapshot.id}")
///             .build_struct(),
///     );
///     let exampleSnapshot = snapshot::create(
///         "exampleSnapshot",
///         SnapshotArgs::builder().volume_id("${example.id}").build_struct(),
///     );
/// }
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod snapshot_create_volume_permission {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SnapshotCreateVolumePermissionArgs {
        /// An AWS Account ID to add create volume permissions. The AWS Account cannot be the snapshot's owner
        #[builder(into)]
        pub account_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A snapshot ID
        #[builder(into)]
        pub snapshot_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct SnapshotCreateVolumePermissionResult {
        /// An AWS Account ID to add create volume permissions. The AWS Account cannot be the snapshot's owner
        pub account_id: pulumi_gestalt_rust::Output<String>,
        /// A snapshot ID
        pub snapshot_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: SnapshotCreateVolumePermissionArgs,
    ) -> SnapshotCreateVolumePermissionResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let account_id_binding = args.account_id.get_output(context).get_inner();
        let snapshot_id_binding = args.snapshot_id.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ec2/snapshotCreateVolumePermission:SnapshotCreateVolumePermission"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding,
                },
                register_interface::ObjectField {
                    name: "snapshotId".into(),
                    value: &snapshot_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        SnapshotCreateVolumePermissionResult {
            account_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("accountId"),
            ),
            snapshot_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("snapshotId"),
            ),
        }
    }
}
