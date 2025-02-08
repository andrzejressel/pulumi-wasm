#[allow(clippy::doc_lazy_continuation)]
pub mod get_elastic_ip {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetElasticIpArgs {
        /// One or more name/value pairs to use as filters. There are several valid keys, for a full reference, check out the [EC2 API Reference](https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_DescribeAddresses.html).
        #[builder(into, default)]
        pub filters: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::super::types::ec2::GetElasticIpFilter>>,
        >,
        /// Allocation ID of the specific VPC EIP to retrieve. If a classic EIP is required, do NOT set `id`, only set `public_ip`
        #[builder(into, default)]
        pub id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Public IP of the specific EIP to retrieve.
        #[builder(into, default)]
        pub public_ip: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Map of tags, each pair of which must exactly match a pair on the desired Elastic IP
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetElasticIpResult {
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// ID representing the association of the address with an instance in a VPC.
        pub association_id: pulumi_gestalt_rust::Output<String>,
        /// Carrier IP address.
        pub carrier_ip: pulumi_gestalt_rust::Output<String>,
        /// Customer Owned IP.
        pub customer_owned_ip: pulumi_gestalt_rust::Output<String>,
        /// The ID of a Customer Owned IP Pool. For more on customer owned IP addressed check out [Customer-owned IP addresses guide](https://docs.aws.amazon.com/outposts/latest/userguide/outposts-networking-components.html#ip-addressing)
        pub customer_owned_ipv4_pool: pulumi_gestalt_rust::Output<String>,
        /// Whether the address is for use in EC2-Classic (standard) or in a VPC (vpc).
        pub domain: pulumi_gestalt_rust::Output<String>,
        pub filters: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::super::types::ec2::GetElasticIpFilter>>,
        >,
        /// If VPC Elastic IP, the allocation identifier. If EC2-Classic Elastic IP, the public IP address.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// ID of the instance that the address is associated with (if any).
        pub instance_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of an IPAM pool which has an Amazon-provided or BYOIP public IPv4 CIDR provisioned to it.
        pub ipam_pool_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the network interface.
        pub network_interface_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the AWS account that owns the network interface.
        pub network_interface_owner_id: pulumi_gestalt_rust::Output<String>,
        /// Private DNS associated with the Elastic IP address.
        pub private_dns: pulumi_gestalt_rust::Output<String>,
        /// Private IP address associated with the Elastic IP address.
        pub private_ip: pulumi_gestalt_rust::Output<String>,
        /// The DNS pointer (PTR) record for the IP address.
        pub ptr_record: pulumi_gestalt_rust::Output<String>,
        /// Public DNS associated with the Elastic IP address.
        pub public_dns: pulumi_gestalt_rust::Output<String>,
        /// Public IP address of Elastic IP.
        pub public_ip: pulumi_gestalt_rust::Output<String>,
        /// ID of an address pool.
        pub public_ipv4_pool: pulumi_gestalt_rust::Output<String>,
        /// Key-value map of tags associated with Elastic IP.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetElasticIpArgs,
    ) -> GetElasticIpResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let filters_binding = args.filters.get_output(context).get_inner();
        let id_binding = args.id.get_output(context).get_inner();
        let public_ip_binding = args.public_ip.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:ec2/getElasticIp:getElasticIp".into(),
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
                    name: "publicIp".into(),
                    value: &public_ip_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetElasticIpResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            association_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("associationId"),
            ),
            carrier_ip: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("carrierIp"),
            ),
            customer_owned_ip: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("customerOwnedIp"),
            ),
            customer_owned_ipv4_pool: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("customerOwnedIpv4Pool"),
            ),
            domain: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("domain"),
            ),
            filters: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("filters"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            instance_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("instanceId"),
            ),
            ipam_pool_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ipamPoolId"),
            ),
            network_interface_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("networkInterfaceId"),
            ),
            network_interface_owner_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("networkInterfaceOwnerId"),
            ),
            private_dns: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("privateDns"),
            ),
            private_ip: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("privateIp"),
            ),
            ptr_record: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ptrRecord"),
            ),
            public_dns: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("publicDns"),
            ),
            public_ip: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("publicIp"),
            ),
            public_ipv4_pool: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("publicIpv4Pool"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
