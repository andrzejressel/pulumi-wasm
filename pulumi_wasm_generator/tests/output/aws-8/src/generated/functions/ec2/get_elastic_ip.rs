pub mod get_elastic_ip {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetElasticIpArgs {
        /// One or more name/value pairs to use as filters. There are several valid keys, for a full reference, check out the [EC2 API Reference](https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_DescribeAddresses.html).
        #[builder(into, default)]
        pub filters: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::super::types::ec2::GetElasticIpFilter>>,
        >,
        /// Allocation ID of the specific VPC EIP to retrieve. If a classic EIP is required, do NOT set `id`, only set `public_ip`
        #[builder(into, default)]
        pub id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Public IP of the specific EIP to retrieve.
        #[builder(into, default)]
        pub public_ip: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Map of tags, each pair of which must exactly match a pair on the desired Elastic IP
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetElasticIpResult {
        pub arn: pulumi_wasm_rust::Output<String>,
        /// ID representing the association of the address with an instance in a VPC.
        pub association_id: pulumi_wasm_rust::Output<String>,
        /// Carrier IP address.
        pub carrier_ip: pulumi_wasm_rust::Output<String>,
        /// Customer Owned IP.
        pub customer_owned_ip: pulumi_wasm_rust::Output<String>,
        /// The ID of a Customer Owned IP Pool. For more on customer owned IP addressed check out [Customer-owned IP addresses guide](https://docs.aws.amazon.com/outposts/latest/userguide/outposts-networking-components.html#ip-addressing)
        pub customer_owned_ipv4_pool: pulumi_wasm_rust::Output<String>,
        /// Whether the address is for use in EC2-Classic (standard) or in a VPC (vpc).
        pub domain: pulumi_wasm_rust::Output<String>,
        pub filters: pulumi_wasm_rust::Output<
            Option<Vec<super::super::super::types::ec2::GetElasticIpFilter>>,
        >,
        /// If VPC Elastic IP, the allocation identifier. If EC2-Classic Elastic IP, the public IP address.
        pub id: pulumi_wasm_rust::Output<String>,
        /// ID of the instance that the address is associated with (if any).
        pub instance_id: pulumi_wasm_rust::Output<String>,
        /// The ID of an IPAM pool which has an Amazon-provided or BYOIP public IPv4 CIDR provisioned to it.
        pub ipam_pool_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the network interface.
        pub network_interface_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the AWS account that owns the network interface.
        pub network_interface_owner_id: pulumi_wasm_rust::Output<String>,
        /// Private DNS associated with the Elastic IP address.
        pub private_dns: pulumi_wasm_rust::Output<String>,
        /// Private IP address associated with the Elastic IP address.
        pub private_ip: pulumi_wasm_rust::Output<String>,
        /// The DNS pointer (PTR) record for the IP address.
        pub ptr_record: pulumi_wasm_rust::Output<String>,
        /// Public DNS associated with the Elastic IP address.
        pub public_dns: pulumi_wasm_rust::Output<String>,
        /// Public IP address of Elastic IP.
        pub public_ip: pulumi_wasm_rust::Output<String>,
        /// ID of an address pool.
        pub public_ipv4_pool: pulumi_wasm_rust::Output<String>,
        /// Key-value map of tags associated with Elastic IP.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetElasticIpArgs,
    ) -> GetElasticIpResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "associationId".into(),
                },
                register_interface::ResultField {
                    name: "carrierIp".into(),
                },
                register_interface::ResultField {
                    name: "customerOwnedIp".into(),
                },
                register_interface::ResultField {
                    name: "customerOwnedIpv4Pool".into(),
                },
                register_interface::ResultField {
                    name: "domain".into(),
                },
                register_interface::ResultField {
                    name: "filters".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "instanceId".into(),
                },
                register_interface::ResultField {
                    name: "ipamPoolId".into(),
                },
                register_interface::ResultField {
                    name: "networkInterfaceId".into(),
                },
                register_interface::ResultField {
                    name: "networkInterfaceOwnerId".into(),
                },
                register_interface::ResultField {
                    name: "privateDns".into(),
                },
                register_interface::ResultField {
                    name: "privateIp".into(),
                },
                register_interface::ResultField {
                    name: "ptrRecord".into(),
                },
                register_interface::ResultField {
                    name: "publicDns".into(),
                },
                register_interface::ResultField {
                    name: "publicIp".into(),
                },
                register_interface::ResultField {
                    name: "publicIpv4Pool".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetElasticIpResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            association_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("associationId").unwrap(),
            ),
            carrier_ip: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("carrierIp").unwrap(),
            ),
            customer_owned_ip: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customerOwnedIp").unwrap(),
            ),
            customer_owned_ipv4_pool: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customerOwnedIpv4Pool").unwrap(),
            ),
            domain: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("domain").unwrap(),
            ),
            filters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("filters").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            instance_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instanceId").unwrap(),
            ),
            ipam_pool_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipamPoolId").unwrap(),
            ),
            network_interface_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("networkInterfaceId").unwrap(),
            ),
            network_interface_owner_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("networkInterfaceOwnerId").unwrap(),
            ),
            private_dns: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("privateDns").unwrap(),
            ),
            private_ip: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("privateIp").unwrap(),
            ),
            ptr_record: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ptrRecord").unwrap(),
            ),
            public_dns: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("publicDns").unwrap(),
            ),
            public_ip: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("publicIp").unwrap(),
            ),
            public_ipv4_pool: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("publicIpv4Pool").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}
