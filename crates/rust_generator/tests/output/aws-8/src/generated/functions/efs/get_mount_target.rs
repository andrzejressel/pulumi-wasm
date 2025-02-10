#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_mount_target {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetMountTargetArgs {
        /// ID or ARN of the access point whose mount target that you want to find. It must be included if a `file_system_id` and `mount_target_id` are not included.
        #[builder(into, default)]
        pub access_point_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// ID or ARN of the file system whose mount target that you want to find. It must be included if an `access_point_id` and `mount_target_id` are not included.
        #[builder(into, default)]
        pub file_system_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// ID or ARN of the mount target that you want to find. It must be included in your request if an `access_point_id` and `file_system_id` are not included.
        #[builder(into, default)]
        pub mount_target_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetMountTargetResult {
        pub access_point_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The unique and consistent identifier of the Availability Zone (AZ) that the mount target resides in.
        pub availability_zone_id: pulumi_gestalt_rust::Output<String>,
        /// The name of the Availability Zone (AZ) that the mount target resides in.
        pub availability_zone_name: pulumi_gestalt_rust::Output<String>,
        /// DNS name for the EFS file system.
        pub dns_name: pulumi_gestalt_rust::Output<String>,
        /// Amazon Resource Name of the file system for which the mount target is intended.
        pub file_system_arn: pulumi_gestalt_rust::Output<String>,
        pub file_system_id: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Address at which the file system may be mounted via the mount target.
        pub ip_address: pulumi_gestalt_rust::Output<String>,
        /// The DNS name for the given subnet/AZ per [documented convention](http://docs.aws.amazon.com/efs/latest/ug/mounting-fs-mount-cmd-dns-name.html).
        pub mount_target_dns_name: pulumi_gestalt_rust::Output<String>,
        pub mount_target_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the network interface that Amazon EFS created when it created the mount target.
        pub network_interface_id: pulumi_gestalt_rust::Output<String>,
        /// AWS account ID that owns the resource.
        pub owner_id: pulumi_gestalt_rust::Output<String>,
        /// List of VPC security group IDs attached to the mount target.
        pub security_groups: pulumi_gestalt_rust::Output<Vec<String>>,
        /// ID of the mount target's subnet.
        pub subnet_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetMountTargetArgs,
    ) -> GetMountTargetResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let access_point_id_binding = args.access_point_id.get_output(context);
        let file_system_id_binding = args.file_system_id.get_output(context);
        let mount_target_id_binding = args.mount_target_id.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:efs/getMountTarget:getMountTarget".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accessPointId".into(),
                    value: access_point_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "fileSystemId".into(),
                    value: file_system_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "mountTargetId".into(),
                    value: mount_target_id_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetMountTargetResult {
            access_point_id: o.get_field("accessPointId"),
            availability_zone_id: o.get_field("availabilityZoneId"),
            availability_zone_name: o.get_field("availabilityZoneName"),
            dns_name: o.get_field("dnsName"),
            file_system_arn: o.get_field("fileSystemArn"),
            file_system_id: o.get_field("fileSystemId"),
            id: o.get_field("id"),
            ip_address: o.get_field("ipAddress"),
            mount_target_dns_name: o.get_field("mountTargetDnsName"),
            mount_target_id: o.get_field("mountTargetId"),
            network_interface_id: o.get_field("networkInterfaceId"),
            owner_id: o.get_field("ownerId"),
            security_groups: o.get_field("securityGroups"),
            subnet_id: o.get_field("subnetId"),
        }
    }
}
