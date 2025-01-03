pub mod get_mount_target {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetMountTargetArgs {
        /// ID or ARN of the access point whose mount target that you want to find. It must be included if a `file_system_id` and `mount_target_id` are not included.
        #[builder(into, default)]
        pub access_point_id: pulumi_wasm_rust::Output<Option<String>>,
        /// ID or ARN of the file system whose mount target that you want to find. It must be included if an `access_point_id` and `mount_target_id` are not included.
        #[builder(into, default)]
        pub file_system_id: pulumi_wasm_rust::Output<Option<String>>,
        /// ID or ARN of the mount target that you want to find. It must be included in your request if an `access_point_id` and `file_system_id` are not included.
        #[builder(into, default)]
        pub mount_target_id: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetMountTargetResult {
        pub access_point_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The unique and consistent identifier of the Availability Zone (AZ) that the mount target resides in.
        pub availability_zone_id: pulumi_wasm_rust::Output<String>,
        /// The name of the Availability Zone (AZ) that the mount target resides in.
        pub availability_zone_name: pulumi_wasm_rust::Output<String>,
        /// DNS name for the EFS file system.
        pub dns_name: pulumi_wasm_rust::Output<String>,
        /// Amazon Resource Name of the file system for which the mount target is intended.
        pub file_system_arn: pulumi_wasm_rust::Output<String>,
        pub file_system_id: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// Address at which the file system may be mounted via the mount target.
        pub ip_address: pulumi_wasm_rust::Output<String>,
        /// The DNS name for the given subnet/AZ per [documented convention](http://docs.aws.amazon.com/efs/latest/ug/mounting-fs-mount-cmd-dns-name.html).
        pub mount_target_dns_name: pulumi_wasm_rust::Output<String>,
        pub mount_target_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the network interface that Amazon EFS created when it created the mount target.
        pub network_interface_id: pulumi_wasm_rust::Output<String>,
        /// AWS account ID that owns the resource.
        pub owner_id: pulumi_wasm_rust::Output<String>,
        /// List of VPC security group IDs attached to the mount target.
        pub security_groups: pulumi_wasm_rust::Output<Vec<String>>,
        /// ID of the mount target's subnet.
        pub subnet_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetMountTargetArgs) -> GetMountTargetResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let access_point_id_binding = args.access_point_id.get_inner();
        let file_system_id_binding = args.file_system_id.get_inner();
        let mount_target_id_binding = args.mount_target_id.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:efs/getMountTarget:getMountTarget".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accessPointId".into(),
                    value: &access_point_id_binding,
                },
                register_interface::ObjectField {
                    name: "fileSystemId".into(),
                    value: &file_system_id_binding,
                },
                register_interface::ObjectField {
                    name: "mountTargetId".into(),
                    value: &mount_target_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accessPointId".into(),
                },
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
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "ipAddress".into(),
                },
                register_interface::ResultField {
                    name: "mountTargetDnsName".into(),
                },
                register_interface::ResultField {
                    name: "mountTargetId".into(),
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
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetMountTargetResult {
            access_point_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accessPointId").unwrap(),
            ),
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
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            ip_address: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipAddress").unwrap(),
            ),
            mount_target_dns_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("mountTargetDnsName").unwrap(),
            ),
            mount_target_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("mountTargetId").unwrap(),
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
