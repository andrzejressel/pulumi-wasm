/// Creates a Redshift cluster snapshot
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:redshift:ClusterSnapshot
///     properties:
///       clusterSnapshotName: example
///       clusterSnapshotContent:
///         fn::toJSON:
///           AllowDBUserOverride: '1'
///           Client_ID: ExampleClientID
///           App_ID: example
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Redshift Cluster Snapshots using `snapshot_identifier`. For example:
///
/// ```sh
/// $ pulumi import aws:redshift/clusterSnapshot:ClusterSnapshot test example
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod cluster_snapshot {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ClusterSnapshotArgs {
        /// The cluster identifier for which you want a snapshot.
        #[builder(into)]
        pub cluster_identifier: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The number of days that a manual snapshot is retained. If the value is `-1`, the manual snapshot is retained indefinitely. Valid values are -1 and between `1` and `3653`.
        #[builder(into, default)]
        pub manual_snapshot_retention_period: pulumi_gestalt_rust::InputOrOutput<
            Option<i32>,
        >,
        /// A unique identifier for the snapshot that you are requesting. This identifier must be unique for all snapshots within the Amazon Web Services account.
        #[builder(into)]
        pub snapshot_identifier: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ClusterSnapshotResult {
        /// Amazon Resource Name (ARN) of the snapshot.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The cluster identifier for which you want a snapshot.
        pub cluster_identifier: pulumi_gestalt_rust::Output<String>,
        /// The Key Management Service (KMS) key ID of the encryption key that was used to encrypt data in the cluster from which the snapshot was taken.
        pub kms_key_id: pulumi_gestalt_rust::Output<String>,
        /// The number of days that a manual snapshot is retained. If the value is `-1`, the manual snapshot is retained indefinitely. Valid values are -1 and between `1` and `3653`.
        pub manual_snapshot_retention_period: pulumi_gestalt_rust::Output<Option<i32>>,
        /// For manual snapshots, the Amazon Web Services account used to create or copy the snapshot. For automatic snapshots, the owner of the cluster. The owner can perform all snapshot actions, such as sharing a manual snapshot.
        pub owner_account: pulumi_gestalt_rust::Output<String>,
        /// A unique identifier for the snapshot that you are requesting. This identifier must be unique for all snapshots within the Amazon Web Services account.
        pub snapshot_identifier: pulumi_gestalt_rust::Output<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ClusterSnapshotArgs,
    ) -> ClusterSnapshotResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let cluster_identifier_binding_1 = args.cluster_identifier.get_output(context);
        let cluster_identifier_binding = cluster_identifier_binding_1.get_inner();
        let manual_snapshot_retention_period_binding_1 = args
            .manual_snapshot_retention_period
            .get_output(context);
        let manual_snapshot_retention_period_binding = manual_snapshot_retention_period_binding_1
            .get_inner();
        let snapshot_identifier_binding_1 = args.snapshot_identifier.get_output(context);
        let snapshot_identifier_binding = snapshot_identifier_binding_1.get_inner();
        let tags_binding_1 = args.tags.get_output(context);
        let tags_binding = tags_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:redshift/clusterSnapshot:ClusterSnapshot".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "clusterIdentifier".into(),
                    value: &cluster_identifier_binding,
                },
                register_interface::ObjectField {
                    name: "manualSnapshotRetentionPeriod".into(),
                    value: &manual_snapshot_retention_period_binding,
                },
                register_interface::ObjectField {
                    name: "snapshotIdentifier".into(),
                    value: &snapshot_identifier_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ClusterSnapshotResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            cluster_identifier: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("clusterIdentifier"),
            ),
            kms_key_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("kmsKeyId"),
            ),
            manual_snapshot_retention_period: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("manualSnapshotRetentionPeriod"),
            ),
            owner_account: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ownerAccount"),
            ),
            snapshot_identifier: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("snapshotIdentifier"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
        }
    }
}
