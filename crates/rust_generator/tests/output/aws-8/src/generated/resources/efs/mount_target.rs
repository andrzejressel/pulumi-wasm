/// Provides an Elastic File System (EFS) mount target.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod mount_target {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MountTargetArgs {
        /// The ID of the file system for which the mount target is intended.
        #[builder(into)]
        pub file_system_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The address (within the address range of the specified subnet) at
        /// which the file system may be mounted via the mount target.
        #[builder(into, default)]
        pub ip_address: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A list of up to 5 VPC security group IDs (that must
        /// be for the same VPC as subnet specified) in effect for the mount target.
        #[builder(into, default)]
        pub security_groups: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The ID of the subnet to add the mount target in.
        #[builder(into)]
        pub subnet_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct MountTargetResult {
        /// The unique and consistent identifier of the Availability Zone (AZ) that the mount target resides in.
        pub availability_zone_id: pulumi_gestalt_rust::Output<String>,
        /// The name of the Availability Zone (AZ) that the mount target resides in.
        pub availability_zone_name: pulumi_gestalt_rust::Output<String>,
        /// The DNS name for the EFS file system.
        pub dns_name: pulumi_gestalt_rust::Output<String>,
        /// Amazon Resource Name of the file system.
        pub file_system_arn: pulumi_gestalt_rust::Output<String>,
        /// The ID of the file system for which the mount target is intended.
        pub file_system_id: pulumi_gestalt_rust::Output<String>,
        /// The address (within the address range of the specified subnet) at
        /// which the file system may be mounted via the mount target.
        pub ip_address: pulumi_gestalt_rust::Output<String>,
        /// The DNS name for the given subnet/AZ per [documented convention](http://docs.aws.amazon.com/efs/latest/ug/mounting-fs-mount-cmd-dns-name.html).
        pub mount_target_dns_name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the network interface that Amazon EFS created when it created the mount target.
        pub network_interface_id: pulumi_gestalt_rust::Output<String>,
        /// AWS account ID that owns the resource.
        pub owner_id: pulumi_gestalt_rust::Output<String>,
        /// A list of up to 5 VPC security group IDs (that must
        /// be for the same VPC as subnet specified) in effect for the mount target.
        pub security_groups: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The ID of the subnet to add the mount target in.
        pub subnet_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: MountTargetArgs,
    ) -> MountTargetResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let file_system_id_binding = args.file_system_id.get_output(context);
        let ip_address_binding = args.ip_address.get_output(context);
        let security_groups_binding = args.security_groups.get_output(context);
        let subnet_id_binding = args.subnet_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:efs/mountTarget:MountTarget".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "fileSystemId".into(),
                    value: file_system_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ipAddress".into(),
                    value: ip_address_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "securityGroups".into(),
                    value: security_groups_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "subnetId".into(),
                    value: subnet_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        MountTargetResult {
            availability_zone_id: o.get_field("availabilityZoneId"),
            availability_zone_name: o.get_field("availabilityZoneName"),
            dns_name: o.get_field("dnsName"),
            file_system_arn: o.get_field("fileSystemArn"),
            file_system_id: o.get_field("fileSystemId"),
            ip_address: o.get_field("ipAddress"),
            mount_target_dns_name: o.get_field("mountTargetDnsName"),
            network_interface_id: o.get_field("networkInterfaceId"),
            owner_id: o.get_field("ownerId"),
            security_groups: o.get_field("securityGroups"),
            subnet_id: o.get_field("subnetId"),
        }
    }
}
