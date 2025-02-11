/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod snapshot_schedule_association {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SnapshotScheduleAssociationArgs {
        /// The cluster identifier.
        #[builder(into)]
        pub cluster_identifier: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The snapshot schedule identifier.
        #[builder(into)]
        pub schedule_identifier: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct SnapshotScheduleAssociationResult {
        /// The cluster identifier.
        pub cluster_identifier: pulumi_gestalt_rust::Output<String>,
        /// The snapshot schedule identifier.
        pub schedule_identifier: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SnapshotScheduleAssociationArgs,
    ) -> SnapshotScheduleAssociationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let cluster_identifier_binding = args.cluster_identifier.get_output(context);
        let schedule_identifier_binding = args.schedule_identifier.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:redshift/snapshotScheduleAssociation:SnapshotScheduleAssociation"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clusterIdentifier".into(),
                    value: &cluster_identifier_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "scheduleIdentifier".into(),
                    value: &schedule_identifier_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        SnapshotScheduleAssociationResult {
            cluster_identifier: o.get_field("clusterIdentifier"),
            schedule_identifier: o.get_field("scheduleIdentifier"),
        }
    }
}
