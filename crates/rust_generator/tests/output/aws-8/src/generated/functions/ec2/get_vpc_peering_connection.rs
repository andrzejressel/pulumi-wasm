#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_vpc_peering_connection {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetVpcPeeringConnectionArgs {
        /// Primary CIDR block of the requester VPC of the specific VPC Peering Connection to retrieve.
        #[builder(into, default)]
        pub cidr_block: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Custom filter block as described below.
        #[builder(into, default)]
        pub filters: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::super::types::ec2::GetVpcPeeringConnectionFilter>>,
        >,
        /// ID of the specific VPC Peering Connection to retrieve.
        #[builder(into, default)]
        pub id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// AWS account ID of the owner of the requester VPC of the specific VPC Peering Connection to retrieve.
        #[builder(into, default)]
        pub owner_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Primary CIDR block of the accepter VPC of the specific VPC Peering Connection to retrieve.
        #[builder(into, default)]
        pub peer_cidr_block: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// AWS account ID of the owner of the accepter VPC of the specific VPC Peering Connection to retrieve.
        #[builder(into, default)]
        pub peer_owner_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Region of the accepter VPC of the specific VPC Peering Connection to retrieve.
        #[builder(into, default)]
        pub peer_region: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// ID of the accepter VPC of the specific VPC Peering Connection to retrieve.
        #[builder(into, default)]
        pub peer_vpc_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Region of the requester VPC of the specific VPC Peering Connection to retrieve.
        #[builder(into, default)]
        pub region: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Status of the specific VPC Peering Connection to retrieve.
        #[builder(into, default)]
        pub status: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Map of tags, each pair of which must exactly match
        /// a pair on the desired VPC Peering Connection.
        ///
        /// More complex filters can be expressed using one or more `filter` sub-blocks,
        /// which take the following arguments:
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// ID of the requester VPC of the specific VPC Peering Connection to retrieve.
        #[builder(into, default)]
        pub vpc_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetVpcPeeringConnectionResult {
        /// Configuration block that describes [VPC Peering Connection]
        /// (https://docs.aws.amazon.com/vpc/latest/peering/what-is-vpc-peering.html) options set for the accepter VPC.
        pub accepter: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, bool>,
        >,
        /// CIDR block associated to the VPC of the specific VPC Peering Connection.
        pub cidr_block: pulumi_gestalt_rust::Output<String>,
        /// List of objects with IPv4 CIDR blocks of the requester VPC.
        pub cidr_block_sets: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::ec2::GetVpcPeeringConnectionCidrBlockSet>,
        >,
        pub filters: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::super::types::ec2::GetVpcPeeringConnectionFilter>>,
        >,
        pub id: pulumi_gestalt_rust::Output<String>,
        /// List of objects with IPv6 CIDR blocks of the requester VPC.
        pub ipv6_cidr_block_sets: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::ec2::GetVpcPeeringConnectionIpv6CidrBlockSet>,
        >,
        pub owner_id: pulumi_gestalt_rust::Output<String>,
        pub peer_cidr_block: pulumi_gestalt_rust::Output<String>,
        /// List of objects with IPv4 CIDR blocks of the accepter VPC.
        pub peer_cidr_block_sets: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::ec2::GetVpcPeeringConnectionPeerCidrBlockSet>,
        >,
        /// List of objects with IPv6 CIDR blocks of the accepter VPC.
        pub peer_ipv6_cidr_block_sets: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::ec2::GetVpcPeeringConnectionPeerIpv6CidrBlockSet,
            >,
        >,
        pub peer_owner_id: pulumi_gestalt_rust::Output<String>,
        pub peer_region: pulumi_gestalt_rust::Output<String>,
        pub peer_vpc_id: pulumi_gestalt_rust::Output<String>,
        pub region: pulumi_gestalt_rust::Output<String>,
        /// Configuration block that describes [VPC Peering Connection]
        /// (https://docs.aws.amazon.com/vpc/latest/peering/what-is-vpc-peering.html) options set for the requester VPC.
        pub requester: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, bool>,
        >,
        pub status: pulumi_gestalt_rust::Output<String>,
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        pub vpc_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetVpcPeeringConnectionArgs,
    ) -> GetVpcPeeringConnectionResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let cidr_block_binding = args.cidr_block.get_output(context);
        let filters_binding = args.filters.get_output(context);
        let id_binding = args.id.get_output(context);
        let owner_id_binding = args.owner_id.get_output(context);
        let peer_cidr_block_binding = args.peer_cidr_block.get_output(context);
        let peer_owner_id_binding = args.peer_owner_id.get_output(context);
        let peer_region_binding = args.peer_region.get_output(context);
        let peer_vpc_id_binding = args.peer_vpc_id.get_output(context);
        let region_binding = args.region.get_output(context);
        let status_binding = args.status.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let vpc_id_binding = args.vpc_id.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:ec2/getVpcPeeringConnection:getVpcPeeringConnection".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cidrBlock".into(),
                    value: &cidr_block_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "filters".into(),
                    value: &filters_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "id".into(),
                    value: &id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ownerId".into(),
                    value: &owner_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "peerCidrBlock".into(),
                    value: &peer_cidr_block_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "peerOwnerId".into(),
                    value: &peer_owner_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "peerRegion".into(),
                    value: &peer_region_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "peerVpcId".into(),
                    value: &peer_vpc_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "region".into(),
                    value: &region_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "status".into(),
                    value: &status_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vpcId".into(),
                    value: &vpc_id_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetVpcPeeringConnectionResult {
            accepter: o.get_field("accepter"),
            cidr_block: o.get_field("cidrBlock"),
            cidr_block_sets: o.get_field("cidrBlockSets"),
            filters: o.get_field("filters"),
            id: o.get_field("id"),
            ipv6_cidr_block_sets: o.get_field("ipv6CidrBlockSets"),
            owner_id: o.get_field("ownerId"),
            peer_cidr_block: o.get_field("peerCidrBlock"),
            peer_cidr_block_sets: o.get_field("peerCidrBlockSets"),
            peer_ipv6_cidr_block_sets: o.get_field("peerIpv6CidrBlockSets"),
            peer_owner_id: o.get_field("peerOwnerId"),
            peer_region: o.get_field("peerRegion"),
            peer_vpc_id: o.get_field("peerVpcId"),
            region: o.get_field("region"),
            requester: o.get_field("requester"),
            status: o.get_field("status"),
            tags: o.get_field("tags"),
            vpc_id: o.get_field("vpcId"),
        }
    }
}
