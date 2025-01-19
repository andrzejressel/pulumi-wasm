/// Resource for managing an AWS Redshift Snapshot Copy.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod snapshot_copy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SnapshotCopyArgs {
        /// Identifier of the source cluster.
        #[builder(into)]
        pub cluster_identifier: pulumi_wasm_rust::Output<String>,
        /// AWS Region to copy snapshots to.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub destination_region: pulumi_wasm_rust::Output<String>,
        /// Number of days to retain newly copied snapshots in the destination AWS Region after they are copied from the source AWS Region. If the value is `-1`, the manual snapshot is retained indefinitely.
        #[builder(into, default)]
        pub manual_snapshot_retention_period: pulumi_wasm_rust::Output<Option<i32>>,
        /// Number of days to retain automated snapshots in the destination region after they are copied from the source region.
        #[builder(into, default)]
        pub retention_period: pulumi_wasm_rust::Output<Option<i32>>,
        /// Name of the snapshot copy grant to use when snapshots of an AWS KMS-encrypted cluster are copied to the destination region.
        #[builder(into, default)]
        pub snapshot_copy_grant_name: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct SnapshotCopyResult {
        /// Identifier of the source cluster.
        pub cluster_identifier: pulumi_wasm_rust::Output<String>,
        /// AWS Region to copy snapshots to.
        ///
        /// The following arguments are optional:
        pub destination_region: pulumi_wasm_rust::Output<String>,
        /// Number of days to retain newly copied snapshots in the destination AWS Region after they are copied from the source AWS Region. If the value is `-1`, the manual snapshot is retained indefinitely.
        pub manual_snapshot_retention_period: pulumi_wasm_rust::Output<i32>,
        /// Number of days to retain automated snapshots in the destination region after they are copied from the source region.
        pub retention_period: pulumi_wasm_rust::Output<i32>,
        /// Name of the snapshot copy grant to use when snapshots of an AWS KMS-encrypted cluster are copied to the destination region.
        pub snapshot_copy_grant_name: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: SnapshotCopyArgs) -> SnapshotCopyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let cluster_identifier_binding = args.cluster_identifier.get_inner();
        let destination_region_binding = args.destination_region.get_inner();
        let manual_snapshot_retention_period_binding = args
            .manual_snapshot_retention_period
            .get_inner();
        let retention_period_binding = args.retention_period.get_inner();
        let snapshot_copy_grant_name_binding = args.snapshot_copy_grant_name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:redshift/snapshotCopy:SnapshotCopy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "clusterIdentifier".into(),
                    value: &cluster_identifier_binding,
                },
                register_interface::ObjectField {
                    name: "destinationRegion".into(),
                    value: &destination_region_binding,
                },
                register_interface::ObjectField {
                    name: "manualSnapshotRetentionPeriod".into(),
                    value: &manual_snapshot_retention_period_binding,
                },
                register_interface::ObjectField {
                    name: "retentionPeriod".into(),
                    value: &retention_period_binding,
                },
                register_interface::ObjectField {
                    name: "snapshotCopyGrantName".into(),
                    value: &snapshot_copy_grant_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "clusterIdentifier".into(),
                },
                register_interface::ResultField {
                    name: "destinationRegion".into(),
                },
                register_interface::ResultField {
                    name: "manualSnapshotRetentionPeriod".into(),
                },
                register_interface::ResultField {
                    name: "retentionPeriod".into(),
                },
                register_interface::ResultField {
                    name: "snapshotCopyGrantName".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        SnapshotCopyResult {
            cluster_identifier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clusterIdentifier").unwrap(),
            ),
            destination_region: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("destinationRegion").unwrap(),
            ),
            manual_snapshot_retention_period: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("manualSnapshotRetentionPeriod").unwrap(),
            ),
            retention_period: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("retentionPeriod").unwrap(),
            ),
            snapshot_copy_grant_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("snapshotCopyGrantName").unwrap(),
            ),
        }
    }
}
