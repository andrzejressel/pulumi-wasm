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
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SnapshotCreateVolumePermissionArgs,
    ) -> SnapshotCreateVolumePermissionResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let account_id_binding = args.account_id.get_output(context);
        let snapshot_id_binding = args.snapshot_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ec2/snapshotCreateVolumePermission:SnapshotCreateVolumePermission"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountId".into(),
                    value: account_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "snapshotId".into(),
                    value: snapshot_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        SnapshotCreateVolumePermissionResult {
            account_id: o.get_field("accountId"),
            snapshot_id: o.get_field("snapshotId"),
        }
    }
}
