/// Provides an OpsWorks instance resource.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   my-instance:
///     type: aws:opsworks:Instance
///     properties:
///       stackId: ${main.id}
///       layerIds:
///         - ${["my-layer"].id}
///       instanceType: t2.micro
///       os: Amazon Linux 2015.09
///       state: stopped
/// ```
///
/// ## Block devices
///
/// Each of the `*_block_device` attributes controls a portion of the AWS
/// Instance's "Block Device Mapping". It's a good idea to familiarize yourself with [AWS's Block Device
/// Mapping docs](http://docs.aws.amazon.com/AWSEC2/latest/UserGuide/block-device-mapping-concepts.html)
/// to understand the implications of using these attributes.
///
/// ### `ebs_block_device`
///
/// * `delete_on_termination` - (Optional) Whether the volume should be destroyed on instance termination. Default is `true`.
/// * `device_name` - (Required) Name of the device to mount.
/// * `iops` - (Optional) Amount of provisioned [IOPS](http://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ebs-io-characteristics.html). This must be set with a `volume_type` of `io1`.
/// * `snapshot_id` - (Optional) Snapshot ID to mount.
/// * `volume_size` - (Optional) Size of the volume in gigabytes.
/// * `volume_type` - (Optional) Type of volume. Valid values are `standard`, `gp2`, or `io1`. Default is `standard`.
///
/// Modifying any `ebs_block_device` currently requires resource replacement.
///
/// ### `ephemeral_block_device`
///
/// * `device_name` - Name of the block device to mount on the instance.
/// * `virtual_name` - The [Instance Store Device Name](http://docs.aws.amazon.com/AWSEC2/latest/UserGuide/InstanceStorage.html#InstanceStoreDeviceNames) (e.g., `ephemeral0`).
///
/// Each AWS Instance type has a different set of Instance Store block devices
/// available for attachment. AWS [publishes a
/// list](http://docs.aws.amazon.com/AWSEC2/latest/UserGuide/InstanceStorage.html#StorageOnInstanceTypes)
/// of which ephemeral devices are available on each type. The devices are always
/// identified by the `virtual_name` in the format `ephemeral{0..N}`.
///
/// ### `root_block_device`
///
/// * `delete_on_termination` - (Optional) Whether the volume should be destroyed on instance termination. Default is `true`.
/// * `iops` - (Optional) Amount of provisioned [IOPS](http://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ebs-io-characteristics.html). This must be set with a `volume_type` of `io1`.
/// * `volume_size` - (Optional) Size of the volume in gigabytes.
/// * `volume_type` - (Optional) Type of volume. Valid values are `standard`, `gp2`, or `io1`. Default is `standard`.
///
/// Modifying any of the `root_block_device` settings requires resource
/// replacement.
///
/// > **NOTE:** Currently, changes to `*_block_device` configuration of _existing_
/// resources cannot be automatically detected by this provider. After making updates
/// to block device configuration, resource recreation can be manually triggered by
/// using the [`up` command with the --replace argument](https://www.pulumi.com/docs/reference/cli/pulumi_up/).
///
/// ## Import
///
/// Using `pulumi import`, import Opsworks Instances using the instance `id`. For example:
///
/// ```sh
/// $ pulumi import aws:opsworks/instance:Instance my_instance 4d6d1710-ded9-42a1-b08e-b043ad7af1e2
/// ```
pub mod instance {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct InstanceArgs {
        /// OpsWorks agent to install. Default is `INHERIT`.
        #[builder(into, default)]
        pub agent_version: pulumi_wasm_rust::Output<Option<String>>,
        /// AMI to use for the instance.  If an AMI is specified, `os` must be `Custom`.
        #[builder(into, default)]
        pub ami_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Machine architecture for created instances.  Valid values are `x86_64` or `i386`. The default is `x86_64`.
        #[builder(into, default)]
        pub architecture: pulumi_wasm_rust::Output<Option<String>>,
        /// Creates load-based or time-based instances.  Valid values are `load`, `timer`.
        #[builder(into, default)]
        pub auto_scaling_type: pulumi_wasm_rust::Output<Option<String>>,
        /// Name of the availability zone where instances will be created by default.
        #[builder(into, default)]
        pub availability_zone: pulumi_wasm_rust::Output<Option<String>>,
        /// Time that the instance was created.
        #[builder(into, default)]
        pub created_at: pulumi_wasm_rust::Output<Option<String>>,
        /// Whether to delete EBS volume on deletion. Default is `true`.
        #[builder(into, default)]
        pub delete_ebs: pulumi_wasm_rust::Output<Option<bool>>,
        /// Whether to delete the Elastic IP on deletion.
        #[builder(into, default)]
        pub delete_eip: pulumi_wasm_rust::Output<Option<bool>>,
        /// Configuration block for additional EBS block devices to attach to the instance. See Block Devices below.
        #[builder(into, default)]
        pub ebs_block_devices: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::opsworks::InstanceEbsBlockDevice>>,
        >,
        /// Whether the launched EC2 instance will be EBS-optimized.
        #[builder(into, default)]
        pub ebs_optimized: pulumi_wasm_rust::Output<Option<bool>>,
        /// ECS cluster's ARN for container instances.
        #[builder(into, default)]
        pub ecs_cluster_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// Instance Elastic IP address.
        #[builder(into, default)]
        pub elastic_ip: pulumi_wasm_rust::Output<Option<String>>,
        /// Configuration block for ephemeral (also known as "Instance Store") volumes on the instance. See Block Devices below.
        #[builder(into, default)]
        pub ephemeral_block_devices: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::opsworks::InstanceEphemeralBlockDevice>>,
        >,
        /// Instance's host name.
        #[builder(into, default)]
        pub hostname: pulumi_wasm_rust::Output<Option<String>>,
        /// For registered instances, infrastructure class: ec2 or on-premises.
        #[builder(into, default)]
        pub infrastructure_class: pulumi_wasm_rust::Output<Option<String>>,
        /// Controls where to install OS and package updates when the instance boots.  Default is `true`.
        #[builder(into, default)]
        pub install_updates_on_boot: pulumi_wasm_rust::Output<Option<bool>>,
        /// ARN of the instance's IAM profile.
        #[builder(into, default)]
        pub instance_profile_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// Type of instance to start.
        #[builder(into, default)]
        pub instance_type: pulumi_wasm_rust::Output<Option<String>>,
        /// List of the layers the instance will belong to.
        #[builder(into)]
        pub layer_ids: pulumi_wasm_rust::Output<Vec<String>>,
        /// Name of operating system that will be installed.
        #[builder(into, default)]
        pub os: pulumi_wasm_rust::Output<Option<String>>,
        /// Configuration block for the root block device of the instance. See Block Devices below.
        #[builder(into, default)]
        pub root_block_devices: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::opsworks::InstanceRootBlockDevice>>,
        >,
        /// Name of the type of root device instances will have by default. Valid values are `ebs` or `instance-store`.
        #[builder(into, default)]
        pub root_device_type: pulumi_wasm_rust::Output<Option<String>>,
        /// Associated security groups.
        #[builder(into, default)]
        pub security_group_ids: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Name of the SSH keypair that instances will have by default.
        #[builder(into, default)]
        pub ssh_key_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Identifier of the stack the instance will belong to.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub stack_id: pulumi_wasm_rust::Output<String>,
        /// Desired state of the instance. Valid values are `running` or `stopped`.
        #[builder(into, default)]
        pub state: pulumi_wasm_rust::Output<Option<String>>,
        /// Instance status. Will be one of `booting`, `connection_lost`, `online`, `pending`, `rebooting`, `requested`, `running_setup`, `setup_failed`, `shutting_down`, `start_failed`, `stop_failed`, `stopped`, `stopping`, `terminated`, or `terminating`.
        #[builder(into, default)]
        pub status: pulumi_wasm_rust::Output<Option<String>>,
        /// Subnet ID to attach to.
        #[builder(into, default)]
        pub subnet_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Instance tenancy to use. Valid values are `default`, `dedicated` or `host`.
        #[builder(into, default)]
        pub tenancy: pulumi_wasm_rust::Output<Option<String>>,
        /// Keyword to choose what virtualization mode created instances will use. Valid values are `paravirtual` or `hvm`.
        #[builder(into, default)]
        pub virtualization_type: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct InstanceResult {
        /// OpsWorks agent to install. Default is `INHERIT`.
        pub agent_version: pulumi_wasm_rust::Output<Option<String>>,
        /// AMI to use for the instance.  If an AMI is specified, `os` must be `Custom`.
        pub ami_id: pulumi_wasm_rust::Output<String>,
        /// Machine architecture for created instances.  Valid values are `x86_64` or `i386`. The default is `x86_64`.
        pub architecture: pulumi_wasm_rust::Output<Option<String>>,
        /// Creates load-based or time-based instances.  Valid values are `load`, `timer`.
        pub auto_scaling_type: pulumi_wasm_rust::Output<Option<String>>,
        /// Name of the availability zone where instances will be created by default.
        pub availability_zone: pulumi_wasm_rust::Output<String>,
        /// Time that the instance was created.
        pub created_at: pulumi_wasm_rust::Output<String>,
        /// Whether to delete EBS volume on deletion. Default is `true`.
        pub delete_ebs: pulumi_wasm_rust::Output<Option<bool>>,
        /// Whether to delete the Elastic IP on deletion.
        pub delete_eip: pulumi_wasm_rust::Output<Option<bool>>,
        /// Configuration block for additional EBS block devices to attach to the instance. See Block Devices below.
        pub ebs_block_devices: pulumi_wasm_rust::Output<
            Vec<super::super::types::opsworks::InstanceEbsBlockDevice>,
        >,
        /// Whether the launched EC2 instance will be EBS-optimized.
        pub ebs_optimized: pulumi_wasm_rust::Output<Option<bool>>,
        /// EC2 instance ID.
        pub ec2_instance_id: pulumi_wasm_rust::Output<String>,
        /// ECS cluster's ARN for container instances.
        pub ecs_cluster_arn: pulumi_wasm_rust::Output<String>,
        /// Instance Elastic IP address.
        pub elastic_ip: pulumi_wasm_rust::Output<String>,
        /// Configuration block for ephemeral (also known as "Instance Store") volumes on the instance. See Block Devices below.
        pub ephemeral_block_devices: pulumi_wasm_rust::Output<
            Vec<super::super::types::opsworks::InstanceEphemeralBlockDevice>,
        >,
        /// Instance's host name.
        pub hostname: pulumi_wasm_rust::Output<String>,
        /// For registered instances, infrastructure class: ec2 or on-premises.
        pub infrastructure_class: pulumi_wasm_rust::Output<String>,
        /// Controls where to install OS and package updates when the instance boots.  Default is `true`.
        pub install_updates_on_boot: pulumi_wasm_rust::Output<Option<bool>>,
        /// ARN of the instance's IAM profile.
        pub instance_profile_arn: pulumi_wasm_rust::Output<String>,
        /// Type of instance to start.
        pub instance_type: pulumi_wasm_rust::Output<Option<String>>,
        /// ID of the last service error.
        pub last_service_error_id: pulumi_wasm_rust::Output<String>,
        /// List of the layers the instance will belong to.
        pub layer_ids: pulumi_wasm_rust::Output<Vec<String>>,
        /// Name of operating system that will be installed.
        pub os: pulumi_wasm_rust::Output<String>,
        /// Instance's platform.
        pub platform: pulumi_wasm_rust::Output<String>,
        /// Private DNS name assigned to the instance. Can only be used inside the Amazon EC2, and only available if you've enabled DNS hostnames for your VPC.
        pub private_dns: pulumi_wasm_rust::Output<String>,
        /// Private IP address assigned to the instance.
        pub private_ip: pulumi_wasm_rust::Output<String>,
        /// Public DNS name assigned to the instance. For EC2-VPC, this is only available if you've enabled DNS hostnames for your VPC.
        pub public_dns: pulumi_wasm_rust::Output<String>,
        /// Public IP address assigned to the instance, if applicable.
        pub public_ip: pulumi_wasm_rust::Output<String>,
        /// For registered instances, who performed the registration.
        pub registered_by: pulumi_wasm_rust::Output<String>,
        /// Instance's reported AWS OpsWorks Stacks agent version.
        pub reported_agent_version: pulumi_wasm_rust::Output<String>,
        /// For registered instances, the reported operating system family.
        pub reported_os_family: pulumi_wasm_rust::Output<String>,
        /// For registered instances, the reported operating system name.
        pub reported_os_name: pulumi_wasm_rust::Output<String>,
        /// For registered instances, the reported operating system version.
        pub reported_os_version: pulumi_wasm_rust::Output<String>,
        /// Configuration block for the root block device of the instance. See Block Devices below.
        pub root_block_devices: pulumi_wasm_rust::Output<
            Vec<super::super::types::opsworks::InstanceRootBlockDevice>,
        >,
        /// Name of the type of root device instances will have by default. Valid values are `ebs` or `instance-store`.
        pub root_device_type: pulumi_wasm_rust::Output<String>,
        /// Root device volume ID.
        pub root_device_volume_id: pulumi_wasm_rust::Output<String>,
        /// Associated security groups.
        pub security_group_ids: pulumi_wasm_rust::Output<Vec<String>>,
        /// SSH key's Deep Security Agent (DSA) fingerprint.
        pub ssh_host_dsa_key_fingerprint: pulumi_wasm_rust::Output<String>,
        /// SSH key's RSA fingerprint.
        pub ssh_host_rsa_key_fingerprint: pulumi_wasm_rust::Output<String>,
        /// Name of the SSH keypair that instances will have by default.
        pub ssh_key_name: pulumi_wasm_rust::Output<String>,
        /// Identifier of the stack the instance will belong to.
        ///
        /// The following arguments are optional:
        pub stack_id: pulumi_wasm_rust::Output<String>,
        /// Desired state of the instance. Valid values are `running` or `stopped`.
        pub state: pulumi_wasm_rust::Output<Option<String>>,
        /// Instance status. Will be one of `booting`, `connection_lost`, `online`, `pending`, `rebooting`, `requested`, `running_setup`, `setup_failed`, `shutting_down`, `start_failed`, `stop_failed`, `stopped`, `stopping`, `terminated`, or `terminating`.
        pub status: pulumi_wasm_rust::Output<String>,
        /// Subnet ID to attach to.
        pub subnet_id: pulumi_wasm_rust::Output<String>,
        /// Instance tenancy to use. Valid values are `default`, `dedicated` or `host`.
        pub tenancy: pulumi_wasm_rust::Output<String>,
        /// Keyword to choose what virtualization mode created instances will use. Valid values are `paravirtual` or `hvm`.
        pub virtualization_type: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: InstanceArgs) -> InstanceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let agent_version_binding = args.agent_version.get_inner();
        let ami_id_binding = args.ami_id.get_inner();
        let architecture_binding = args.architecture.get_inner();
        let auto_scaling_type_binding = args.auto_scaling_type.get_inner();
        let availability_zone_binding = args.availability_zone.get_inner();
        let created_at_binding = args.created_at.get_inner();
        let delete_ebs_binding = args.delete_ebs.get_inner();
        let delete_eip_binding = args.delete_eip.get_inner();
        let ebs_block_devices_binding = args.ebs_block_devices.get_inner();
        let ebs_optimized_binding = args.ebs_optimized.get_inner();
        let ecs_cluster_arn_binding = args.ecs_cluster_arn.get_inner();
        let elastic_ip_binding = args.elastic_ip.get_inner();
        let ephemeral_block_devices_binding = args.ephemeral_block_devices.get_inner();
        let hostname_binding = args.hostname.get_inner();
        let infrastructure_class_binding = args.infrastructure_class.get_inner();
        let install_updates_on_boot_binding = args.install_updates_on_boot.get_inner();
        let instance_profile_arn_binding = args.instance_profile_arn.get_inner();
        let instance_type_binding = args.instance_type.get_inner();
        let layer_ids_binding = args.layer_ids.get_inner();
        let os_binding = args.os.get_inner();
        let root_block_devices_binding = args.root_block_devices.get_inner();
        let root_device_type_binding = args.root_device_type.get_inner();
        let security_group_ids_binding = args.security_group_ids.get_inner();
        let ssh_key_name_binding = args.ssh_key_name.get_inner();
        let stack_id_binding = args.stack_id.get_inner();
        let state_binding = args.state.get_inner();
        let status_binding = args.status.get_inner();
        let subnet_id_binding = args.subnet_id.get_inner();
        let tenancy_binding = args.tenancy.get_inner();
        let virtualization_type_binding = args.virtualization_type.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:opsworks/instance:Instance".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "agentVersion".into(),
                    value: &agent_version_binding,
                },
                register_interface::ObjectField {
                    name: "amiId".into(),
                    value: &ami_id_binding,
                },
                register_interface::ObjectField {
                    name: "architecture".into(),
                    value: &architecture_binding,
                },
                register_interface::ObjectField {
                    name: "autoScalingType".into(),
                    value: &auto_scaling_type_binding,
                },
                register_interface::ObjectField {
                    name: "availabilityZone".into(),
                    value: &availability_zone_binding,
                },
                register_interface::ObjectField {
                    name: "createdAt".into(),
                    value: &created_at_binding,
                },
                register_interface::ObjectField {
                    name: "deleteEbs".into(),
                    value: &delete_ebs_binding,
                },
                register_interface::ObjectField {
                    name: "deleteEip".into(),
                    value: &delete_eip_binding,
                },
                register_interface::ObjectField {
                    name: "ebsBlockDevices".into(),
                    value: &ebs_block_devices_binding,
                },
                register_interface::ObjectField {
                    name: "ebsOptimized".into(),
                    value: &ebs_optimized_binding,
                },
                register_interface::ObjectField {
                    name: "ecsClusterArn".into(),
                    value: &ecs_cluster_arn_binding,
                },
                register_interface::ObjectField {
                    name: "elasticIp".into(),
                    value: &elastic_ip_binding,
                },
                register_interface::ObjectField {
                    name: "ephemeralBlockDevices".into(),
                    value: &ephemeral_block_devices_binding,
                },
                register_interface::ObjectField {
                    name: "hostname".into(),
                    value: &hostname_binding,
                },
                register_interface::ObjectField {
                    name: "infrastructureClass".into(),
                    value: &infrastructure_class_binding,
                },
                register_interface::ObjectField {
                    name: "installUpdatesOnBoot".into(),
                    value: &install_updates_on_boot_binding,
                },
                register_interface::ObjectField {
                    name: "instanceProfileArn".into(),
                    value: &instance_profile_arn_binding,
                },
                register_interface::ObjectField {
                    name: "instanceType".into(),
                    value: &instance_type_binding,
                },
                register_interface::ObjectField {
                    name: "layerIds".into(),
                    value: &layer_ids_binding,
                },
                register_interface::ObjectField {
                    name: "os".into(),
                    value: &os_binding,
                },
                register_interface::ObjectField {
                    name: "rootBlockDevices".into(),
                    value: &root_block_devices_binding,
                },
                register_interface::ObjectField {
                    name: "rootDeviceType".into(),
                    value: &root_device_type_binding,
                },
                register_interface::ObjectField {
                    name: "securityGroupIds".into(),
                    value: &security_group_ids_binding,
                },
                register_interface::ObjectField {
                    name: "sshKeyName".into(),
                    value: &ssh_key_name_binding,
                },
                register_interface::ObjectField {
                    name: "stackId".into(),
                    value: &stack_id_binding,
                },
                register_interface::ObjectField {
                    name: "state".into(),
                    value: &state_binding,
                },
                register_interface::ObjectField {
                    name: "status".into(),
                    value: &status_binding,
                },
                register_interface::ObjectField {
                    name: "subnetId".into(),
                    value: &subnet_id_binding,
                },
                register_interface::ObjectField {
                    name: "tenancy".into(),
                    value: &tenancy_binding,
                },
                register_interface::ObjectField {
                    name: "virtualizationType".into(),
                    value: &virtualization_type_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "agentVersion".into(),
                },
                register_interface::ResultField {
                    name: "amiId".into(),
                },
                register_interface::ResultField {
                    name: "architecture".into(),
                },
                register_interface::ResultField {
                    name: "autoScalingType".into(),
                },
                register_interface::ResultField {
                    name: "availabilityZone".into(),
                },
                register_interface::ResultField {
                    name: "createdAt".into(),
                },
                register_interface::ResultField {
                    name: "deleteEbs".into(),
                },
                register_interface::ResultField {
                    name: "deleteEip".into(),
                },
                register_interface::ResultField {
                    name: "ebsBlockDevices".into(),
                },
                register_interface::ResultField {
                    name: "ebsOptimized".into(),
                },
                register_interface::ResultField {
                    name: "ec2InstanceId".into(),
                },
                register_interface::ResultField {
                    name: "ecsClusterArn".into(),
                },
                register_interface::ResultField {
                    name: "elasticIp".into(),
                },
                register_interface::ResultField {
                    name: "ephemeralBlockDevices".into(),
                },
                register_interface::ResultField {
                    name: "hostname".into(),
                },
                register_interface::ResultField {
                    name: "infrastructureClass".into(),
                },
                register_interface::ResultField {
                    name: "installUpdatesOnBoot".into(),
                },
                register_interface::ResultField {
                    name: "instanceProfileArn".into(),
                },
                register_interface::ResultField {
                    name: "instanceType".into(),
                },
                register_interface::ResultField {
                    name: "lastServiceErrorId".into(),
                },
                register_interface::ResultField {
                    name: "layerIds".into(),
                },
                register_interface::ResultField {
                    name: "os".into(),
                },
                register_interface::ResultField {
                    name: "platform".into(),
                },
                register_interface::ResultField {
                    name: "privateDns".into(),
                },
                register_interface::ResultField {
                    name: "privateIp".into(),
                },
                register_interface::ResultField {
                    name: "publicDns".into(),
                },
                register_interface::ResultField {
                    name: "publicIp".into(),
                },
                register_interface::ResultField {
                    name: "registeredBy".into(),
                },
                register_interface::ResultField {
                    name: "reportedAgentVersion".into(),
                },
                register_interface::ResultField {
                    name: "reportedOsFamily".into(),
                },
                register_interface::ResultField {
                    name: "reportedOsName".into(),
                },
                register_interface::ResultField {
                    name: "reportedOsVersion".into(),
                },
                register_interface::ResultField {
                    name: "rootBlockDevices".into(),
                },
                register_interface::ResultField {
                    name: "rootDeviceType".into(),
                },
                register_interface::ResultField {
                    name: "rootDeviceVolumeId".into(),
                },
                register_interface::ResultField {
                    name: "securityGroupIds".into(),
                },
                register_interface::ResultField {
                    name: "sshHostDsaKeyFingerprint".into(),
                },
                register_interface::ResultField {
                    name: "sshHostRsaKeyFingerprint".into(),
                },
                register_interface::ResultField {
                    name: "sshKeyName".into(),
                },
                register_interface::ResultField {
                    name: "stackId".into(),
                },
                register_interface::ResultField {
                    name: "state".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
                register_interface::ResultField {
                    name: "subnetId".into(),
                },
                register_interface::ResultField {
                    name: "tenancy".into(),
                },
                register_interface::ResultField {
                    name: "virtualizationType".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        InstanceResult {
            agent_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("agentVersion").unwrap(),
            ),
            ami_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("amiId").unwrap(),
            ),
            architecture: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("architecture").unwrap(),
            ),
            auto_scaling_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("autoScalingType").unwrap(),
            ),
            availability_zone: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("availabilityZone").unwrap(),
            ),
            created_at: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createdAt").unwrap(),
            ),
            delete_ebs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deleteEbs").unwrap(),
            ),
            delete_eip: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deleteEip").unwrap(),
            ),
            ebs_block_devices: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ebsBlockDevices").unwrap(),
            ),
            ebs_optimized: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ebsOptimized").unwrap(),
            ),
            ec2_instance_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ec2InstanceId").unwrap(),
            ),
            ecs_cluster_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ecsClusterArn").unwrap(),
            ),
            elastic_ip: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("elasticIp").unwrap(),
            ),
            ephemeral_block_devices: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ephemeralBlockDevices").unwrap(),
            ),
            hostname: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hostname").unwrap(),
            ),
            infrastructure_class: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("infrastructureClass").unwrap(),
            ),
            install_updates_on_boot: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("installUpdatesOnBoot").unwrap(),
            ),
            instance_profile_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instanceProfileArn").unwrap(),
            ),
            instance_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instanceType").unwrap(),
            ),
            last_service_error_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lastServiceErrorId").unwrap(),
            ),
            layer_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("layerIds").unwrap(),
            ),
            os: pulumi_wasm_rust::__private::into_domain(hashmap.remove("os").unwrap()),
            platform: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("platform").unwrap(),
            ),
            private_dns: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("privateDns").unwrap(),
            ),
            private_ip: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("privateIp").unwrap(),
            ),
            public_dns: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("publicDns").unwrap(),
            ),
            public_ip: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("publicIp").unwrap(),
            ),
            registered_by: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("registeredBy").unwrap(),
            ),
            reported_agent_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("reportedAgentVersion").unwrap(),
            ),
            reported_os_family: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("reportedOsFamily").unwrap(),
            ),
            reported_os_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("reportedOsName").unwrap(),
            ),
            reported_os_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("reportedOsVersion").unwrap(),
            ),
            root_block_devices: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("rootBlockDevices").unwrap(),
            ),
            root_device_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("rootDeviceType").unwrap(),
            ),
            root_device_volume_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("rootDeviceVolumeId").unwrap(),
            ),
            security_group_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("securityGroupIds").unwrap(),
            ),
            ssh_host_dsa_key_fingerprint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sshHostDsaKeyFingerprint").unwrap(),
            ),
            ssh_host_rsa_key_fingerprint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sshHostRsaKeyFingerprint").unwrap(),
            ),
            ssh_key_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sshKeyName").unwrap(),
            ),
            stack_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("stackId").unwrap(),
            ),
            state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("state").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
            subnet_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subnetId").unwrap(),
            ),
            tenancy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tenancy").unwrap(),
            ),
            virtualization_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("virtualizationType").unwrap(),
            ),
        }
    }
}