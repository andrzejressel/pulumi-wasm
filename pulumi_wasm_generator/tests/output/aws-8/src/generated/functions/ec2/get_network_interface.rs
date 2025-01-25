pub mod get_network_interface {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetNetworkInterfaceArgs {
        /// One or more name/value pairs to filter off of. There are several valid keys, for a full reference, check out [describe-network-interfaces](https://docs.aws.amazon.com/cli/latest/reference/ec2/describe-network-interfaces.html) in the AWS CLI reference.
        #[builder(into, default)]
        pub filters: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::super::types::ec2::GetNetworkInterfaceFilter>>,
        >,
        /// Identifier for the network interface.
        #[builder(into, default)]
        pub id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Any tags assigned to the network interface.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetNetworkInterfaceResult {
        /// ARN of the network interface.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Association information for an Elastic IP address (IPv4) associated with the network interface. See supported fields below.
        pub associations: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::ec2::GetNetworkInterfaceAssociation>,
        >,
        pub attachments: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::ec2::GetNetworkInterfaceAttachment>,
        >,
        /// Availability Zone.
        pub availability_zone: pulumi_wasm_rust::Output<String>,
        /// Description of the network interface.
        pub description: pulumi_wasm_rust::Output<String>,
        pub filters: pulumi_wasm_rust::Output<
            Option<Vec<super::super::super::types::ec2::GetNetworkInterfaceFilter>>,
        >,
        pub id: pulumi_wasm_rust::Output<String>,
        /// Type of interface.
        pub interface_type: pulumi_wasm_rust::Output<String>,
        /// List of IPv6 addresses to assign to the ENI.
        pub ipv6_addresses: pulumi_wasm_rust::Output<Vec<String>>,
        /// MAC address.
        pub mac_address: pulumi_wasm_rust::Output<String>,
        /// ARN of the Outpost.
        pub outpost_arn: pulumi_wasm_rust::Output<String>,
        /// AWS account ID of the owner of the network interface.
        pub owner_id: pulumi_wasm_rust::Output<String>,
        /// Private DNS name.
        pub private_dns_name: pulumi_wasm_rust::Output<String>,
        /// Private IPv4 address of the network interface within the subnet.
        pub private_ip: pulumi_wasm_rust::Output<String>,
        /// Private IPv4 addresses associated with the network interface.
        pub private_ips: pulumi_wasm_rust::Output<Vec<String>>,
        /// ID of the entity that launched the instance on your behalf.
        pub requester_id: pulumi_wasm_rust::Output<String>,
        /// List of security groups for the network interface.
        pub security_groups: pulumi_wasm_rust::Output<Vec<String>>,
        /// ID of the subnet.
        pub subnet_id: pulumi_wasm_rust::Output<String>,
        /// Any tags assigned to the network interface.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// ID of the VPC.
        pub vpc_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetNetworkInterfaceArgs,
    ) -> GetNetworkInterfaceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let filters_binding = args.filters.get_output(context).get_inner();
        let id_binding = args.id.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:ec2/getNetworkInterface:getNetworkInterface".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "filters".into(),
                    value: &filters_binding,
                },
                register_interface::ObjectField {
                    name: "id".into(),
                    value: &id_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "associations".into(),
                },
                register_interface::ResultField {
                    name: "attachments".into(),
                },
                register_interface::ResultField {
                    name: "availabilityZone".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "filters".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "interfaceType".into(),
                },
                register_interface::ResultField {
                    name: "ipv6Addresses".into(),
                },
                register_interface::ResultField {
                    name: "macAddress".into(),
                },
                register_interface::ResultField {
                    name: "outpostArn".into(),
                },
                register_interface::ResultField {
                    name: "ownerId".into(),
                },
                register_interface::ResultField {
                    name: "privateDnsName".into(),
                },
                register_interface::ResultField {
                    name: "privateIp".into(),
                },
                register_interface::ResultField {
                    name: "privateIps".into(),
                },
                register_interface::ResultField {
                    name: "requesterId".into(),
                },
                register_interface::ResultField {
                    name: "securityGroups".into(),
                },
                register_interface::ResultField {
                    name: "subnetId".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "vpcId".into(),
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetNetworkInterfaceResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            associations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("associations").unwrap(),
            ),
            attachments: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("attachments").unwrap(),
            ),
            availability_zone: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("availabilityZone").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            filters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("filters").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            interface_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("interfaceType").unwrap(),
            ),
            ipv6_addresses: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipv6Addresses").unwrap(),
            ),
            mac_address: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("macAddress").unwrap(),
            ),
            outpost_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("outpostArn").unwrap(),
            ),
            owner_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ownerId").unwrap(),
            ),
            private_dns_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("privateDnsName").unwrap(),
            ),
            private_ip: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("privateIp").unwrap(),
            ),
            private_ips: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("privateIps").unwrap(),
            ),
            requester_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("requesterId").unwrap(),
            ),
            security_groups: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("securityGroups").unwrap(),
            ),
            subnet_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subnetId").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            vpc_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpcId").unwrap(),
            ),
        }
    }
}
