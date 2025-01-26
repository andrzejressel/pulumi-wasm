pub mod get_snapshot {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetSnapshotArgs {
        /// Returns the list of snapshots created by the specific db_instance
        #[builder(into, default)]
        pub db_instance_identifier: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Returns information on a specific snapshot_id.
        #[builder(into, default)]
        pub db_snapshot_identifier: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Set this value to true to include manual DB snapshots that are public and can be
        /// copied or restored by any AWS account, otherwise set this value to false. The default is `false`.
        #[builder(into, default)]
        pub include_public: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Set this value to true to include shared manual DB snapshots from other
        /// AWS accounts that this AWS account has been given permission to copy or restore, otherwise set this value to false.
        /// The default is `false`.
        #[builder(into, default)]
        pub include_shared: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// If more than one result is returned, use the most
        /// recent Snapshot.
        #[builder(into, default)]
        pub most_recent: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Type of snapshots to be returned. If you don't specify a SnapshotType
        /// value, then both automated and manual snapshots are returned. Shared and public DB snapshots are not
        /// included in the returned results by default. Possible values are, `automated`, `manual`, `shared`, `public` and `awsbackup`.
        #[builder(into, default)]
        pub snapshot_type: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Mapping of tags, each pair of which must exactly match
        /// a pair on the desired DB snapshot.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetSnapshotResult {
        /// Allocated storage size in gigabytes (GB).
        pub allocated_storage: pulumi_wasm_rust::Output<i32>,
        /// Name of the Availability Zone the DB instance was located in at the time of the DB snapshot.
        pub availability_zone: pulumi_wasm_rust::Output<String>,
        pub db_instance_identifier: pulumi_wasm_rust::Output<Option<String>>,
        /// ARN for the DB snapshot.
        pub db_snapshot_arn: pulumi_wasm_rust::Output<String>,
        pub db_snapshot_identifier: pulumi_wasm_rust::Output<Option<String>>,
        /// Whether the DB snapshot is encrypted.
        pub encrypted: pulumi_wasm_rust::Output<bool>,
        /// Name of the database engine.
        pub engine: pulumi_wasm_rust::Output<String>,
        /// Version of the database engine.
        pub engine_version: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub include_public: pulumi_wasm_rust::Output<Option<bool>>,
        pub include_shared: pulumi_wasm_rust::Output<Option<bool>>,
        /// Provisioned IOPS (I/O operations per second) value of the DB instance at the time of the snapshot.
        pub iops: pulumi_wasm_rust::Output<i32>,
        /// ARN for the KMS encryption key.
        pub kms_key_id: pulumi_wasm_rust::Output<String>,
        /// License model information for the restored DB instance.
        pub license_model: pulumi_wasm_rust::Output<String>,
        pub most_recent: pulumi_wasm_rust::Output<Option<bool>>,
        /// Provides the option group name for the DB snapshot.
        pub option_group_name: pulumi_wasm_rust::Output<String>,
        /// Provides the time when the snapshot was taken, in Universal Coordinated Time (UTC). Doesn't change when the snapshot is copied.
        pub original_snapshot_create_time: pulumi_wasm_rust::Output<String>,
        pub port: pulumi_wasm_rust::Output<i32>,
        /// Provides the time when the snapshot was taken, in Universal Coordinated Time (UTC). Changes for the copy when the snapshot is copied.
        pub snapshot_create_time: pulumi_wasm_rust::Output<String>,
        pub snapshot_type: pulumi_wasm_rust::Output<Option<String>>,
        /// DB snapshot ARN that the DB snapshot was copied from. It only has value in case of cross customer or cross region copy.
        pub source_db_snapshot_identifier: pulumi_wasm_rust::Output<String>,
        /// Region that the DB snapshot was created in or copied from.
        pub source_region: pulumi_wasm_rust::Output<String>,
        /// Status of this DB snapshot.
        pub status: pulumi_wasm_rust::Output<String>,
        /// Storage type associated with DB snapshot.
        pub storage_type: pulumi_wasm_rust::Output<String>,
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// ID of the VPC associated with the DB snapshot.
        pub vpc_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetSnapshotArgs,
    ) -> GetSnapshotResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let db_instance_identifier_binding = args
            .db_instance_identifier
            .get_output(context)
            .get_inner();
        let db_snapshot_identifier_binding = args
            .db_snapshot_identifier
            .get_output(context)
            .get_inner();
        let include_public_binding = args.include_public.get_output(context).get_inner();
        let include_shared_binding = args.include_shared.get_output(context).get_inner();
        let most_recent_binding = args.most_recent.get_output(context).get_inner();
        let snapshot_type_binding = args.snapshot_type.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:rds/getSnapshot:getSnapshot".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "dbInstanceIdentifier".into(),
                    value: &db_instance_identifier_binding,
                },
                register_interface::ObjectField {
                    name: "dbSnapshotIdentifier".into(),
                    value: &db_snapshot_identifier_binding,
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
                    name: "availabilityZone".into(),
                },
                register_interface::ResultField {
                    name: "dbInstanceIdentifier".into(),
                },
                register_interface::ResultField {
                    name: "dbSnapshotArn".into(),
                },
                register_interface::ResultField {
                    name: "dbSnapshotIdentifier".into(),
                },
                register_interface::ResultField {
                    name: "encrypted".into(),
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
                    name: "iops".into(),
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
                    name: "optionGroupName".into(),
                },
                register_interface::ResultField {
                    name: "originalSnapshotCreateTime".into(),
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
                    name: "sourceDbSnapshotIdentifier".into(),
                },
                register_interface::ResultField {
                    name: "sourceRegion".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
                register_interface::ResultField {
                    name: "storageType".into(),
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
        GetSnapshotResult {
            allocated_storage: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("allocatedStorage").unwrap(),
            ),
            availability_zone: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("availabilityZone").unwrap(),
            ),
            db_instance_identifier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dbInstanceIdentifier").unwrap(),
            ),
            db_snapshot_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dbSnapshotArn").unwrap(),
            ),
            db_snapshot_identifier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dbSnapshotIdentifier").unwrap(),
            ),
            encrypted: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("encrypted").unwrap(),
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
            iops: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("iops").unwrap(),
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
            option_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("optionGroupName").unwrap(),
            ),
            original_snapshot_create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("originalSnapshotCreateTime").unwrap(),
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
            source_db_snapshot_identifier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceDbSnapshotIdentifier").unwrap(),
            ),
            source_region: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceRegion").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
            storage_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageType").unwrap(),
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
