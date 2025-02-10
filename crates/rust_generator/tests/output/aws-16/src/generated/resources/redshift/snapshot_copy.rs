/// Resource for managing an AWS Redshift Snapshot Copy.
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
///     let example = snapshot_copy::create(
///         "example",
///         SnapshotCopyArgs::builder()
///             .cluster_identifier("${exampleAwsRedshiftCluster.id}")
///             .destination_region("us-east-1")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Redshift Snapshot Copy using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:redshift/snapshotCopy:SnapshotCopy example cluster-id-12345678
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod snapshot_copy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SnapshotCopyArgs {
        /// Identifier of the source cluster.
        #[builder(into)]
        pub cluster_identifier: pulumi_gestalt_rust::InputOrOutput<String>,
        /// AWS Region to copy snapshots to.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub destination_region: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Number of days to retain newly copied snapshots in the destination AWS Region after they are copied from the source AWS Region. If the value is `-1`, the manual snapshot is retained indefinitely.
        #[builder(into, default)]
        pub manual_snapshot_retention_period: pulumi_gestalt_rust::InputOrOutput<
            Option<i32>,
        >,
        /// Number of days to retain automated snapshots in the destination region after they are copied from the source region.
        #[builder(into, default)]
        pub retention_period: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Name of the snapshot copy grant to use when snapshots of an AWS KMS-encrypted cluster are copied to the destination region.
        #[builder(into, default)]
        pub snapshot_copy_grant_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct SnapshotCopyResult {
        /// Identifier of the source cluster.
        pub cluster_identifier: pulumi_gestalt_rust::Output<String>,
        /// AWS Region to copy snapshots to.
        ///
        /// The following arguments are optional:
        pub destination_region: pulumi_gestalt_rust::Output<String>,
        /// Number of days to retain newly copied snapshots in the destination AWS Region after they are copied from the source AWS Region. If the value is `-1`, the manual snapshot is retained indefinitely.
        pub manual_snapshot_retention_period: pulumi_gestalt_rust::Output<i32>,
        /// Number of days to retain automated snapshots in the destination region after they are copied from the source region.
        pub retention_period: pulumi_gestalt_rust::Output<i32>,
        /// Name of the snapshot copy grant to use when snapshots of an AWS KMS-encrypted cluster are copied to the destination region.
        pub snapshot_copy_grant_name: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SnapshotCopyArgs,
    ) -> SnapshotCopyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let cluster_identifier_binding = args.cluster_identifier.get_output(context);
        let destination_region_binding = args.destination_region.get_output(context);
        let manual_snapshot_retention_period_binding = args
            .manual_snapshot_retention_period
            .get_output(context);
        let retention_period_binding = args.retention_period.get_output(context);
        let snapshot_copy_grant_name_binding = args
            .snapshot_copy_grant_name
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:redshift/snapshotCopy:SnapshotCopy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clusterIdentifier".into(),
                    value: cluster_identifier_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "destinationRegion".into(),
                    value: destination_region_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "manualSnapshotRetentionPeriod".into(),
                    value: manual_snapshot_retention_period_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "retentionPeriod".into(),
                    value: retention_period_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "snapshotCopyGrantName".into(),
                    value: snapshot_copy_grant_name_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        SnapshotCopyResult {
            cluster_identifier: o.get_field("clusterIdentifier"),
            destination_region: o.get_field("destinationRegion"),
            manual_snapshot_retention_period: o
                .get_field("manualSnapshotRetentionPeriod"),
            retention_period: o.get_field("retentionPeriod"),
            snapshot_copy_grant_name: o.get_field("snapshotCopyGrantName"),
        }
    }
}
