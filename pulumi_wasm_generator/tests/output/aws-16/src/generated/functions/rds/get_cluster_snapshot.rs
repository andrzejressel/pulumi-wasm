pub mod get_cluster_snapshot {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetClusterSnapshotArgs {
        /// Returns the list of snapshots created by the specific db_cluster
        #[builder(into, default)]
        pub db_cluster_identifier: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Returns information on a specific snapshot_id.
        #[builder(into, default)]
        pub db_cluster_snapshot_identifier: pulumi_wasm_rust::InputOrOutput<
            Option<String>,
        >,
        /// Set this value to true to include manual DB Cluster Snapshots that are public and can be
        /// copied or restored by any AWS account, otherwise set this value to false. The default is `false`.
        #[builder(into, default)]
        pub include_public: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Set this value to true to include shared manual DB Cluster Snapshots from other
        /// AWS accounts that this AWS account has been given permission to copy or restore, otherwise set this value to false.
        /// The default is `false`.
        #[builder(into, default)]
        pub include_shared: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// If more than one result is returned, use the most recent Snapshot.
        #[builder(into, default)]
        pub most_recent: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Type of snapshots to be returned. If you don't specify a SnapshotType
        /// value, then both automated and manual DB cluster snapshots are returned. Shared and public DB Cluster Snapshots are not
        /// included in the returned results by default. Possible values are, `automated`, `manual`, `shared`, `public` and `awsbackup`.
        #[builder(into, default)]
        pub snapshot_type: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Mapping of tags, each pair of which must exactly match
        /// a pair on the desired DB cluster snapshot.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetClusterSnapshotResult {
        /// Allocated storage size in gigabytes (GB).
        pub allocated_storage: pulumi_wasm_rust::Output<i32>,
        /// List of EC2 Availability Zones that instances in the DB cluster snapshot can be restored in.
        pub availability_zones: pulumi_wasm_rust::Output<Vec<String>>,
        /// Specifies the DB cluster identifier of the DB cluster that this DB cluster snapshot was created from.
        pub db_cluster_identifier: pulumi_wasm_rust::Output<Option<String>>,
        /// The ARN for the DB Cluster Snapshot.
        pub db_cluster_snapshot_arn: pulumi_wasm_rust::Output<String>,
        pub db_cluster_snapshot_identifier: pulumi_wasm_rust::Output<Option<String>>,
        /// Name of the database engine.
        pub engine: pulumi_wasm_rust::Output<String>,
        /// Version of the database engine for this DB cluster snapshot.
        pub engine_version: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub include_public: pulumi_wasm_rust::Output<Option<bool>>,
        pub include_shared: pulumi_wasm_rust::Output<Option<bool>>,
        /// If storage_encrypted is true, the AWS KMS key identifier for the encrypted DB cluster snapshot.
        pub kms_key_id: pulumi_wasm_rust::Output<String>,
        /// License model information for the restored DB cluster.
        pub license_model: pulumi_wasm_rust::Output<String>,
        pub most_recent: pulumi_wasm_rust::Output<Option<bool>>,
        /// Port that the DB cluster was listening on at the time of the snapshot.
        pub port: pulumi_wasm_rust::Output<i32>,
        /// Time when the snapshot was taken, in Universal Coordinated Time (UTC).
        pub snapshot_create_time: pulumi_wasm_rust::Output<String>,
        pub snapshot_type: pulumi_wasm_rust::Output<Option<String>>,
        pub source_db_cluster_snapshot_arn: pulumi_wasm_rust::Output<String>,
        /// Status of this DB Cluster Snapshot.
        pub status: pulumi_wasm_rust::Output<String>,
        /// Whether the DB cluster snapshot is encrypted.
        pub storage_encrypted: pulumi_wasm_rust::Output<bool>,
        /// Map of tags for the resource.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// VPC ID associated with the DB cluster snapshot.
        pub vpc_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetClusterSnapshotArgs,
    ) -> GetClusterSnapshotResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let db_cluster_identifier_binding = args
            .db_cluster_identifier
            .get_output(context)
            .get_inner();
        let db_cluster_snapshot_identifier_binding = args
            .db_cluster_snapshot_identifier
            .get_output(context)
            .get_inner();
        let include_public_binding = args.include_public.get_output(context).get_inner();
        let include_shared_binding = args.include_shared.get_output(context).get_inner();
        let most_recent_binding = args.most_recent.get_output(context).get_inner();
        let snapshot_type_binding = args.snapshot_type.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:rds/getClusterSnapshot:getClusterSnapshot".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "dbClusterIdentifier".into(),
                    value: &db_cluster_identifier_binding,
                },
                register_interface::ObjectField {
                    name: "dbClusterSnapshotIdentifier".into(),
                    value: &db_cluster_snapshot_identifier_binding,
                },
                register_interface::ObjectField {
                    name: "includePublic".into(),
                    value: &include_public_binding,
                },
                register_interface::ObjectField {
                    name: "includeShared".into(),
                    value: &include_shared_binding,
                },
                register_interface::ObjectField {
                    name: "mostRecent".into(),
                    value: &most_recent_binding,
                },
                register_interface::ObjectField {
                    name: "snapshotType".into(),
                    value: &snapshot_type_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "allocatedStorage".into(),
                },
                register_interface::ResultField {
                    name: "availabilityZones".into(),
                },
                register_interface::ResultField {
                    name: "dbClusterIdentifier".into(),
                },
                register_interface::ResultField {
                    name: "dbClusterSnapshotArn".into(),
                },
                register_interface::ResultField {
                    name: "dbClusterSnapshotIdentifier".into(),
                },
                register_interface::ResultField {
                    name: "engine".into(),
                },
                register_interface::ResultField {
                    name: "engineVersion".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "includePublic".into(),
                },
                register_interface::ResultField {
                    name: "includeShared".into(),
                },
                register_interface::ResultField {
                    name: "kmsKeyId".into(),
                },
                register_interface::ResultField {
                    name: "licenseModel".into(),
                },
                register_interface::ResultField {
                    name: "mostRecent".into(),
                },
                register_interface::ResultField {
                    name: "port".into(),
                },
                register_interface::ResultField {
                    name: "snapshotCreateTime".into(),
                },
                register_interface::ResultField {
                    name: "snapshotType".into(),
                },
                register_interface::ResultField {
                    name: "sourceDbClusterSnapshotArn".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
                register_interface::ResultField {
                    name: "storageEncrypted".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "vpcId".into(),
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetClusterSnapshotResult {
            allocated_storage: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("allocatedStorage").unwrap(),
            ),
            availability_zones: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("availabilityZones").unwrap(),
            ),
            db_cluster_identifier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dbClusterIdentifier").unwrap(),
            ),
            db_cluster_snapshot_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dbClusterSnapshotArn").unwrap(),
            ),
            db_cluster_snapshot_identifier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dbClusterSnapshotIdentifier").unwrap(),
            ),
            engine: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("engine").unwrap(),
            ),
            engine_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("engineVersion").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            include_public: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("includePublic").unwrap(),
            ),
            include_shared: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("includeShared").unwrap(),
            ),
            kms_key_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kmsKeyId").unwrap(),
            ),
            license_model: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("licenseModel").unwrap(),
            ),
            most_recent: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("mostRecent").unwrap(),
            ),
            port: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("port").unwrap(),
            ),
            snapshot_create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("snapshotCreateTime").unwrap(),
            ),
            snapshot_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("snapshotType").unwrap(),
            ),
            source_db_cluster_snapshot_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceDbClusterSnapshotArn").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
            storage_encrypted: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageEncrypted").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            vpc_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpcId").unwrap(),
            ),
        }
    }
}
