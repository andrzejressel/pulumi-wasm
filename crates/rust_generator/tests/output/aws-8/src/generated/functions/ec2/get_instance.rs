#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_instance {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetInstanceArgs {
        /// One or more name/value pairs to use as filters. There are
        /// several valid keys, for a full reference, check out
        /// [describe-instances in the AWS CLI reference][1].
        #[builder(into, default)]
        pub filters: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::super::types::ec2::GetInstanceFilter>>,
        >,
        /// If true, wait for password data to become available and retrieve it. Useful for getting the administrator password for instances running Microsoft Windows. The password data is exported to the `password_data` attribute. See [GetPasswordData](https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_GetPasswordData.html) for more information.
        #[builder(into, default)]
        pub get_password_data: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Retrieve Base64 encoded User Data contents into the `user_data_base64` attribute. A SHA-1 hash of the User Data contents will always be present in the `user_data` attribute. Defaults to `false`.
        ///
        /// > **NOTE:** At least one of `filter`, `instance_tags`, or `instance_id` must be specified.
        ///
        /// > **NOTE:** If anything other than a single match is returned by the search,
        /// this call will fail. Ensure that your search is specific enough to return
        /// a single Instance ID only.
        #[builder(into, default)]
        pub get_user_data: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Specify the exact Instance ID with which to populate the data source.
        #[builder(into, default)]
        pub instance_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Map of tags, each pair of which must
        /// exactly match a pair on the desired Instance.
        #[builder(into, default)]
        pub instance_tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the Instance.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetInstanceResult {
        /// ID of the AMI used to launch the instance.
        pub ami: pulumi_gestalt_rust::Output<String>,
        /// ARN of the instance.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Whether or not the Instance is associated with a public IP address or not (Boolean).
        pub associate_public_ip_address: pulumi_gestalt_rust::Output<bool>,
        /// Availability zone of the Instance.
        pub availability_zone: pulumi_gestalt_rust::Output<String>,
        /// Credit specification of the Instance.
        pub credit_specifications: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::ec2::GetInstanceCreditSpecification>,
        >,
        /// Whether or not EC2 Instance Stop Protection](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/Stop_Start.html#Using_StopProtection) is enabled (Boolean).
        pub disable_api_stop: pulumi_gestalt_rust::Output<bool>,
        /// Whether or not [EC2 Instance Termination Protection](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/terminating-instances.html#Using_ChangingDisableAPITermination) is enabled (Boolean).
        pub disable_api_termination: pulumi_gestalt_rust::Output<bool>,
        /// EBS block device mappings of the Instance.
        pub ebs_block_devices: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::ec2::GetInstanceEbsBlockDevice>,
        >,
        /// Whether the Instance is EBS optimized or not (Boolean).
        pub ebs_optimized: pulumi_gestalt_rust::Output<bool>,
        /// Enclave options of the instance.
        pub enclave_options: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::ec2::GetInstanceEnclaveOption>,
        >,
        /// Ephemeral block device mappings of the Instance.
        pub ephemeral_block_devices: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::ec2::GetInstanceEphemeralBlockDevice>,
        >,
        pub filters: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::super::types::ec2::GetInstanceFilter>>,
        >,
        pub get_password_data: pulumi_gestalt_rust::Output<Option<bool>>,
        pub get_user_data: pulumi_gestalt_rust::Output<Option<bool>>,
        /// ID of the dedicated host the instance will be assigned to.
        pub host_id: pulumi_gestalt_rust::Output<String>,
        /// ARN of the host resource group the instance is associated with.
        pub host_resource_group_arn: pulumi_gestalt_rust::Output<String>,
        /// Name of the instance profile associated with the Instance.
        pub iam_instance_profile: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub instance_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// State of the instance. One of: `pending`, `running`, `shutting-down`, `terminated`, `stopping`, `stopped`. See [Instance Lifecycle](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-instance-lifecycle.html) for more information.
        pub instance_state: pulumi_gestalt_rust::Output<String>,
        pub instance_tags: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Type of the Instance.
        pub instance_type: pulumi_gestalt_rust::Output<String>,
        /// IPv6 addresses associated to the Instance, if applicable. **NOTE**: Unlike the IPv4 address, this doesn't change if you attach an EIP to the instance.
        pub ipv6_addresses: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Key name of the Instance.
        pub key_name: pulumi_gestalt_rust::Output<String>,
        /// Time the instance was launched.
        pub launch_time: pulumi_gestalt_rust::Output<String>,
        /// Maintenance and recovery options for the instance.
        pub maintenance_options: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::ec2::GetInstanceMaintenanceOption>,
        >,
        /// Metadata options of the Instance.
        pub metadata_options: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::ec2::GetInstanceMetadataOption>,
        >,
        /// Whether detailed monitoring is enabled or disabled for the Instance (Boolean).
        pub monitoring: pulumi_gestalt_rust::Output<bool>,
        /// ID of the network interface that was created with the Instance.
        pub network_interface_id: pulumi_gestalt_rust::Output<String>,
        /// ARN of the Outpost.
        pub outpost_arn: pulumi_gestalt_rust::Output<String>,
        /// Base-64 encoded encrypted password data for the instance. Useful for getting the administrator password for instances running Microsoft Windows. This attribute is only exported if `get_password_data` is true. See [GetPasswordData](https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_GetPasswordData.html) for more information.
        pub password_data: pulumi_gestalt_rust::Output<String>,
        /// Placement group of the Instance.
        pub placement_group: pulumi_gestalt_rust::Output<String>,
        /// Number of the partition the instance is in.
        pub placement_partition_number: pulumi_gestalt_rust::Output<i32>,
        /// Private DNS name assigned to the Instance. Can only be used inside the Amazon EC2, and only available if you've enabled DNS hostnames for your VPC.
        pub private_dns: pulumi_gestalt_rust::Output<String>,
        /// Options for the instance hostname.
        pub private_dns_name_options: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::ec2::GetInstancePrivateDnsNameOption>,
        >,
        /// Private IP address assigned to the Instance.
        pub private_ip: pulumi_gestalt_rust::Output<String>,
        /// Public DNS name assigned to the Instance. For EC2-VPC, this is only available if you've enabled DNS hostnames for your VPC.
        pub public_dns: pulumi_gestalt_rust::Output<String>,
        /// Public IP address assigned to the Instance, if applicable. **NOTE**: If you are using an `aws.ec2.Eip` with your instance, you should refer to the EIP's address directly and not use `public_ip`, as this field will change after the EIP is attached.
        pub public_ip: pulumi_gestalt_rust::Output<String>,
        /// Root block device mappings of the Instance
        pub root_block_devices: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::ec2::GetInstanceRootBlockDevice>,
        >,
        /// Secondary private IPv4 addresses assigned to the instance's primary network interface (eth0) in a VPC.
        pub secondary_private_ips: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Associated security groups.
        pub security_groups: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Whether the network interface performs source/destination checking (Boolean).
        pub source_dest_check: pulumi_gestalt_rust::Output<bool>,
        /// VPC subnet ID.
        pub subnet_id: pulumi_gestalt_rust::Output<String>,
        /// Map of tags assigned to the Instance.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// Tenancy of the instance: `dedicated`, `default`, `host`.
        pub tenancy: pulumi_gestalt_rust::Output<String>,
        /// SHA-1 hash of User Data supplied to the Instance.
        pub user_data: pulumi_gestalt_rust::Output<String>,
        /// Base64 encoded contents of User Data supplied to the Instance. This attribute is only exported if `get_user_data` is true.
        pub user_data_base64: pulumi_gestalt_rust::Output<String>,
        /// Associated security groups in a non-default VPC.
        pub vpc_security_group_ids: pulumi_gestalt_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetInstanceArgs,
    ) -> GetInstanceResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let filters_binding = args.filters.get_output(context);
        let get_password_data_binding = args.get_password_data.get_output(context);
        let get_user_data_binding = args.get_user_data.get_output(context);
        let instance_id_binding = args.instance_id.get_output(context);
        let instance_tags_binding = args.instance_tags.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:ec2/getInstance:getInstance".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "filters".into(),
                    value: filters_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "getPasswordData".into(),
                    value: get_password_data_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "getUserData".into(),
                    value: get_user_data_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "instanceId".into(),
                    value: instance_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "instanceTags".into(),
                    value: instance_tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetInstanceResult {
            ami: o.get_field("ami"),
            arn: o.get_field("arn"),
            associate_public_ip_address: o.get_field("associatePublicIpAddress"),
            availability_zone: o.get_field("availabilityZone"),
            credit_specifications: o.get_field("creditSpecifications"),
            disable_api_stop: o.get_field("disableApiStop"),
            disable_api_termination: o.get_field("disableApiTermination"),
            ebs_block_devices: o.get_field("ebsBlockDevices"),
            ebs_optimized: o.get_field("ebsOptimized"),
            enclave_options: o.get_field("enclaveOptions"),
            ephemeral_block_devices: o.get_field("ephemeralBlockDevices"),
            filters: o.get_field("filters"),
            get_password_data: o.get_field("getPasswordData"),
            get_user_data: o.get_field("getUserData"),
            host_id: o.get_field("hostId"),
            host_resource_group_arn: o.get_field("hostResourceGroupArn"),
            iam_instance_profile: o.get_field("iamInstanceProfile"),
            id: o.get_field("id"),
            instance_id: o.get_field("instanceId"),
            instance_state: o.get_field("instanceState"),
            instance_tags: o.get_field("instanceTags"),
            instance_type: o.get_field("instanceType"),
            ipv6_addresses: o.get_field("ipv6Addresses"),
            key_name: o.get_field("keyName"),
            launch_time: o.get_field("launchTime"),
            maintenance_options: o.get_field("maintenanceOptions"),
            metadata_options: o.get_field("metadataOptions"),
            monitoring: o.get_field("monitoring"),
            network_interface_id: o.get_field("networkInterfaceId"),
            outpost_arn: o.get_field("outpostArn"),
            password_data: o.get_field("passwordData"),
            placement_group: o.get_field("placementGroup"),
            placement_partition_number: o.get_field("placementPartitionNumber"),
            private_dns: o.get_field("privateDns"),
            private_dns_name_options: o.get_field("privateDnsNameOptions"),
            private_ip: o.get_field("privateIp"),
            public_dns: o.get_field("publicDns"),
            public_ip: o.get_field("publicIp"),
            root_block_devices: o.get_field("rootBlockDevices"),
            secondary_private_ips: o.get_field("secondaryPrivateIps"),
            security_groups: o.get_field("securityGroups"),
            source_dest_check: o.get_field("sourceDestCheck"),
            subnet_id: o.get_field("subnetId"),
            tags: o.get_field("tags"),
            tenancy: o.get_field("tenancy"),
            user_data: o.get_field("userData"),
            user_data_base64: o.get_field("userDataBase64"),
            vpc_security_group_ids: o.get_field("vpcSecurityGroupIds"),
        }
    }
}
