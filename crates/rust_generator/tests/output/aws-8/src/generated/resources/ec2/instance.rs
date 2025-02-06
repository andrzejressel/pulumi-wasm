/// Provides an EC2 instance resource. This allows instances to be created, updated, and deleted.
///
/// ## Example Usage
///
/// ### Basic example using AMI lookup
///
/// ```yaml
/// resources:
///   web:
///     type: aws:ec2:Instance
///     properties:
///       ami: ${ubuntu.id}
///       instanceType: t3.micro
///       tags:
///         Name: HelloWorld
/// variables:
///   ubuntu:
///     fn::invoke:
///       function: aws:ec2:getAmi
///       arguments:
///         mostRecent: true
///         filters:
///           - name: name
///             values:
///               - ubuntu/images/hvm-ssd/ubuntu-jammy-22.04-amd64-server-*
///           - name: virtualization-type
///             values:
///               - hvm
///         owners:
///           - '099720109477'
/// ```
///
/// ### Spot instance example
///
/// ```yaml
/// resources:
///   thisInstance:
///     type: aws:ec2:Instance
///     name: this
///     properties:
///       ami: ${this.id}
///       instanceMarketOptions:
///         marketType: spot
///         spotOptions:
///           maxPrice: 0.0031
///       instanceType: t4g.nano
///       tags:
///         Name: test-spot
/// variables:
///   this:
///     fn::invoke:
///       function: aws:ec2:getAmi
///       arguments:
///         mostRecent: true
///         owners:
///           - amazon
///         filters:
///           - name: architecture
///             values:
///               - arm64
///           - name: name
///             values:
///               - al2023-ami-2023*
/// ```
///
/// ### Network and credit specification example
///
/// ```yaml
/// resources:
///   myVpc:
///     type: aws:ec2:Vpc
///     name: my_vpc
///     properties:
///       cidrBlock: 172.16.0.0/16
///       tags:
///         Name: tf-example
///   mySubnet:
///     type: aws:ec2:Subnet
///     name: my_subnet
///     properties:
///       vpcId: ${myVpc.id}
///       cidrBlock: 172.16.10.0/24
///       availabilityZone: us-west-2a
///       tags:
///         Name: tf-example
///   foo:
///     type: aws:ec2:NetworkInterface
///     properties:
///       subnetId: ${mySubnet.id}
///       privateIps:
///         - 172.16.10.100
///       tags:
///         Name: primary_network_interface
///   fooInstance:
///     type: aws:ec2:Instance
///     name: foo
///     properties:
///       ami: ami-005e54dee72cc1d00
///       instanceType: t2.micro
///       networkInterfaces:
///         - networkInterfaceId: ${foo.id}
///           deviceIndex: 0
///       creditSpecification:
///         cpuCredits: unlimited
/// ```
///
/// ### CPU options example
///
/// ```yaml
/// resources:
///   example:
///     type: aws:ec2:Vpc
///     properties:
///       cidrBlock: 172.16.0.0/16
///       tags:
///         Name: tf-example
///   exampleSubnet:
///     type: aws:ec2:Subnet
///     name: example
///     properties:
///       vpcId: ${example.id}
///       cidrBlock: 172.16.10.0/24
///       availabilityZone: us-east-2a
///       tags:
///         Name: tf-example
///   exampleInstance:
///     type: aws:ec2:Instance
///     name: example
///     properties:
///       ami: ${["amzn-linux-2023-ami"].id}
///       instanceType: c6a.2xlarge
///       subnetId: ${exampleSubnet.id}
///       cpuOptions:
///         coreCount: 2
///         threadsPerCore: 2
///       tags:
///         Name: tf-example
/// variables:
///   amzn-linux-2023-ami:
///     fn::invoke:
///       function: aws:ec2:getAmi
///       arguments:
///         mostRecent: true
///         owners:
///           - amazon
///         filters:
///           - name: name
///             values:
///               - al2023-ami-2023.*-x86_64
/// ```
///
/// ### Host resource group or License Manager registered AMI example
///
/// A host resource group is a collection of Dedicated Hosts that you can manage as a single entity. As you launch instances, License Manager allocates the hosts and launches instances on them based on the settings that you configured. You can add existing Dedicated Hosts to a host resource group and take advantage of automated host management through License Manager.
///
/// > **NOTE:** A dedicated host is automatically associated with a License Manager host resource group if **Allocate hosts automatically** is enabled. Otherwise, use the `host_resource_group_arn` argument to explicitly associate the instance with the host resource group.
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let this = instance::create(
///         "this",
///         InstanceArgs::builder()
///             .ami("ami-0dcc1e21636832c5d")
///             .host_resource_group_arn(
///                 "arn:aws:resource-groups:us-west-2:123456789012:group/win-testhost",
///             )
///             .instance_type("m5.large")
///             .tenancy("host")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Tag Guide
///
/// These are the five types of tags you might encounter relative to an `aws.ec2.Instance`:
///
/// 1. **Instance tags**: Applied to instances but not to `ebs_block_device` and `root_block_device` volumes.
/// 2. **Default tags**: Applied to the instance and to `ebs_block_device` and `root_block_device` volumes.
/// 3. **Volume tags**: Applied during creation to `ebs_block_device` and `root_block_device` volumes.
/// 4. **Root block device tags**: Applied only to the `root_block_device` volume. These conflict with `volume_tags`.
/// 5. **EBS block device tags**: Applied only to the specific `ebs_block_device` volume you configure them for and cannot be updated. These conflict with `volume_tags`.
///
/// Do not use `volume_tags` if you plan to manage block device tags outside the `aws.ec2.Instance` configuration, such as using `tags` in an `aws.ebs.Volume` resource attached via `aws.ec2.VolumeAttachment`. Doing so will result in resource cycling and inconsistent behavior.
///
/// ## Import
///
/// Using `pulumi import`, import instances using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:ec2/instance:Instance web i-12345678
/// ```
pub mod instance {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct InstanceArgs {
        /// AMI to use for the instance. Required unless `launch_template` is specified and the Launch Template specifes an AMI. If an AMI is specified in the Launch Template, setting `ami` will override the AMI specified in the Launch Template.
        #[builder(into, default)]
        pub ami: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Whether to associate a public IP address with an instance in a VPC.
        #[builder(into, default)]
        pub associate_public_ip_address: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// AZ to start the instance in.
        #[builder(into, default)]
        pub availability_zone: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Describes an instance's Capacity Reservation targeting option. See Capacity Reservation Specification below for more details.
        ///
        /// > **NOTE:** Changing `cpu_core_count` and/or `cpu_threads_per_core` will cause the resource to be destroyed and re-created.
        #[builder(into, default)]
        pub capacity_reservation_specification: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::ec2::InstanceCapacityReservationSpecification>,
        >,
        /// Sets the number of CPU cores for an instance. This option is only supported on creation of instance type that support CPU Options [CPU Cores and Threads Per CPU Core Per Instance Type](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/instance-optimize-cpu.html#cpu-options-supported-instances-values) - specifying this option for unsupported instance types will return an error from the EC2 API.
        #[builder(into, default)]
        pub cpu_core_count: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// The CPU options for the instance. See CPU Options below for more details.
        #[builder(into, default)]
        pub cpu_options: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::ec2::InstanceCpuOptions>,
        >,
        /// If set to 1, hyperthreading is disabled on the launched instance. Defaults to 2 if not set. See [Optimizing CPU Options](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/instance-optimize-cpu.html) for more information.
        #[builder(into, default)]
        pub cpu_threads_per_core: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// Configuration block for customizing the credit specification of the instance. See Credit Specification below for more details. This provider will only perform drift detection of its value when present in a configuration. Removing this configuration on existing instances will only stop managing it. It will not change the configuration back to the default for the instance type.
        #[builder(into, default)]
        pub credit_specification: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::ec2::InstanceCreditSpecification>,
        >,
        /// If true, enables [EC2 Instance Stop Protection](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/Stop_Start.html#Using_StopProtection).
        #[builder(into, default)]
        pub disable_api_stop: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// If true, enables [EC2 Instance Termination Protection](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/terminating-instances.html#Using_ChangingDisableAPITermination).
        #[builder(into, default)]
        pub disable_api_termination: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// One or more configuration blocks with additional EBS block devices to attach to the instance. Block device configurations only apply on resource creation. See Block Devices below for details on attributes and drift detection. When accessing this as an attribute reference, it is a set of objects.
        #[builder(into, default)]
        pub ebs_block_devices: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::ec2::InstanceEbsBlockDevice>>,
        >,
        /// If true, the launched EC2 instance will be EBS-optimized. Note that if this is not set on an instance type that is optimized by default then this will show as disabled but if the instance type is optimized by default then there is no need to set this and there is no effect to disabling it. See the [EBS Optimized section](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/EBSOptimized.html) of the AWS User Guide for more information.
        #[builder(into, default)]
        pub ebs_optimized: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Whether to assign a primary IPv6 Global Unicast Address (GUA) to the instance when launched in a dual-stack or IPv6-only subnet. A primary IPv6 address ensures a consistent IPv6 address for the instance and is automatically assigned by AWS to the ENI. Once enabled, the first IPv6 GUA becomes the primary IPv6 address and cannot be disabled. The primary IPv6 address remains until the instance is terminated or the ENI is detached. Disabling `enable_primary_ipv6` after it has been enabled forces recreation of the instance.
        #[builder(into, default)]
        pub enable_primary_ipv6: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Enable Nitro Enclaves on launched instances. See Enclave Options below for more details.
        #[builder(into, default)]
        pub enclave_options: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::ec2::InstanceEnclaveOptions>,
        >,
        /// One or more configuration blocks to customize Ephemeral (also known as "Instance Store") volumes on the instance. See Block Devices below for details. When accessing this as an attribute reference, it is a set of objects.
        #[builder(into, default)]
        pub ephemeral_block_devices: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::ec2::InstanceEphemeralBlockDevice>>,
        >,
        /// If true, wait for password data to become available and retrieve it. Useful for getting the administrator password for instances running Microsoft Windows. The password data is exported to the `password_data` attribute. See [GetPasswordData](https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_GetPasswordData.html) for more information.
        #[builder(into, default)]
        pub get_password_data: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// If true, the launched EC2 instance will support hibernation.
        #[builder(into, default)]
        pub hibernation: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// ID of a dedicated host that the instance will be assigned to. Use when an instance is to be launched on a specific dedicated host.
        #[builder(into, default)]
        pub host_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// ARN of the host resource group in which to launch the instances. If you specify an ARN, omit the `tenancy` parameter or set it to `host`.
        #[builder(into, default)]
        pub host_resource_group_arn: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// IAM Instance Profile to launch the instance with. Specified as the name of the Instance Profile. Ensure your credentials have the correct permission to assign the instance profile according to the [EC2 documentation](http://docs.aws.amazon.com/IAM/latest/UserGuide/id_roles_use_switch-role-ec2.html#roles-usingrole-ec2instance-permissions), notably `iam:PassRole`.
        #[builder(into, default)]
        pub iam_instance_profile: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Shutdown behavior for the instance. Amazon defaults this to `stop` for EBS-backed instances and `terminate` for instance-store instances. Cannot be set on instance-store instances. See [Shutdown Behavior](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/terminating-instances.html#Using_ChangingInstanceInitiatedShutdownBehavior) for more information.
        #[builder(into, default)]
        pub instance_initiated_shutdown_behavior: pulumi_wasm_rust::InputOrOutput<
            Option<String>,
        >,
        /// Describes the market (purchasing) option for the instances. See Market Options below for details on attributes.
        #[builder(into, default)]
        pub instance_market_options: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::ec2::InstanceInstanceMarketOptions>,
        >,
        /// Instance type to use for the instance. Required unless `launch_template` is specified and the Launch Template specifies an instance type. If an instance type is specified in the Launch Template, setting `instance_type` will override the instance type specified in the Launch Template. Updates to this field will trigger a stop/start of the EC2 instance.
        #[builder(into, default)]
        pub instance_type: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Number of IPv6 addresses to associate with the primary network interface. Amazon EC2 chooses the IPv6 addresses from the range of your subnet.
        #[builder(into, default)]
        pub ipv6_address_count: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// Specify one or more IPv6 addresses from the range of the subnet to associate with the primary network interface
        #[builder(into, default)]
        pub ipv6_addresses: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// Key name of the Key Pair to use for the instance; which can be managed using the `aws.ec2.KeyPair` resource.
        #[builder(into, default)]
        pub key_name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specifies a Launch Template to configure the instance. Parameters configured on this resource will override the corresponding parameters in the Launch Template. See Launch Template Specification below for more details.
        #[builder(into, default)]
        pub launch_template: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::ec2::InstanceLaunchTemplate>,
        >,
        /// Maintenance and recovery options for the instance. See Maintenance Options below for more details.
        #[builder(into, default)]
        pub maintenance_options: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::ec2::InstanceMaintenanceOptions>,
        >,
        /// Customize the metadata options of the instance. See Metadata Options below for more details.
        #[builder(into, default)]
        pub metadata_options: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::ec2::InstanceMetadataOptions>,
        >,
        /// If true, the launched EC2 instance will have detailed monitoring enabled. (Available since v0.6.0)
        #[builder(into, default)]
        pub monitoring: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Customize network interfaces to be attached at instance boot time. See Network Interfaces below for more details.
        #[builder(into, default)]
        pub network_interfaces: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::ec2::InstanceNetworkInterface>>,
        >,
        /// Placement Group to start the instance in.
        #[builder(into, default)]
        pub placement_group: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Number of the partition the instance is in. Valid only if the `aws.ec2.PlacementGroup` resource's `strategy` argument is set to `"partition"`.
        #[builder(into, default)]
        pub placement_partition_number: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// Options for the instance hostname. The default values are inherited from the subnet. See Private DNS Name Options below for more details.
        #[builder(into, default)]
        pub private_dns_name_options: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::ec2::InstancePrivateDnsNameOptions>,
        >,
        /// Private IP address to associate with the instance in a VPC.
        #[builder(into, default)]
        pub private_ip: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Configuration block to customize details about the root block device of the instance. See Block Devices below for details. When accessing this as an attribute reference, it is a list containing one object.
        #[builder(into, default)]
        pub root_block_device: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::ec2::InstanceRootBlockDevice>,
        >,
        /// List of secondary private IPv4 addresses to assign to the instance's primary network interface (eth0) in a VPC. Can only be assigned to the primary network interface (eth0) attached at instance creation, not a pre-existing network interface i.e., referenced in a `network_interface` block. Refer to the [Elastic network interfaces documentation](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/using-eni.html#AvailableIpPerENI) to see the maximum number of private IP addresses allowed per instance type.
        #[builder(into, default)]
        pub secondary_private_ips: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// List of security group names to associate with.
        ///
        /// > **NOTE:** If you are creating Instances in a VPC, use `vpc_security_group_ids` instead.
        #[builder(into, default)]
        pub security_groups: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// Controls if traffic is routed to the instance when the destination address does not match the instance. Used for NAT or VPNs. Defaults true.
        #[builder(into, default)]
        pub source_dest_check: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// VPC Subnet ID to launch in.
        #[builder(into, default)]
        pub subnet_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Map of tags to assign to the resource. Note that these tags apply to the instance and not block storage devices. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Tenancy of the instance (if the instance is running in a VPC). An instance with a tenancy of `dedicated` runs on single-tenant hardware. The `host` tenancy is not supported for the import-instance command. Valid values are `default`, `dedicated`, and `host`.
        #[builder(into, default)]
        pub tenancy: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// User data to provide when launching the instance. Do not pass gzip-compressed data via this argument; see `user_data_base64` instead. Updates to this field will trigger a stop/start of the EC2 instance by default. If the `user_data_replace_on_change` is set then updates to this field will trigger a destroy and recreate of the EC2 instance.
        #[builder(into, default)]
        pub user_data: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Can be used instead of `user_data` to pass base64-encoded binary data directly. Use this instead of `user_data` whenever the value is not a valid UTF-8 string. For example, gzip-encoded user data must be base64-encoded and passed via this argument to avoid corruption. Updates to this field will trigger a stop/start of the EC2 instance by default. If the `user_data_replace_on_change` is set then updates to this field will trigger a destroy and recreate of the EC2 instance.
        #[builder(into, default)]
        pub user_data_base64: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// When used in combination with `user_data` or `user_data_base64` will trigger a destroy and recreate of the EC2 instance when set to `true`. Defaults to `false` if not set.
        #[builder(into, default)]
        pub user_data_replace_on_change: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Map of tags to assign, at instance-creation time, to root and EBS volumes.
        ///
        /// > **NOTE:** Do not use `volume_tags` if you plan to manage block device tags outside the `aws.ec2.Instance` configuration, such as using `tags` in an `aws.ebs.Volume` resource attached via `aws.ec2.VolumeAttachment`. Doing so will result in resource cycling and inconsistent behavior.
        #[builder(into, default)]
        pub volume_tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// List of security group IDs to associate with.
        #[builder(into, default)]
        pub vpc_security_group_ids: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
    }
    #[allow(dead_code)]
    pub struct InstanceResult {
        /// AMI to use for the instance. Required unless `launch_template` is specified and the Launch Template specifes an AMI. If an AMI is specified in the Launch Template, setting `ami` will override the AMI specified in the Launch Template.
        pub ami: pulumi_wasm_rust::Output<String>,
        /// ARN of the instance.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Whether to associate a public IP address with an instance in a VPC.
        pub associate_public_ip_address: pulumi_wasm_rust::Output<bool>,
        /// AZ to start the instance in.
        pub availability_zone: pulumi_wasm_rust::Output<String>,
        /// Describes an instance's Capacity Reservation targeting option. See Capacity Reservation Specification below for more details.
        ///
        /// > **NOTE:** Changing `cpu_core_count` and/or `cpu_threads_per_core` will cause the resource to be destroyed and re-created.
        pub capacity_reservation_specification: pulumi_wasm_rust::Output<
            super::super::types::ec2::InstanceCapacityReservationSpecification,
        >,
        /// Sets the number of CPU cores for an instance. This option is only supported on creation of instance type that support CPU Options [CPU Cores and Threads Per CPU Core Per Instance Type](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/instance-optimize-cpu.html#cpu-options-supported-instances-values) - specifying this option for unsupported instance types will return an error from the EC2 API.
        pub cpu_core_count: pulumi_wasm_rust::Output<i32>,
        /// The CPU options for the instance. See CPU Options below for more details.
        pub cpu_options: pulumi_wasm_rust::Output<
            super::super::types::ec2::InstanceCpuOptions,
        >,
        /// If set to 1, hyperthreading is disabled on the launched instance. Defaults to 2 if not set. See [Optimizing CPU Options](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/instance-optimize-cpu.html) for more information.
        pub cpu_threads_per_core: pulumi_wasm_rust::Output<i32>,
        /// Configuration block for customizing the credit specification of the instance. See Credit Specification below for more details. This provider will only perform drift detection of its value when present in a configuration. Removing this configuration on existing instances will only stop managing it. It will not change the configuration back to the default for the instance type.
        pub credit_specification: pulumi_wasm_rust::Output<
            Option<super::super::types::ec2::InstanceCreditSpecification>,
        >,
        /// If true, enables [EC2 Instance Stop Protection](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/Stop_Start.html#Using_StopProtection).
        pub disable_api_stop: pulumi_wasm_rust::Output<bool>,
        /// If true, enables [EC2 Instance Termination Protection](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/terminating-instances.html#Using_ChangingDisableAPITermination).
        pub disable_api_termination: pulumi_wasm_rust::Output<bool>,
        /// One or more configuration blocks with additional EBS block devices to attach to the instance. Block device configurations only apply on resource creation. See Block Devices below for details on attributes and drift detection. When accessing this as an attribute reference, it is a set of objects.
        pub ebs_block_devices: pulumi_wasm_rust::Output<
            Vec<super::super::types::ec2::InstanceEbsBlockDevice>,
        >,
        /// If true, the launched EC2 instance will be EBS-optimized. Note that if this is not set on an instance type that is optimized by default then this will show as disabled but if the instance type is optimized by default then there is no need to set this and there is no effect to disabling it. See the [EBS Optimized section](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/EBSOptimized.html) of the AWS User Guide for more information.
        pub ebs_optimized: pulumi_wasm_rust::Output<bool>,
        /// Whether to assign a primary IPv6 Global Unicast Address (GUA) to the instance when launched in a dual-stack or IPv6-only subnet. A primary IPv6 address ensures a consistent IPv6 address for the instance and is automatically assigned by AWS to the ENI. Once enabled, the first IPv6 GUA becomes the primary IPv6 address and cannot be disabled. The primary IPv6 address remains until the instance is terminated or the ENI is detached. Disabling `enable_primary_ipv6` after it has been enabled forces recreation of the instance.
        pub enable_primary_ipv6: pulumi_wasm_rust::Output<bool>,
        /// Enable Nitro Enclaves on launched instances. See Enclave Options below for more details.
        pub enclave_options: pulumi_wasm_rust::Output<
            super::super::types::ec2::InstanceEnclaveOptions,
        >,
        /// One or more configuration blocks to customize Ephemeral (also known as "Instance Store") volumes on the instance. See Block Devices below for details. When accessing this as an attribute reference, it is a set of objects.
        pub ephemeral_block_devices: pulumi_wasm_rust::Output<
            Vec<super::super::types::ec2::InstanceEphemeralBlockDevice>,
        >,
        /// If true, wait for password data to become available and retrieve it. Useful for getting the administrator password for instances running Microsoft Windows. The password data is exported to the `password_data` attribute. See [GetPasswordData](https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_GetPasswordData.html) for more information.
        pub get_password_data: pulumi_wasm_rust::Output<Option<bool>>,
        /// If true, the launched EC2 instance will support hibernation.
        pub hibernation: pulumi_wasm_rust::Output<Option<bool>>,
        /// ID of a dedicated host that the instance will be assigned to. Use when an instance is to be launched on a specific dedicated host.
        pub host_id: pulumi_wasm_rust::Output<String>,
        /// ARN of the host resource group in which to launch the instances. If you specify an ARN, omit the `tenancy` parameter or set it to `host`.
        pub host_resource_group_arn: pulumi_wasm_rust::Output<String>,
        /// IAM Instance Profile to launch the instance with. Specified as the name of the Instance Profile. Ensure your credentials have the correct permission to assign the instance profile according to the [EC2 documentation](http://docs.aws.amazon.com/IAM/latest/UserGuide/id_roles_use_switch-role-ec2.html#roles-usingrole-ec2instance-permissions), notably `iam:PassRole`.
        pub iam_instance_profile: pulumi_wasm_rust::Output<String>,
        /// Shutdown behavior for the instance. Amazon defaults this to `stop` for EBS-backed instances and `terminate` for instance-store instances. Cannot be set on instance-store instances. See [Shutdown Behavior](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/terminating-instances.html#Using_ChangingInstanceInitiatedShutdownBehavior) for more information.
        pub instance_initiated_shutdown_behavior: pulumi_wasm_rust::Output<String>,
        /// Indicates whether this is a Spot Instance or a Scheduled Instance.
        pub instance_lifecycle: pulumi_wasm_rust::Output<String>,
        /// Describes the market (purchasing) option for the instances. See Market Options below for details on attributes.
        pub instance_market_options: pulumi_wasm_rust::Output<
            super::super::types::ec2::InstanceInstanceMarketOptions,
        >,
        /// State of the instance. One of: `pending`, `running`, `shutting-down`, `terminated`, `stopping`, `stopped`. See [Instance Lifecycle](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-instance-lifecycle.html) for more information.
        pub instance_state: pulumi_wasm_rust::Output<String>,
        /// Instance type to use for the instance. Required unless `launch_template` is specified and the Launch Template specifies an instance type. If an instance type is specified in the Launch Template, setting `instance_type` will override the instance type specified in the Launch Template. Updates to this field will trigger a stop/start of the EC2 instance.
        pub instance_type: pulumi_wasm_rust::Output<String>,
        /// Number of IPv6 addresses to associate with the primary network interface. Amazon EC2 chooses the IPv6 addresses from the range of your subnet.
        pub ipv6_address_count: pulumi_wasm_rust::Output<i32>,
        /// Specify one or more IPv6 addresses from the range of the subnet to associate with the primary network interface
        pub ipv6_addresses: pulumi_wasm_rust::Output<Vec<String>>,
        /// Key name of the Key Pair to use for the instance; which can be managed using the `aws.ec2.KeyPair` resource.
        pub key_name: pulumi_wasm_rust::Output<String>,
        /// Specifies a Launch Template to configure the instance. Parameters configured on this resource will override the corresponding parameters in the Launch Template. See Launch Template Specification below for more details.
        pub launch_template: pulumi_wasm_rust::Output<
            Option<super::super::types::ec2::InstanceLaunchTemplate>,
        >,
        /// Maintenance and recovery options for the instance. See Maintenance Options below for more details.
        pub maintenance_options: pulumi_wasm_rust::Output<
            super::super::types::ec2::InstanceMaintenanceOptions,
        >,
        /// Customize the metadata options of the instance. See Metadata Options below for more details.
        pub metadata_options: pulumi_wasm_rust::Output<
            super::super::types::ec2::InstanceMetadataOptions,
        >,
        /// If true, the launched EC2 instance will have detailed monitoring enabled. (Available since v0.6.0)
        pub monitoring: pulumi_wasm_rust::Output<bool>,
        /// Customize network interfaces to be attached at instance boot time. See Network Interfaces below for more details.
        pub network_interfaces: pulumi_wasm_rust::Output<
            Vec<super::super::types::ec2::InstanceNetworkInterface>,
        >,
        /// ARN of the Outpost the instance is assigned to.
        pub outpost_arn: pulumi_wasm_rust::Output<String>,
        /// Base-64 encoded encrypted password data for the instance. Useful for getting the administrator password for instances running Microsoft Windows. This attribute is only exported if `get_password_data` is true. Note that this encrypted value will be stored in the state file, as with all exported attributes. See [GetPasswordData](https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_GetPasswordData.html) for more information.
        pub password_data: pulumi_wasm_rust::Output<String>,
        /// Placement Group to start the instance in.
        pub placement_group: pulumi_wasm_rust::Output<String>,
        /// Number of the partition the instance is in. Valid only if the `aws.ec2.PlacementGroup` resource's `strategy` argument is set to `"partition"`.
        pub placement_partition_number: pulumi_wasm_rust::Output<i32>,
        /// ID of the instance's primary network interface.
        pub primary_network_interface_id: pulumi_wasm_rust::Output<String>,
        /// Private DNS name assigned to the instance. Can only be used inside the Amazon EC2, and only available if you've enabled DNS hostnames for your VPC.
        pub private_dns: pulumi_wasm_rust::Output<String>,
        /// Options for the instance hostname. The default values are inherited from the subnet. See Private DNS Name Options below for more details.
        pub private_dns_name_options: pulumi_wasm_rust::Output<
            super::super::types::ec2::InstancePrivateDnsNameOptions,
        >,
        /// Private IP address to associate with the instance in a VPC.
        pub private_ip: pulumi_wasm_rust::Output<String>,
        /// Public DNS name assigned to the instance. For EC2-VPC, this is only available if you've enabled DNS hostnames for your VPC.
        pub public_dns: pulumi_wasm_rust::Output<String>,
        /// Public IP address assigned to the instance, if applicable. **NOTE**: If you are using an `aws.ec2.Eip` with your instance, you should refer to the EIP's address directly and not use `public_ip` as this field will change after the EIP is attached.
        pub public_ip: pulumi_wasm_rust::Output<String>,
        /// Configuration block to customize details about the root block device of the instance. See Block Devices below for details. When accessing this as an attribute reference, it is a list containing one object.
        pub root_block_device: pulumi_wasm_rust::Output<
            super::super::types::ec2::InstanceRootBlockDevice,
        >,
        /// List of secondary private IPv4 addresses to assign to the instance's primary network interface (eth0) in a VPC. Can only be assigned to the primary network interface (eth0) attached at instance creation, not a pre-existing network interface i.e., referenced in a `network_interface` block. Refer to the [Elastic network interfaces documentation](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/using-eni.html#AvailableIpPerENI) to see the maximum number of private IP addresses allowed per instance type.
        pub secondary_private_ips: pulumi_wasm_rust::Output<Vec<String>>,
        /// List of security group names to associate with.
        ///
        /// > **NOTE:** If you are creating Instances in a VPC, use `vpc_security_group_ids` instead.
        pub security_groups: pulumi_wasm_rust::Output<Vec<String>>,
        /// Controls if traffic is routed to the instance when the destination address does not match the instance. Used for NAT or VPNs. Defaults true.
        pub source_dest_check: pulumi_wasm_rust::Output<Option<bool>>,
        /// If the request is a Spot Instance request, the ID of the request.
        pub spot_instance_request_id: pulumi_wasm_rust::Output<String>,
        /// VPC Subnet ID to launch in.
        pub subnet_id: pulumi_wasm_rust::Output<String>,
        /// Map of tags to assign to the resource. Note that these tags apply to the instance and not block storage devices. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Tenancy of the instance (if the instance is running in a VPC). An instance with a tenancy of `dedicated` runs on single-tenant hardware. The `host` tenancy is not supported for the import-instance command. Valid values are `default`, `dedicated`, and `host`.
        pub tenancy: pulumi_wasm_rust::Output<String>,
        /// User data to provide when launching the instance. Do not pass gzip-compressed data via this argument; see `user_data_base64` instead. Updates to this field will trigger a stop/start of the EC2 instance by default. If the `user_data_replace_on_change` is set then updates to this field will trigger a destroy and recreate of the EC2 instance.
        pub user_data: pulumi_wasm_rust::Output<String>,
        /// Can be used instead of `user_data` to pass base64-encoded binary data directly. Use this instead of `user_data` whenever the value is not a valid UTF-8 string. For example, gzip-encoded user data must be base64-encoded and passed via this argument to avoid corruption. Updates to this field will trigger a stop/start of the EC2 instance by default. If the `user_data_replace_on_change` is set then updates to this field will trigger a destroy and recreate of the EC2 instance.
        pub user_data_base64: pulumi_wasm_rust::Output<String>,
        /// When used in combination with `user_data` or `user_data_base64` will trigger a destroy and recreate of the EC2 instance when set to `true`. Defaults to `false` if not set.
        pub user_data_replace_on_change: pulumi_wasm_rust::Output<Option<bool>>,
        /// Map of tags to assign, at instance-creation time, to root and EBS volumes.
        ///
        /// > **NOTE:** Do not use `volume_tags` if you plan to manage block device tags outside the `aws.ec2.Instance` configuration, such as using `tags` in an `aws.ebs.Volume` resource attached via `aws.ec2.VolumeAttachment`. Doing so will result in resource cycling and inconsistent behavior.
        pub volume_tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// List of security group IDs to associate with.
        pub vpc_security_group_ids: pulumi_wasm_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: InstanceArgs,
    ) -> InstanceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let ami_binding = args.ami.get_output(context).get_inner();
        let associate_public_ip_address_binding = args
            .associate_public_ip_address
            .get_output(context)
            .get_inner();
        let availability_zone_binding = args
            .availability_zone
            .get_output(context)
            .get_inner();
        let capacity_reservation_specification_binding = args
            .capacity_reservation_specification
            .get_output(context)
            .get_inner();
        let cpu_core_count_binding = args.cpu_core_count.get_output(context).get_inner();
        let cpu_options_binding = args.cpu_options.get_output(context).get_inner();
        let cpu_threads_per_core_binding = args
            .cpu_threads_per_core
            .get_output(context)
            .get_inner();
        let credit_specification_binding = args
            .credit_specification
            .get_output(context)
            .get_inner();
        let disable_api_stop_binding = args
            .disable_api_stop
            .get_output(context)
            .get_inner();
        let disable_api_termination_binding = args
            .disable_api_termination
            .get_output(context)
            .get_inner();
        let ebs_block_devices_binding = args
            .ebs_block_devices
            .get_output(context)
            .get_inner();
        let ebs_optimized_binding = args.ebs_optimized.get_output(context).get_inner();
        let enable_primary_ipv6_binding = args
            .enable_primary_ipv6
            .get_output(context)
            .get_inner();
        let enclave_options_binding = args
            .enclave_options
            .get_output(context)
            .get_inner();
        let ephemeral_block_devices_binding = args
            .ephemeral_block_devices
            .get_output(context)
            .get_inner();
        let get_password_data_binding = args
            .get_password_data
            .get_output(context)
            .get_inner();
        let hibernation_binding = args.hibernation.get_output(context).get_inner();
        let host_id_binding = args.host_id.get_output(context).get_inner();
        let host_resource_group_arn_binding = args
            .host_resource_group_arn
            .get_output(context)
            .get_inner();
        let iam_instance_profile_binding = args
            .iam_instance_profile
            .get_output(context)
            .get_inner();
        let instance_initiated_shutdown_behavior_binding = args
            .instance_initiated_shutdown_behavior
            .get_output(context)
            .get_inner();
        let instance_market_options_binding = args
            .instance_market_options
            .get_output(context)
            .get_inner();
        let instance_type_binding = args.instance_type.get_output(context).get_inner();
        let ipv6_address_count_binding = args
            .ipv6_address_count
            .get_output(context)
            .get_inner();
        let ipv6_addresses_binding = args.ipv6_addresses.get_output(context).get_inner();
        let key_name_binding = args.key_name.get_output(context).get_inner();
        let launch_template_binding = args
            .launch_template
            .get_output(context)
            .get_inner();
        let maintenance_options_binding = args
            .maintenance_options
            .get_output(context)
            .get_inner();
        let metadata_options_binding = args
            .metadata_options
            .get_output(context)
            .get_inner();
        let monitoring_binding = args.monitoring.get_output(context).get_inner();
        let network_interfaces_binding = args
            .network_interfaces
            .get_output(context)
            .get_inner();
        let placement_group_binding = args
            .placement_group
            .get_output(context)
            .get_inner();
        let placement_partition_number_binding = args
            .placement_partition_number
            .get_output(context)
            .get_inner();
        let private_dns_name_options_binding = args
            .private_dns_name_options
            .get_output(context)
            .get_inner();
        let private_ip_binding = args.private_ip.get_output(context).get_inner();
        let root_block_device_binding = args
            .root_block_device
            .get_output(context)
            .get_inner();
        let secondary_private_ips_binding = args
            .secondary_private_ips
            .get_output(context)
            .get_inner();
        let security_groups_binding = args
            .security_groups
            .get_output(context)
            .get_inner();
        let source_dest_check_binding = args
            .source_dest_check
            .get_output(context)
            .get_inner();
        let subnet_id_binding = args.subnet_id.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let tenancy_binding = args.tenancy.get_output(context).get_inner();
        let user_data_binding = args.user_data.get_output(context).get_inner();
        let user_data_base64_binding = args
            .user_data_base64
            .get_output(context)
            .get_inner();
        let user_data_replace_on_change_binding = args
            .user_data_replace_on_change
            .get_output(context)
            .get_inner();
        let volume_tags_binding = args.volume_tags.get_output(context).get_inner();
        let vpc_security_group_ids_binding = args
            .vpc_security_group_ids
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ec2/instance:Instance".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "ami".into(),
                    value: &ami_binding,
                },
                register_interface::ObjectField {
                    name: "associatePublicIpAddress".into(),
                    value: &associate_public_ip_address_binding,
                },
                register_interface::ObjectField {
                    name: "availabilityZone".into(),
                    value: &availability_zone_binding,
                },
                register_interface::ObjectField {
                    name: "capacityReservationSpecification".into(),
                    value: &capacity_reservation_specification_binding,
                },
                register_interface::ObjectField {
                    name: "cpuCoreCount".into(),
                    value: &cpu_core_count_binding,
                },
                register_interface::ObjectField {
                    name: "cpuOptions".into(),
                    value: &cpu_options_binding,
                },
                register_interface::ObjectField {
                    name: "cpuThreadsPerCore".into(),
                    value: &cpu_threads_per_core_binding,
                },
                register_interface::ObjectField {
                    name: "creditSpecification".into(),
                    value: &credit_specification_binding,
                },
                register_interface::ObjectField {
                    name: "disableApiStop".into(),
                    value: &disable_api_stop_binding,
                },
                register_interface::ObjectField {
                    name: "disableApiTermination".into(),
                    value: &disable_api_termination_binding,
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
                    name: "enablePrimaryIpv6".into(),
                    value: &enable_primary_ipv6_binding,
                },
                register_interface::ObjectField {
                    name: "enclaveOptions".into(),
                    value: &enclave_options_binding,
                },
                register_interface::ObjectField {
                    name: "ephemeralBlockDevices".into(),
                    value: &ephemeral_block_devices_binding,
                },
                register_interface::ObjectField {
                    name: "getPasswordData".into(),
                    value: &get_password_data_binding,
                },
                register_interface::ObjectField {
                    name: "hibernation".into(),
                    value: &hibernation_binding,
                },
                register_interface::ObjectField {
                    name: "hostId".into(),
                    value: &host_id_binding,
                },
                register_interface::ObjectField {
                    name: "hostResourceGroupArn".into(),
                    value: &host_resource_group_arn_binding,
                },
                register_interface::ObjectField {
                    name: "iamInstanceProfile".into(),
                    value: &iam_instance_profile_binding,
                },
                register_interface::ObjectField {
                    name: "instanceInitiatedShutdownBehavior".into(),
                    value: &instance_initiated_shutdown_behavior_binding,
                },
                register_interface::ObjectField {
                    name: "instanceMarketOptions".into(),
                    value: &instance_market_options_binding,
                },
                register_interface::ObjectField {
                    name: "instanceType".into(),
                    value: &instance_type_binding,
                },
                register_interface::ObjectField {
                    name: "ipv6AddressCount".into(),
                    value: &ipv6_address_count_binding,
                },
                register_interface::ObjectField {
                    name: "ipv6Addresses".into(),
                    value: &ipv6_addresses_binding,
                },
                register_interface::ObjectField {
                    name: "keyName".into(),
                    value: &key_name_binding,
                },
                register_interface::ObjectField {
                    name: "launchTemplate".into(),
                    value: &launch_template_binding,
                },
                register_interface::ObjectField {
                    name: "maintenanceOptions".into(),
                    value: &maintenance_options_binding,
                },
                register_interface::ObjectField {
                    name: "metadataOptions".into(),
                    value: &metadata_options_binding,
                },
                register_interface::ObjectField {
                    name: "monitoring".into(),
                    value: &monitoring_binding,
                },
                register_interface::ObjectField {
                    name: "networkInterfaces".into(),
                    value: &network_interfaces_binding,
                },
                register_interface::ObjectField {
                    name: "placementGroup".into(),
                    value: &placement_group_binding,
                },
                register_interface::ObjectField {
                    name: "placementPartitionNumber".into(),
                    value: &placement_partition_number_binding,
                },
                register_interface::ObjectField {
                    name: "privateDnsNameOptions".into(),
                    value: &private_dns_name_options_binding,
                },
                register_interface::ObjectField {
                    name: "privateIp".into(),
                    value: &private_ip_binding,
                },
                register_interface::ObjectField {
                    name: "rootBlockDevice".into(),
                    value: &root_block_device_binding,
                },
                register_interface::ObjectField {
                    name: "secondaryPrivateIps".into(),
                    value: &secondary_private_ips_binding,
                },
                register_interface::ObjectField {
                    name: "securityGroups".into(),
                    value: &security_groups_binding,
                },
                register_interface::ObjectField {
                    name: "sourceDestCheck".into(),
                    value: &source_dest_check_binding,
                },
                register_interface::ObjectField {
                    name: "subnetId".into(),
                    value: &subnet_id_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "tenancy".into(),
                    value: &tenancy_binding,
                },
                register_interface::ObjectField {
                    name: "userData".into(),
                    value: &user_data_binding,
                },
                register_interface::ObjectField {
                    name: "userDataBase64".into(),
                    value: &user_data_base64_binding,
                },
                register_interface::ObjectField {
                    name: "userDataReplaceOnChange".into(),
                    value: &user_data_replace_on_change_binding,
                },
                register_interface::ObjectField {
                    name: "volumeTags".into(),
                    value: &volume_tags_binding,
                },
                register_interface::ObjectField {
                    name: "vpcSecurityGroupIds".into(),
                    value: &vpc_security_group_ids_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        InstanceResult {
            ami: pulumi_wasm_rust::__private::into_domain(o.extract_field("ami")),
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            associate_public_ip_address: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("associatePublicIpAddress"),
            ),
            availability_zone: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("availabilityZone"),
            ),
            capacity_reservation_specification: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("capacityReservationSpecification"),
            ),
            cpu_core_count: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("cpuCoreCount"),
            ),
            cpu_options: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("cpuOptions"),
            ),
            cpu_threads_per_core: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("cpuThreadsPerCore"),
            ),
            credit_specification: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("creditSpecification"),
            ),
            disable_api_stop: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("disableApiStop"),
            ),
            disable_api_termination: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("disableApiTermination"),
            ),
            ebs_block_devices: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("ebsBlockDevices"),
            ),
            ebs_optimized: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("ebsOptimized"),
            ),
            enable_primary_ipv6: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("enablePrimaryIpv6"),
            ),
            enclave_options: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("enclaveOptions"),
            ),
            ephemeral_block_devices: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("ephemeralBlockDevices"),
            ),
            get_password_data: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("getPasswordData"),
            ),
            hibernation: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("hibernation"),
            ),
            host_id: pulumi_wasm_rust::__private::into_domain(o.extract_field("hostId")),
            host_resource_group_arn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("hostResourceGroupArn"),
            ),
            iam_instance_profile: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("iamInstanceProfile"),
            ),
            instance_initiated_shutdown_behavior: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("instanceInitiatedShutdownBehavior"),
            ),
            instance_lifecycle: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("instanceLifecycle"),
            ),
            instance_market_options: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("instanceMarketOptions"),
            ),
            instance_state: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("instanceState"),
            ),
            instance_type: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("instanceType"),
            ),
            ipv6_address_count: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("ipv6AddressCount"),
            ),
            ipv6_addresses: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("ipv6Addresses"),
            ),
            key_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("keyName"),
            ),
            launch_template: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("launchTemplate"),
            ),
            maintenance_options: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("maintenanceOptions"),
            ),
            metadata_options: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("metadataOptions"),
            ),
            monitoring: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("monitoring"),
            ),
            network_interfaces: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("networkInterfaces"),
            ),
            outpost_arn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("outpostArn"),
            ),
            password_data: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("passwordData"),
            ),
            placement_group: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("placementGroup"),
            ),
            placement_partition_number: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("placementPartitionNumber"),
            ),
            primary_network_interface_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("primaryNetworkInterfaceId"),
            ),
            private_dns: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("privateDns"),
            ),
            private_dns_name_options: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("privateDnsNameOptions"),
            ),
            private_ip: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("privateIp"),
            ),
            public_dns: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("publicDns"),
            ),
            public_ip: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("publicIp"),
            ),
            root_block_device: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("rootBlockDevice"),
            ),
            secondary_private_ips: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("secondaryPrivateIps"),
            ),
            security_groups: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("securityGroups"),
            ),
            source_dest_check: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("sourceDestCheck"),
            ),
            spot_instance_request_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("spotInstanceRequestId"),
            ),
            subnet_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("subnetId"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            tenancy: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("tenancy"),
            ),
            user_data: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("userData"),
            ),
            user_data_base64: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("userDataBase64"),
            ),
            user_data_replace_on_change: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("userDataReplaceOnChange"),
            ),
            volume_tags: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("volumeTags"),
            ),
            vpc_security_group_ids: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("vpcSecurityGroupIds"),
            ),
        }
    }
}
