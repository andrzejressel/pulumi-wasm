pub mod get_instance {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetInstanceArgs {
        /// One or more name/value pairs to use as filters. There are
        /// several valid keys, for a full reference, check out
        /// [describe-instances in the AWS CLI reference][1].
        #[builder(into, default)]
        pub filters: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::super::types::ec2::GetInstanceFilter>>,
        >,
        /// If true, wait for password data to become available and retrieve it. Useful for getting the administrator password for instances running Microsoft Windows. The password data is exported to the `password_data` attribute. See [GetPasswordData](https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_GetPasswordData.html) for more information.
        #[builder(into, default)]
        pub get_password_data: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Retrieve Base64 encoded User Data contents into the `user_data_base64` attribute. A SHA-1 hash of the User Data contents will always be present in the `user_data` attribute. Defaults to `false`.
        ///
        /// > **NOTE:** At least one of `filter`, `instance_tags`, or `instance_id` must be specified.
        ///
        /// > **NOTE:** If anything other than a single match is returned by the search,
        /// this call will fail. Ensure that your search is specific enough to return
        /// a single Instance ID only.
        #[builder(into, default)]
        pub get_user_data: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Specify the exact Instance ID with which to populate the data source.
        #[builder(into, default)]
        pub instance_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Map of tags, each pair of which must
        /// exactly match a pair on the desired Instance.
        #[builder(into, default)]
        pub instance_tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the Instance.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetInstanceResult {
        /// ID of the AMI used to launch the instance.
        pub ami: pulumi_wasm_rust::Output<String>,
        /// ARN of the instance.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Whether or not the Instance is associated with a public IP address or not (Boolean).
        pub associate_public_ip_address: pulumi_wasm_rust::Output<bool>,
        /// Availability zone of the Instance.
        pub availability_zone: pulumi_wasm_rust::Output<String>,
        /// Credit specification of the Instance.
        pub credit_specifications: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::ec2::GetInstanceCreditSpecification>,
        >,
        /// Whether or not EC2 Instance Stop Protection](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/Stop_Start.html#Using_StopProtection) is enabled (Boolean).
        pub disable_api_stop: pulumi_wasm_rust::Output<bool>,
        /// Whether or not [EC2 Instance Termination Protection](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/terminating-instances.html#Using_ChangingDisableAPITermination) is enabled (Boolean).
        pub disable_api_termination: pulumi_wasm_rust::Output<bool>,
        /// EBS block device mappings of the Instance.
        pub ebs_block_devices: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::ec2::GetInstanceEbsBlockDevice>,
        >,
        /// Whether the Instance is EBS optimized or not (Boolean).
        pub ebs_optimized: pulumi_wasm_rust::Output<bool>,
        /// Enclave options of the instance.
        pub enclave_options: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::ec2::GetInstanceEnclaveOption>,
        >,
        /// Ephemeral block device mappings of the Instance.
        pub ephemeral_block_devices: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::ec2::GetInstanceEphemeralBlockDevice>,
        >,
        pub filters: pulumi_wasm_rust::Output<
            Option<Vec<super::super::super::types::ec2::GetInstanceFilter>>,
        >,
        pub get_password_data: pulumi_wasm_rust::Output<Option<bool>>,
        pub get_user_data: pulumi_wasm_rust::Output<Option<bool>>,
        /// ID of the dedicated host the instance will be assigned to.
        pub host_id: pulumi_wasm_rust::Output<String>,
        /// ARN of the host resource group the instance is associated with.
        pub host_resource_group_arn: pulumi_wasm_rust::Output<String>,
        /// Name of the instance profile associated with the Instance.
        pub iam_instance_profile: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub instance_id: pulumi_wasm_rust::Output<Option<String>>,
        /// State of the instance. One of: `pending`, `running`, `shutting-down`, `terminated`, `stopping`, `stopped`. See [Instance Lifecycle](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-instance-lifecycle.html) for more information.
        pub instance_state: pulumi_wasm_rust::Output<String>,
        pub instance_tags: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Type of the Instance.
        pub instance_type: pulumi_wasm_rust::Output<String>,
        /// IPv6 addresses associated to the Instance, if applicable. **NOTE**: Unlike the IPv4 address, this doesn't change if you attach an EIP to the instance.
        pub ipv6_addresses: pulumi_wasm_rust::Output<Vec<String>>,
        /// Key name of the Instance.
        pub key_name: pulumi_wasm_rust::Output<String>,
        /// Time the instance was launched.
        pub launch_time: pulumi_wasm_rust::Output<String>,
        /// Maintenance and recovery options for the instance.
        pub maintenance_options: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::ec2::GetInstanceMaintenanceOption>,
        >,
        /// Metadata options of the Instance.
        pub metadata_options: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::ec2::GetInstanceMetadataOption>,
        >,
        /// Whether detailed monitoring is enabled or disabled for the Instance (Boolean).
        pub monitoring: pulumi_wasm_rust::Output<bool>,
        /// ID of the network interface that was created with the Instance.
        pub network_interface_id: pulumi_wasm_rust::Output<String>,
        /// ARN of the Outpost.
        pub outpost_arn: pulumi_wasm_rust::Output<String>,
        /// Base-64 encoded encrypted password data for the instance. Useful for getting the administrator password for instances running Microsoft Windows. This attribute is only exported if `get_password_data` is true. See [GetPasswordData](https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_GetPasswordData.html) for more information.
        pub password_data: pulumi_wasm_rust::Output<String>,
        /// Placement group of the Instance.
        pub placement_group: pulumi_wasm_rust::Output<String>,
        /// Number of the partition the instance is in.
        pub placement_partition_number: pulumi_wasm_rust::Output<i32>,
        /// Private DNS name assigned to the Instance. Can only be used inside the Amazon EC2, and only available if you've enabled DNS hostnames for your VPC.
        pub private_dns: pulumi_wasm_rust::Output<String>,
        /// Options for the instance hostname.
        pub private_dns_name_options: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::ec2::GetInstancePrivateDnsNameOption>,
        >,
        /// Private IP address assigned to the Instance.
        pub private_ip: pulumi_wasm_rust::Output<String>,
        /// Public DNS name assigned to the Instance. For EC2-VPC, this is only available if you've enabled DNS hostnames for your VPC.
        pub public_dns: pulumi_wasm_rust::Output<String>,
        /// Public IP address assigned to the Instance, if applicable. **NOTE**: If you are using an `aws.ec2.Eip` with your instance, you should refer to the EIP's address directly and not use `public_ip`, as this field will change after the EIP is attached.
        pub public_ip: pulumi_wasm_rust::Output<String>,
        /// Root block device mappings of the Instance
        pub root_block_devices: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::ec2::GetInstanceRootBlockDevice>,
        >,
        /// Secondary private IPv4 addresses assigned to the instance's primary network interface (eth0) in a VPC.
        pub secondary_private_ips: pulumi_wasm_rust::Output<Vec<String>>,
        /// Associated security groups.
        pub security_groups: pulumi_wasm_rust::Output<Vec<String>>,
        /// Whether the network interface performs source/destination checking (Boolean).
        pub source_dest_check: pulumi_wasm_rust::Output<bool>,
        /// VPC subnet ID.
        pub subnet_id: pulumi_wasm_rust::Output<String>,
        /// Map of tags assigned to the Instance.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// Tenancy of the instance: `dedicated`, `default`, `host`.
        pub tenancy: pulumi_wasm_rust::Output<String>,
        /// SHA-1 hash of User Data supplied to the Instance.
        pub user_data: pulumi_wasm_rust::Output<String>,
        /// Base64 encoded contents of User Data supplied to the Instance. This attribute is only exported if `get_user_data` is true.
        pub user_data_base64: pulumi_wasm_rust::Output<String>,
        /// Associated security groups in a non-default VPC.
        pub vpc_security_group_ids: pulumi_wasm_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetInstanceArgs,
    ) -> GetInstanceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let filters_binding = args.filters.get_output(context).get_inner();
        let get_password_data_binding = args
            .get_password_data
            .get_output(context)
            .get_inner();
        let get_user_data_binding = args.get_user_data.get_output(context).get_inner();
        let instance_id_binding = args.instance_id.get_output(context).get_inner();
        let instance_tags_binding = args.instance_tags.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:ec2/getInstance:getInstance".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "filters".into(),
                    value: &filters_binding,
                },
                register_interface::ObjectField {
                    name: "getPasswordData".into(),
                    value: &get_password_data_binding,
                },
                register_interface::ObjectField {
                    name: "getUserData".into(),
                    value: &get_user_data_binding,
                },
                register_interface::ObjectField {
                    name: "instanceId".into(),
                    value: &instance_id_binding,
                },
                register_interface::ObjectField {
                    name: "instanceTags".into(),
                    value: &instance_tags_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "ami".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "associatePublicIpAddress".into(),
                },
                register_interface::ResultField {
                    name: "availabilityZone".into(),
                },
                register_interface::ResultField {
                    name: "creditSpecifications".into(),
                },
                register_interface::ResultField {
                    name: "disableApiStop".into(),
                },
                register_interface::ResultField {
                    name: "disableApiTermination".into(),
                },
                register_interface::ResultField {
                    name: "ebsBlockDevices".into(),
                },
                register_interface::ResultField {
                    name: "ebsOptimized".into(),
                },
                register_interface::ResultField {
                    name: "enclaveOptions".into(),
                },
                register_interface::ResultField {
                    name: "ephemeralBlockDevices".into(),
                },
                register_interface::ResultField {
                    name: "filters".into(),
                },
                register_interface::ResultField {
                    name: "getPasswordData".into(),
                },
                register_interface::ResultField {
                    name: "getUserData".into(),
                },
                register_interface::ResultField {
                    name: "hostId".into(),
                },
                register_interface::ResultField {
                    name: "hostResourceGroupArn".into(),
                },
                register_interface::ResultField {
                    name: "iamInstanceProfile".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "instanceId".into(),
                },
                register_interface::ResultField {
                    name: "instanceState".into(),
                },
                register_interface::ResultField {
                    name: "instanceTags".into(),
                },
                register_interface::ResultField {
                    name: "instanceType".into(),
                },
                register_interface::ResultField {
                    name: "ipv6Addresses".into(),
                },
                register_interface::ResultField {
                    name: "keyName".into(),
                },
                register_interface::ResultField {
                    name: "launchTime".into(),
                },
                register_interface::ResultField {
                    name: "maintenanceOptions".into(),
                },
                register_interface::ResultField {
                    name: "metadataOptions".into(),
                },
                register_interface::ResultField {
                    name: "monitoring".into(),
                },
                register_interface::ResultField {
                    name: "networkInterfaceId".into(),
                },
                register_interface::ResultField {
                    name: "outpostArn".into(),
                },
                register_interface::ResultField {
                    name: "passwordData".into(),
                },
                register_interface::ResultField {
                    name: "placementGroup".into(),
                },
                register_interface::ResultField {
                    name: "placementPartitionNumber".into(),
                },
                register_interface::ResultField {
                    name: "privateDns".into(),
                },
                register_interface::ResultField {
                    name: "privateDnsNameOptions".into(),
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
                    name: "rootBlockDevices".into(),
                },
                register_interface::ResultField {
                    name: "secondaryPrivateIps".into(),
                },
                register_interface::ResultField {
                    name: "securityGroups".into(),
                },
                register_interface::ResultField {
                    name: "sourceDestCheck".into(),
                },
                register_interface::ResultField {
                    name: "subnetId".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tenancy".into(),
                },
                register_interface::ResultField {
                    name: "userData".into(),
                },
                register_interface::ResultField {
                    name: "userDataBase64".into(),
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
        GetInstanceResult {
            ami: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ami").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            associate_public_ip_address: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("associatePublicIpAddress").unwrap(),
            ),
            availability_zone: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("availabilityZone").unwrap(),
            ),
            credit_specifications: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("creditSpecifications").unwrap(),
            ),
            disable_api_stop: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("disableApiStop").unwrap(),
            ),
            disable_api_termination: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("disableApiTermination").unwrap(),
            ),
            ebs_block_devices: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ebsBlockDevices").unwrap(),
            ),
            ebs_optimized: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ebsOptimized").unwrap(),
            ),
            enclave_options: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enclaveOptions").unwrap(),
            ),
            ephemeral_block_devices: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ephemeralBlockDevices").unwrap(),
            ),
            filters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("filters").unwrap(),
            ),
            get_password_data: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("getPasswordData").unwrap(),
            ),
            get_user_data: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("getUserData").unwrap(),
            ),
            host_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hostId").unwrap(),
            ),
            host_resource_group_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hostResourceGroupArn").unwrap(),
            ),
            iam_instance_profile: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("iamInstanceProfile").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            instance_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instanceId").unwrap(),
            ),
            instance_state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instanceState").unwrap(),
            ),
            instance_tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instanceTags").unwrap(),
            ),
            instance_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instanceType").unwrap(),
            ),
            ipv6_addresses: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipv6Addresses").unwrap(),
            ),
            key_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("keyName").unwrap(),
            ),
            launch_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("launchTime").unwrap(),
            ),
            maintenance_options: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maintenanceOptions").unwrap(),
            ),
            metadata_options: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("metadataOptions").unwrap(),
            ),
            monitoring: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("monitoring").unwrap(),
            ),
            network_interface_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("networkInterfaceId").unwrap(),
            ),
            outpost_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("outpostArn").unwrap(),
            ),
            password_data: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("passwordData").unwrap(),
            ),
            placement_group: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("placementGroup").unwrap(),
            ),
            placement_partition_number: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("placementPartitionNumber").unwrap(),
            ),
            private_dns: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("privateDns").unwrap(),
            ),
            private_dns_name_options: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("privateDnsNameOptions").unwrap(),
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
            root_block_devices: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("rootBlockDevices").unwrap(),
            ),
            secondary_private_ips: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secondaryPrivateIps").unwrap(),
            ),
            security_groups: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("securityGroups").unwrap(),
            ),
            source_dest_check: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceDestCheck").unwrap(),
            ),
            subnet_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subnetId").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tenancy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tenancy").unwrap(),
            ),
            user_data: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("userData").unwrap(),
            ),
            user_data_base64: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("userDataBase64").unwrap(),
            ),
            vpc_security_group_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpcSecurityGroupIds").unwrap(),
            ),
        }
    }
}
