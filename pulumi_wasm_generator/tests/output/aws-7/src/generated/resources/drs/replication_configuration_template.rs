/// Provides an Elastic Disaster Recovery replication configuration template resource. Before using DRS, your account must be [initialized](https://docs.aws.amazon.com/drs/latest/userguide/getting-started-initializing.html).
///
/// > **NOTE:** Your configuration must use the PIT policy shown in the basic configuration due to AWS rules. The only value that you can change is the `retention_duration` of `rule_id` 3.
///
/// ## Example Usage
///
/// ## Import
///
/// Using `pulumi import`, import DRS Replication Configuration Template using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:drs/replicationConfigurationTemplate:ReplicationConfigurationTemplate example templateid
/// ```
pub mod replication_configuration_template {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ReplicationConfigurationTemplateArgs {
        /// Whether to associate the default Elastic Disaster Recovery Security group with the Replication Configuration Template.
        #[builder(into)]
        pub associate_default_security_group: pulumi_wasm_rust::Output<bool>,
        /// Whether to allow the AWS replication agent to automatically replicate newly added disks.
        #[builder(into, default)]
        pub auto_replicate_new_disks: pulumi_wasm_rust::Output<Option<bool>>,
        /// Configure bandwidth throttling for the outbound data transfer rate of the Source Server in Mbps.
        #[builder(into)]
        pub bandwidth_throttling: pulumi_wasm_rust::Output<i32>,
        /// Whether to create a Public IP for the Recovery Instance by default.
        #[builder(into)]
        pub create_public_ip: pulumi_wasm_rust::Output<bool>,
        /// Data plane routing mechanism that will be used for replication. Valid values are `PUBLIC_IP` and `PRIVATE_IP`.
        #[builder(into)]
        pub data_plane_routing: pulumi_wasm_rust::Output<String>,
        /// Staging Disk EBS volume type to be used during replication. Valid values are `GP2`, `GP3`, `ST1`, or `AUTO`.
        #[builder(into)]
        pub default_large_staging_disk_type: pulumi_wasm_rust::Output<String>,
        /// Type of EBS encryption to be used during replication. Valid values are `DEFAULT` and `CUSTOM`.
        #[builder(into)]
        pub ebs_encryption: pulumi_wasm_rust::Output<String>,
        /// ARN of the EBS encryption key to be used during replication.
        #[builder(into, default)]
        pub ebs_encryption_key_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// Configuration block for Point in time (PIT) policy to manage snapshots taken during replication. See below.
        #[builder(into, default)]
        pub pit_policies: pulumi_wasm_rust::Output<
            Option<
                Vec<super::super::types::drs::ReplicationConfigurationTemplatePitPolicy>,
            >,
        >,
        /// Instance type to be used for the replication server.
        #[builder(into)]
        pub replication_server_instance_type: pulumi_wasm_rust::Output<String>,
        /// Security group IDs that will be used by the replication server.
        #[builder(into)]
        pub replication_servers_security_groups_ids: pulumi_wasm_rust::Output<
            Vec<String>,
        >,
        /// Subnet to be used by the replication staging area.
        #[builder(into)]
        pub staging_area_subnet_id: pulumi_wasm_rust::Output<String>,
        /// Set of tags to be associated with all resources created in the replication staging area: EC2 replication server, EBS volumes, EBS snapshots, etc.
        #[builder(into)]
        pub staging_area_tags: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Set of tags to be associated with the Replication Configuration Template resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        #[builder(into, default)]
        pub timeouts: pulumi_wasm_rust::Output<
            Option<super::super::types::drs::ReplicationConfigurationTemplateTimeouts>,
        >,
        /// Whether to use a dedicated Replication Server in the replication staging area.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub use_dedicated_replication_server: pulumi_wasm_rust::Output<bool>,
    }
    #[allow(dead_code)]
    pub struct ReplicationConfigurationTemplateResult {
        /// Replication configuration template ARN.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Whether to associate the default Elastic Disaster Recovery Security group with the Replication Configuration Template.
        pub associate_default_security_group: pulumi_wasm_rust::Output<bool>,
        /// Whether to allow the AWS replication agent to automatically replicate newly added disks.
        pub auto_replicate_new_disks: pulumi_wasm_rust::Output<bool>,
        /// Configure bandwidth throttling for the outbound data transfer rate of the Source Server in Mbps.
        pub bandwidth_throttling: pulumi_wasm_rust::Output<i32>,
        /// Whether to create a Public IP for the Recovery Instance by default.
        pub create_public_ip: pulumi_wasm_rust::Output<bool>,
        /// Data plane routing mechanism that will be used for replication. Valid values are `PUBLIC_IP` and `PRIVATE_IP`.
        pub data_plane_routing: pulumi_wasm_rust::Output<String>,
        /// Staging Disk EBS volume type to be used during replication. Valid values are `GP2`, `GP3`, `ST1`, or `AUTO`.
        pub default_large_staging_disk_type: pulumi_wasm_rust::Output<String>,
        /// Type of EBS encryption to be used during replication. Valid values are `DEFAULT` and `CUSTOM`.
        pub ebs_encryption: pulumi_wasm_rust::Output<String>,
        /// ARN of the EBS encryption key to be used during replication.
        pub ebs_encryption_key_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// Configuration block for Point in time (PIT) policy to manage snapshots taken during replication. See below.
        pub pit_policies: pulumi_wasm_rust::Output<
            Option<
                Vec<super::super::types::drs::ReplicationConfigurationTemplatePitPolicy>,
            >,
        >,
        /// Instance type to be used for the replication server.
        pub replication_server_instance_type: pulumi_wasm_rust::Output<String>,
        /// Security group IDs that will be used by the replication server.
        pub replication_servers_security_groups_ids: pulumi_wasm_rust::Output<
            Vec<String>,
        >,
        /// Subnet to be used by the replication staging area.
        pub staging_area_subnet_id: pulumi_wasm_rust::Output<String>,
        /// Set of tags to be associated with all resources created in the replication staging area: EC2 replication server, EBS volumes, EBS snapshots, etc.
        pub staging_area_tags: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Set of tags to be associated with the Replication Configuration Template resource.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub timeouts: pulumi_wasm_rust::Output<
            Option<super::super::types::drs::ReplicationConfigurationTemplateTimeouts>,
        >,
        /// Whether to use a dedicated Replication Server in the replication staging area.
        ///
        /// The following arguments are optional:
        pub use_dedicated_replication_server: pulumi_wasm_rust::Output<bool>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: ReplicationConfigurationTemplateArgs,
    ) -> ReplicationConfigurationTemplateResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let associate_default_security_group_binding = args
            .associate_default_security_group
            .get_inner();
        let auto_replicate_new_disks_binding = args.auto_replicate_new_disks.get_inner();
        let bandwidth_throttling_binding = args.bandwidth_throttling.get_inner();
        let create_public_ip_binding = args.create_public_ip.get_inner();
        let data_plane_routing_binding = args.data_plane_routing.get_inner();
        let default_large_staging_disk_type_binding = args
            .default_large_staging_disk_type
            .get_inner();
        let ebs_encryption_binding = args.ebs_encryption.get_inner();
        let ebs_encryption_key_arn_binding = args.ebs_encryption_key_arn.get_inner();
        let pit_policies_binding = args.pit_policies.get_inner();
        let replication_server_instance_type_binding = args
            .replication_server_instance_type
            .get_inner();
        let replication_servers_security_groups_ids_binding = args
            .replication_servers_security_groups_ids
            .get_inner();
        let staging_area_subnet_id_binding = args.staging_area_subnet_id.get_inner();
        let staging_area_tags_binding = args.staging_area_tags.get_inner();
        let tags_binding = args.tags.get_inner();
        let timeouts_binding = args.timeouts.get_inner();
        let use_dedicated_replication_server_binding = args
            .use_dedicated_replication_server
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:drs/replicationConfigurationTemplate:ReplicationConfigurationTemplate"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "associateDefaultSecurityGroup".into(),
                    value: &associate_default_security_group_binding,
                },
                register_interface::ObjectField {
                    name: "autoReplicateNewDisks".into(),
                    value: &auto_replicate_new_disks_binding,
                },
                register_interface::ObjectField {
                    name: "bandwidthThrottling".into(),
                    value: &bandwidth_throttling_binding,
                },
                register_interface::ObjectField {
                    name: "createPublicIp".into(),
                    value: &create_public_ip_binding,
                },
                register_interface::ObjectField {
                    name: "dataPlaneRouting".into(),
                    value: &data_plane_routing_binding,
                },
                register_interface::ObjectField {
                    name: "defaultLargeStagingDiskType".into(),
                    value: &default_large_staging_disk_type_binding,
                },
                register_interface::ObjectField {
                    name: "ebsEncryption".into(),
                    value: &ebs_encryption_binding,
                },
                register_interface::ObjectField {
                    name: "ebsEncryptionKeyArn".into(),
                    value: &ebs_encryption_key_arn_binding,
                },
                register_interface::ObjectField {
                    name: "pitPolicies".into(),
                    value: &pit_policies_binding,
                },
                register_interface::ObjectField {
                    name: "replicationServerInstanceType".into(),
                    value: &replication_server_instance_type_binding,
                },
                register_interface::ObjectField {
                    name: "replicationServersSecurityGroupsIds".into(),
                    value: &replication_servers_security_groups_ids_binding,
                },
                register_interface::ObjectField {
                    name: "stagingAreaSubnetId".into(),
                    value: &staging_area_subnet_id_binding,
                },
                register_interface::ObjectField {
                    name: "stagingAreaTags".into(),
                    value: &staging_area_tags_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "timeouts".into(),
                    value: &timeouts_binding,
                },
                register_interface::ObjectField {
                    name: "useDedicatedReplicationServer".into(),
                    value: &use_dedicated_replication_server_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "associateDefaultSecurityGroup".into(),
                },
                register_interface::ResultField {
                    name: "autoReplicateNewDisks".into(),
                },
                register_interface::ResultField {
                    name: "bandwidthThrottling".into(),
                },
                register_interface::ResultField {
                    name: "createPublicIp".into(),
                },
                register_interface::ResultField {
                    name: "dataPlaneRouting".into(),
                },
                register_interface::ResultField {
                    name: "defaultLargeStagingDiskType".into(),
                },
                register_interface::ResultField {
                    name: "ebsEncryption".into(),
                },
                register_interface::ResultField {
                    name: "ebsEncryptionKeyArn".into(),
                },
                register_interface::ResultField {
                    name: "pitPolicies".into(),
                },
                register_interface::ResultField {
                    name: "replicationServerInstanceType".into(),
                },
                register_interface::ResultField {
                    name: "replicationServersSecurityGroupsIds".into(),
                },
                register_interface::ResultField {
                    name: "stagingAreaSubnetId".into(),
                },
                register_interface::ResultField {
                    name: "stagingAreaTags".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "timeouts".into(),
                },
                register_interface::ResultField {
                    name: "useDedicatedReplicationServer".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ReplicationConfigurationTemplateResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            associate_default_security_group: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("associateDefaultSecurityGroup").unwrap(),
            ),
            auto_replicate_new_disks: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("autoReplicateNewDisks").unwrap(),
            ),
            bandwidth_throttling: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bandwidthThrottling").unwrap(),
            ),
            create_public_ip: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createPublicIp").unwrap(),
            ),
            data_plane_routing: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dataPlaneRouting").unwrap(),
            ),
            default_large_staging_disk_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultLargeStagingDiskType").unwrap(),
            ),
            ebs_encryption: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ebsEncryption").unwrap(),
            ),
            ebs_encryption_key_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ebsEncryptionKeyArn").unwrap(),
            ),
            pit_policies: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pitPolicies").unwrap(),
            ),
            replication_server_instance_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("replicationServerInstanceType").unwrap(),
            ),
            replication_servers_security_groups_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("replicationServersSecurityGroupsIds").unwrap(),
            ),
            staging_area_subnet_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("stagingAreaSubnetId").unwrap(),
            ),
            staging_area_tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("stagingAreaTags").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            timeouts: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("timeouts").unwrap(),
            ),
            use_dedicated_replication_server: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("useDedicatedReplicationServer").unwrap(),
            ),
        }
    }
}
