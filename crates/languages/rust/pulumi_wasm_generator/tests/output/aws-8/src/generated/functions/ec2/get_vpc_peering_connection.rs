pub mod get_vpc_peering_connection {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetVpcPeeringConnectionArgs {
        /// Primary CIDR block of the requester VPC of the specific VPC Peering Connection to retrieve.
        #[builder(into, default)]
        pub cidr_block: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Custom filter block as described below.
        #[builder(into, default)]
        pub filters: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::super::types::ec2::GetVpcPeeringConnectionFilter>>,
        >,
        /// ID of the specific VPC Peering Connection to retrieve.
        #[builder(into, default)]
        pub id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// AWS account ID of the owner of the requester VPC of the specific VPC Peering Connection to retrieve.
        #[builder(into, default)]
        pub owner_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Primary CIDR block of the accepter VPC of the specific VPC Peering Connection to retrieve.
        #[builder(into, default)]
        pub peer_cidr_block: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// AWS account ID of the owner of the accepter VPC of the specific VPC Peering Connection to retrieve.
        #[builder(into, default)]
        pub peer_owner_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Region of the accepter VPC of the specific VPC Peering Connection to retrieve.
        #[builder(into, default)]
        pub peer_region: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// ID of the accepter VPC of the specific VPC Peering Connection to retrieve.
        #[builder(into, default)]
        pub peer_vpc_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Region of the requester VPC of the specific VPC Peering Connection to retrieve.
        #[builder(into, default)]
        pub region: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Status of the specific VPC Peering Connection to retrieve.
        #[builder(into, default)]
        pub status: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Map of tags, each pair of which must exactly match
        /// a pair on the desired VPC Peering Connection.
        ///
        /// More complex filters can be expressed using one or more `filter` sub-blocks,
        /// which take the following arguments:
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// ID of the requester VPC of the specific VPC Peering Connection to retrieve.
        #[builder(into, default)]
        pub vpc_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetVpcPeeringConnectionResult {
        /// Configuration block that describes [VPC Peering Connection]
        /// (https://docs.aws.amazon.com/vpc/latest/peering/what-is-vpc-peering.html) options set for the accepter VPC.
        pub accepter: pulumi_wasm_rust::Output<std::collections::HashMap<String, bool>>,
        /// CIDR block associated to the VPC of the specific VPC Peering Connection.
        pub cidr_block: pulumi_wasm_rust::Output<String>,
        /// List of objects with IPv4 CIDR blocks of the requester VPC.
        pub cidr_block_sets: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::ec2::GetVpcPeeringConnectionCidrBlockSet>,
        >,
        pub filters: pulumi_wasm_rust::Output<
            Option<Vec<super::super::super::types::ec2::GetVpcPeeringConnectionFilter>>,
        >,
        pub id: pulumi_wasm_rust::Output<String>,
        /// List of objects with IPv6 CIDR blocks of the requester VPC.
        pub ipv6_cidr_block_sets: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::ec2::GetVpcPeeringConnectionIpv6CidrBlockSet>,
        >,
        pub owner_id: pulumi_wasm_rust::Output<String>,
        pub peer_cidr_block: pulumi_wasm_rust::Output<String>,
        /// List of objects with IPv4 CIDR blocks of the accepter VPC.
        pub peer_cidr_block_sets: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::ec2::GetVpcPeeringConnectionPeerCidrBlockSet>,
        >,
        /// List of objects with IPv6 CIDR blocks of the accepter VPC.
        pub peer_ipv6_cidr_block_sets: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::ec2::GetVpcPeeringConnectionPeerIpv6CidrBlockSet,
            >,
        >,
        pub peer_owner_id: pulumi_wasm_rust::Output<String>,
        pub peer_region: pulumi_wasm_rust::Output<String>,
        pub peer_vpc_id: pulumi_wasm_rust::Output<String>,
        pub region: pulumi_wasm_rust::Output<String>,
        /// Configuration block that describes [VPC Peering Connection]
        /// (https://docs.aws.amazon.com/vpc/latest/peering/what-is-vpc-peering.html) options set for the requester VPC.
        pub requester: pulumi_wasm_rust::Output<std::collections::HashMap<String, bool>>,
        pub status: pulumi_wasm_rust::Output<String>,
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        pub vpc_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetVpcPeeringConnectionArgs,
    ) -> GetVpcPeeringConnectionResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let cidr_block_binding = args.cidr_block.get_output(context).get_inner();
        let filters_binding = args.filters.get_output(context).get_inner();
        let id_binding = args.id.get_output(context).get_inner();
        let owner_id_binding = args.owner_id.get_output(context).get_inner();
        let peer_cidr_block_binding = args
            .peer_cidr_block
            .get_output(context)
            .get_inner();
        let peer_owner_id_binding = args.peer_owner_id.get_output(context).get_inner();
        let peer_region_binding = args.peer_region.get_output(context).get_inner();
        let peer_vpc_id_binding = args.peer_vpc_id.get_output(context).get_inner();
        let region_binding = args.region.get_output(context).get_inner();
        let status_binding = args.status.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let vpc_id_binding = args.vpc_id.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:ec2/getVpcPeeringConnection:getVpcPeeringConnection".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "cidrBlock".into(),
                    value: &cidr_block_binding,
                },
                register_interface::ObjectField {
                    name: "filters".into(),
                    value: &filters_binding,
                },
                register_interface::ObjectField {
                    name: "id".into(),
                    value: &id_binding,
                },
                register_interface::ObjectField {
                    name: "ownerId".into(),
                    value: &owner_id_binding,
                },
                register_interface::ObjectField {
                    name: "peerCidrBlock".into(),
                    value: &peer_cidr_block_binding,
                },
                register_interface::ObjectField {
                    name: "peerOwnerId".into(),
                    value: &peer_owner_id_binding,
                },
                register_interface::ObjectField {
                    name: "peerRegion".into(),
                    value: &peer_region_binding,
                },
                register_interface::ObjectField {
                    name: "peerVpcId".into(),
                    value: &peer_vpc_id_binding,
                },
                register_interface::ObjectField {
                    name: "region".into(),
                    value: &region_binding,
                },
                register_interface::ObjectField {
                    name: "status".into(),
                    value: &status_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "vpcId".into(),
                    value: &vpc_id_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetVpcPeeringConnectionResult {
            accepter: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("accepter"),
            ),
            cidr_block: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("cidrBlock"),
            ),
            cidr_block_sets: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("cidrBlockSets"),
            ),
            filters: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("filters"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            ipv6_cidr_block_sets: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("ipv6CidrBlockSets"),
            ),
            owner_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("ownerId"),
            ),
            peer_cidr_block: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("peerCidrBlock"),
            ),
            peer_cidr_block_sets: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("peerCidrBlockSets"),
            ),
            peer_ipv6_cidr_block_sets: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("peerIpv6CidrBlockSets"),
            ),
            peer_owner_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("peerOwnerId"),
            ),
            peer_region: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("peerRegion"),
            ),
            peer_vpc_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("peerVpcId"),
            ),
            region: pulumi_wasm_rust::__private::into_domain(o.extract_field("region")),
            requester: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("requester"),
            ),
            status: pulumi_wasm_rust::__private::into_domain(o.extract_field("status")),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            vpc_id: pulumi_wasm_rust::__private::into_domain(o.extract_field("vpcId")),
        }
    }
}
