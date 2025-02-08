#[allow(clippy::doc_lazy_continuation)]
pub mod get_network_interface {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetNetworkInterfaceArgs {
        /// One or more name/value pairs to filter off of. There are several valid keys, for a full reference, check out [describe-network-interfaces](https://docs.aws.amazon.com/cli/latest/reference/ec2/describe-network-interfaces.html) in the AWS CLI reference.
        #[builder(into, default)]
        pub filters: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::super::types::ec2::GetNetworkInterfaceFilter>>,
        >,
        /// Identifier for the network interface.
        #[builder(into, default)]
        pub id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Any tags assigned to the network interface.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetNetworkInterfaceResult {
        /// ARN of the network interface.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Association information for an Elastic IP address (IPv4) associated with the network interface. See supported fields below.
        pub associations: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::ec2::GetNetworkInterfaceAssociation>,
        >,
        pub attachments: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::ec2::GetNetworkInterfaceAttachment>,
        >,
        /// Availability Zone.
        pub availability_zone: pulumi_gestalt_rust::Output<String>,
        /// Description of the network interface.
        pub description: pulumi_gestalt_rust::Output<String>,
        pub filters: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::super::types::ec2::GetNetworkInterfaceFilter>>,
        >,
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Type of interface.
        pub interface_type: pulumi_gestalt_rust::Output<String>,
        /// List of IPv6 addresses to assign to the ENI.
        pub ipv6_addresses: pulumi_gestalt_rust::Output<Vec<String>>,
        /// MAC address.
        pub mac_address: pulumi_gestalt_rust::Output<String>,
        /// ARN of the Outpost.
        pub outpost_arn: pulumi_gestalt_rust::Output<String>,
        /// AWS account ID of the owner of the network interface.
        pub owner_id: pulumi_gestalt_rust::Output<String>,
        /// Private DNS name.
        pub private_dns_name: pulumi_gestalt_rust::Output<String>,
        /// Private IPv4 address of the network interface within the subnet.
        pub private_ip: pulumi_gestalt_rust::Output<String>,
        /// Private IPv4 addresses associated with the network interface.
        pub private_ips: pulumi_gestalt_rust::Output<Vec<String>>,
        /// ID of the entity that launched the instance on your behalf.
        pub requester_id: pulumi_gestalt_rust::Output<String>,
        /// List of security groups for the network interface.
        pub security_groups: pulumi_gestalt_rust::Output<Vec<String>>,
        /// ID of the subnet.
        pub subnet_id: pulumi_gestalt_rust::Output<String>,
        /// Any tags assigned to the network interface.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// ID of the VPC.
        pub vpc_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetNetworkInterfaceArgs,
    ) -> GetNetworkInterfaceResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
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
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetNetworkInterfaceResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            associations: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("associations"),
            ),
            attachments: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("attachments"),
            ),
            availability_zone: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("availabilityZone"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            filters: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("filters"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            interface_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("interfaceType"),
            ),
            ipv6_addresses: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ipv6Addresses"),
            ),
            mac_address: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("macAddress"),
            ),
            outpost_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("outpostArn"),
            ),
            owner_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ownerId"),
            ),
            private_dns_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("privateDnsName"),
            ),
            private_ip: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("privateIp"),
            ),
            private_ips: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("privateIps"),
            ),
            requester_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("requesterId"),
            ),
            security_groups: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("securityGroups"),
            ),
            subnet_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("subnetId"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            vpc_id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("vpcId")),
        }
    }
}
