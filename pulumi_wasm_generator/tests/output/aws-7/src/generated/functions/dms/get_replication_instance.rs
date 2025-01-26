pub mod get_replication_instance {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetReplicationInstanceArgs {
        /// The replication instance identifier.
        #[builder(into)]
        pub replication_instance_id: pulumi_wasm_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetReplicationInstanceResult {
        /// The amount of storage (in gigabytes) to be initially allocated for the replication instance.
        pub allocated_storage: pulumi_wasm_rust::Output<i32>,
        /// Indicates that minor engine upgrades will be applied automatically to the replication instance during the maintenance window.
        pub auto_minor_version_upgrade: pulumi_wasm_rust::Output<bool>,
        /// The EC2 Availability Zone that the replication instance will be created in.
        pub availability_zone: pulumi_wasm_rust::Output<String>,
        /// The engine version number of the replication instance.
        pub engine_version: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The Amazon Resource Name (ARN) for the KMS key used to encrypt the connection parameters.
        pub kms_key_arn: pulumi_wasm_rust::Output<String>,
        /// Specifies if the replication instance is a multi-az deployment.
        pub multi_az: pulumi_wasm_rust::Output<bool>,
        /// The type of IP address protocol used by the replication instance.
        pub network_type: pulumi_wasm_rust::Output<String>,
        /// The weekly time range during which system maintenance can occur, in Universal Coordinated Time (UTC).
        pub preferred_maintenance_window: pulumi_wasm_rust::Output<String>,
        /// Specifies the accessibility options for the replication instance. A value of true represents an instance with a public IP address. A value of false represents an instance with a private IP address.
        pub publicly_accessible: pulumi_wasm_rust::Output<bool>,
        /// The Amazon Resource Name (ARN) of the replication instance.
        pub replication_instance_arn: pulumi_wasm_rust::Output<String>,
        /// The compute and memory capacity of the replication instance as specified by the replication instance class. See [AWS DMS User Guide](https://docs.aws.amazon.com/dms/latest/userguide/CHAP_ReplicationInstance.Types.html) for information on instance classes.
        pub replication_instance_class: pulumi_wasm_rust::Output<String>,
        pub replication_instance_id: pulumi_wasm_rust::Output<String>,
        /// A list of the private IP addresses of the replication instance.
        pub replication_instance_private_ips: pulumi_wasm_rust::Output<Vec<String>>,
        /// A list of the public IP addresses of the replication instance.
        pub replication_instance_public_ips: pulumi_wasm_rust::Output<Vec<String>>,
        /// A subnet group to associate with the replication instance.
        pub replication_subnet_group_id: pulumi_wasm_rust::Output<String>,
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// A set of VPC security group IDs that are used with the replication instance.
        pub vpc_security_group_ids: pulumi_wasm_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetReplicationInstanceArgs,
    ) -> GetReplicationInstanceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let replication_instance_id_binding = args
            .replication_instance_id
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:dms/getReplicationInstance:getReplicationInstance".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "replicationInstanceId".into(),
                    value: &replication_instance_id_binding,
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
                    name: "autoMinorVersionUpgrade".into(),
                },
                register_interface::ResultField {
                    name: "availabilityZone".into(),
                },
                register_interface::ResultField {
                    name: "engineVersion".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "kmsKeyArn".into(),
                },
                register_interface::ResultField {
                    name: "multiAz".into(),
                },
                register_interface::ResultField {
                    name: "networkType".into(),
                },
                register_interface::ResultField {
                    name: "preferredMaintenanceWindow".into(),
                },
                register_interface::ResultField {
                    name: "publiclyAccessible".into(),
                },
                register_interface::ResultField {
                    name: "replicationInstanceArn".into(),
                },
                register_interface::ResultField {
                    name: "replicationInstanceClass".into(),
                },
                register_interface::ResultField {
                    name: "replicationInstanceId".into(),
                },
                register_interface::ResultField {
                    name: "replicationInstancePrivateIps".into(),
                },
                register_interface::ResultField {
                    name: "replicationInstancePublicIps".into(),
                },
                register_interface::ResultField {
                    name: "replicationSubnetGroupId".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "vpcSecurityGroupIds".into(),
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetReplicationInstanceResult {
            allocated_storage: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("allocatedStorage").unwrap(),
            ),
            auto_minor_version_upgrade: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("autoMinorVersionUpgrade").unwrap(),
            ),
            availability_zone: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("availabilityZone").unwrap(),
            ),
            engine_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("engineVersion").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            kms_key_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kmsKeyArn").unwrap(),
            ),
            multi_az: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("multiAz").unwrap(),
            ),
            network_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("networkType").unwrap(),
            ),
            preferred_maintenance_window: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("preferredMaintenanceWindow").unwrap(),
            ),
            publicly_accessible: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("publiclyAccessible").unwrap(),
            ),
            replication_instance_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("replicationInstanceArn").unwrap(),
            ),
            replication_instance_class: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("replicationInstanceClass").unwrap(),
            ),
            replication_instance_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("replicationInstanceId").unwrap(),
            ),
            replication_instance_private_ips: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("replicationInstancePrivateIps").unwrap(),
            ),
            replication_instance_public_ips: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("replicationInstancePublicIps").unwrap(),
            ),
            replication_subnet_group_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("replicationSubnetGroupId").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            vpc_security_group_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpcSecurityGroupIds").unwrap(),
            ),
        }
    }
}
