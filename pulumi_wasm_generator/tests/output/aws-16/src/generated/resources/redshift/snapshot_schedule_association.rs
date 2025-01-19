/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = cluster::create(
///         "default",
///         ClusterArgs::builder()
///             .cluster_identifier("tf-redshift-cluster")
///             .cluster_type("single-node")
///             .database_name("mydb")
///             .master_password("Mustbe8characters")
///             .master_username("foo")
///             .node_type("dc1.large")
///             .build_struct(),
///     );
///     let defaultSnapshotSchedule = snapshot_schedule::create(
///         "defaultSnapshotSchedule",
///         SnapshotScheduleArgs::builder()
///             .definitions(vec!["rate(12 hours)",])
///             .identifier("tf-redshift-snapshot-schedule")
///             .build_struct(),
///     );
///     let defaultSnapshotScheduleAssociation = snapshot_schedule_association::create(
///         "defaultSnapshotScheduleAssociation",
///         SnapshotScheduleAssociationArgs::builder()
///             .cluster_identifier("${default.id}")
///             .schedule_identifier("${defaultSnapshotSchedule.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Redshift Snapshot Schedule Association using the `<cluster-identifier>/<schedule-identifier>`. For example:
///
/// ```sh
/// $ pulumi import aws:redshift/snapshotScheduleAssociation:SnapshotScheduleAssociation default tf-redshift-cluster/tf-redshift-snapshot-schedule
/// ```
pub mod snapshot_schedule_association {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SnapshotScheduleAssociationArgs {
        /// The cluster identifier.
        #[builder(into)]
        pub cluster_identifier: pulumi_wasm_rust::Output<String>,
        /// The snapshot schedule identifier.
        #[builder(into)]
        pub schedule_identifier: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct SnapshotScheduleAssociationResult {
        /// The cluster identifier.
        pub cluster_identifier: pulumi_wasm_rust::Output<String>,
        /// The snapshot schedule identifier.
        pub schedule_identifier: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: SnapshotScheduleAssociationArgs,
    ) -> SnapshotScheduleAssociationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let cluster_identifier_binding = args.cluster_identifier.get_inner();
        let schedule_identifier_binding = args.schedule_identifier.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:redshift/snapshotScheduleAssociation:SnapshotScheduleAssociation"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "clusterIdentifier".into(),
                    value: &cluster_identifier_binding,
                },
                register_interface::ObjectField {
                    name: "scheduleIdentifier".into(),
                    value: &schedule_identifier_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "clusterIdentifier".into(),
                },
                register_interface::ResultField {
                    name: "scheduleIdentifier".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        SnapshotScheduleAssociationResult {
            cluster_identifier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clusterIdentifier").unwrap(),
            ),
            schedule_identifier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("scheduleIdentifier").unwrap(),
            ),
        }
    }
}
