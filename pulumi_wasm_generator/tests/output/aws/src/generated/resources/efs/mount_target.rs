/// Provides an Elastic File System (EFS) mount target.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let alpha = mount_target::create(
///         "alpha",
///         MountTargetArgs::builder()
///             .file_system_id("${fooAwsEfsFileSystem.id}")
///             .subnet_id("${alphaSubnet.id}")
///             .build_struct(),
///     );
///     let alphaSubnet = subnet::create(
///         "alphaSubnet",
///         SubnetArgs::builder()
///             .availability_zone("us-west-2a")
///             .cidr_block("10.0.1.0/24")
///             .vpc_id("${foo.id}")
///             .build_struct(),
///     );
///     let foo = vpc::create(
///         "foo",
///         VpcArgs::builder().cidr_block("10.0.0.0/16").build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import the EFS mount targets using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:efs/mountTarget:MountTarget alpha fsmt-52a643fb
/// ```
pub mod mount_target {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MountTargetArgs {
        /// The ID of the file system for which the mount target is intended.
        #[builder(into)]
        pub file_system_id: pulumi_wasm_rust::Output<String>,
        /// The address (within the address range of the specified subnet) at
        /// which the file system may be mounted via the mount target.
        #[builder(into, default)]
        pub ip_address: pulumi_wasm_rust::Output<Option<String>>,
        /// A list of up to 5 VPC security group IDs (that must
        /// be for the same VPC as subnet specified) in effect for the mount target.
        #[builder(into, default)]
        pub security_groups: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The ID of the subnet to add the mount target in.
        #[builder(into)]
        pub subnet_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct MountTargetResult {
        /// The unique and consistent identifier of the Availability Zone (AZ) that the mount target resides in.
        pub availability_zone_id: pulumi_wasm_rust::Output<String>,
        /// The name of the Availability Zone (AZ) that the mount target resides in.
        pub availability_zone_name: pulumi_wasm_rust::Output<String>,
        /// The DNS name for the EFS file system.
        pub dns_name: pulumi_wasm_rust::Output<String>,
        /// Amazon Resource Name of the file system.
        pub file_system_arn: pulumi_wasm_rust::Output<String>,
        /// The ID of the file system for which the mount target is intended.
        pub file_system_id: pulumi_wasm_rust::Output<String>,
        /// The address (within the address range of the specified subnet) at
        /// which the file system may be mounted via the mount target.
        pub ip_address: pulumi_wasm_rust::Output<String>,
        /// The DNS name for the given subnet/AZ per [documented convention](http://docs.aws.amazon.com/efs/latest/ug/mounting-fs-mount-cmd-dns-name.html).
        pub mount_target_dns_name: pulumi_wasm_rust::Output<String>,
        /// The ID of the network interface that Amazon EFS created when it created the mount target.
        pub network_interface_id: pulumi_wasm_rust::Output<String>,
        /// AWS account ID that owns the resource.
        pub owner_id: pulumi_wasm_rust::Output<String>,
        /// A list of up to 5 VPC security group IDs (that must
        /// be for the same VPC as subnet specified) in effect for the mount target.
        pub security_groups: pulumi_wasm_rust::Output<Vec<String>>,
        /// The ID of the subnet to add the mount target in.
        pub subnet_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: MountTargetArgs) -> MountTargetResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let file_system_id_binding = args.file_system_id.get_inner();
        let ip_address_binding = args.ip_address.get_inner();
        let security_groups_binding = args.security_groups.get_inner();
        let subnet_id_binding = args.subnet_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:efs/mountTarget:MountTarget".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "fileSystemId".into(),
                    value: &file_system_id_binding,
                },
                register_interface::ObjectField {
                    name: "ipAddress".into(),
                    value: &ip_address_binding,
                },
                register_interface::ObjectField {
                    name: "securityGroups".into(),
                    value: &security_groups_binding,
                },
                register_interface::ObjectField {
                    name: "subnetId".into(),
                    value: &subnet_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "availabilityZoneId".into(),
                },
                register_interface::ResultField {
                    name: "availabilityZoneName".into(),
                },
                register_interface::ResultField {
                    name: "dnsName".into(),
                },
                register_interface::ResultField {
                    name: "fileSystemArn".into(),
                },
                register_interface::ResultField {
                    name: "fileSystemId".into(),
                },
                register_interface::ResultField {
                    name: "ipAddress".into(),
                },
                register_interface::ResultField {
                    name: "mountTargetDnsName".into(),
                },
                register_interface::ResultField {
                    name: "networkInterfaceId".into(),
                },
                register_interface::ResultField {
                    name: "ownerId".into(),
                },
                register_interface::ResultField {
                    name: "securityGroups".into(),
                },
                register_interface::ResultField {
                    name: "subnetId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        MountTargetResult {
            availability_zone_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("availabilityZoneId").unwrap(),
            ),
            availability_zone_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("availabilityZoneName").unwrap(),
            ),
            dns_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dnsName").unwrap(),
            ),
            file_system_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("fileSystemArn").unwrap(),
            ),
            file_system_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("fileSystemId").unwrap(),
            ),
            ip_address: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipAddress").unwrap(),
            ),
            mount_target_dns_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("mountTargetDnsName").unwrap(),
            ),
            network_interface_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("networkInterfaceId").unwrap(),
            ),
            owner_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ownerId").unwrap(),
            ),
            security_groups: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("securityGroups").unwrap(),
            ),
            subnet_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subnetId").unwrap(),
            ),
        }
    }
}
