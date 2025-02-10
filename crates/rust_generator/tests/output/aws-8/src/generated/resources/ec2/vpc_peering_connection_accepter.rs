/// Provides a resource to manage the accepter's side of a VPC Peering Connection.
///
/// When a cross-account (requester's AWS account differs from the accepter's AWS account) or an inter-region
/// VPC Peering Connection is created, a VPC Peering Connection resource is automatically created in the
/// accepter's account.
/// The requester can use the `aws.ec2.VpcPeeringConnection` resource to manage its side of the connection
/// and the accepter can use the `aws.ec2.VpcPeeringConnectionAccepter` resource to "adopt" its side of the
/// connection into management.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   main:
///     type: aws:ec2:Vpc
///     properties:
///       cidrBlock: 10.0.0.0/16
///   peerVpc:
///     type: aws:ec2:Vpc
///     name: peer
///     properties:
///       cidrBlock: 10.1.0.0/16
///   # Requester's side of the connection.
///   peerVpcPeeringConnection:
///     type: aws:ec2:VpcPeeringConnection
///     name: peer
///     properties:
///       vpcId: ${main.id}
///       peerVpcId: ${peerVpc.id}
///       peerOwnerId: ${peer.accountId}
///       peerRegion: us-west-2
///       autoAccept: false
///       tags:
///         Side: Requester
///   # Accepter's side of the connection.
///   peerVpcPeeringConnectionAccepter:
///     type: aws:ec2:VpcPeeringConnectionAccepter
///     name: peer
///     properties:
///       vpcPeeringConnectionId: ${peerVpcPeeringConnection.id}
///       autoAccept: true
///       tags:
///         Side: Accepter
/// variables:
///   peer:
///     fn::invoke:
///       function: aws:getCallerIdentity
///       arguments: {}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import VPC Peering Connection Accepters using the Peering Connection ID. For example:
///
/// ```sh
/// $ pulumi import aws:ec2/vpcPeeringConnectionAccepter:VpcPeeringConnectionAccepter example pcx-12345678
/// ```
/// Certain resource arguments, like `auto_accept`, do not have an EC2 API method for reading the information after peering connection creation. If the argument is set in the Pulumi program on an imported resource, Pulumi will always show a difference. To workaround this behavior, either omit the argument from the Pulumi program or use `ignore_changes` to hide the difference. For example:
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod vpc_peering_connection_accepter {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VpcPeeringConnectionAccepterArgs {
        /// A configuration block that describes [VPC Peering Connection]
        /// (https://docs.aws.amazon.com/vpc/latest/peering/what-is-vpc-peering.html) options set for the accepter VPC.
        #[builder(into, default)]
        pub accepter: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::ec2::VpcPeeringConnectionAccepterAccepter>,
        >,
        /// Whether or not to accept the peering request. Defaults to `false`.
        #[builder(into, default)]
        pub auto_accept: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// A configuration block that describes [VPC Peering Connection]
        /// (https://docs.aws.amazon.com/vpc/latest/peering/what-is-vpc-peering.html) options set for the requester VPC.
        #[builder(into, default)]
        pub requester: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::ec2::VpcPeeringConnectionAccepterRequester>,
        >,
        /// A map of tags to assign to the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The VPC Peering Connection ID to manage.
        #[builder(into)]
        pub vpc_peering_connection_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct VpcPeeringConnectionAccepterResult {
        /// The status of the VPC Peering Connection request.
        pub accept_status: pulumi_gestalt_rust::Output<String>,
        /// A configuration block that describes [VPC Peering Connection]
        /// (https://docs.aws.amazon.com/vpc/latest/peering/what-is-vpc-peering.html) options set for the accepter VPC.
        pub accepter: pulumi_gestalt_rust::Output<
            super::super::types::ec2::VpcPeeringConnectionAccepterAccepter,
        >,
        /// Whether or not to accept the peering request. Defaults to `false`.
        pub auto_accept: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The AWS account ID of the owner of the requester VPC.
        pub peer_owner_id: pulumi_gestalt_rust::Output<String>,
        /// The region of the accepter VPC.
        pub peer_region: pulumi_gestalt_rust::Output<String>,
        /// The ID of the requester VPC.
        pub peer_vpc_id: pulumi_gestalt_rust::Output<String>,
        /// A configuration block that describes [VPC Peering Connection]
        /// (https://docs.aws.amazon.com/vpc/latest/peering/what-is-vpc-peering.html) options set for the requester VPC.
        pub requester: pulumi_gestalt_rust::Output<
            super::super::types::ec2::VpcPeeringConnectionAccepterRequester,
        >,
        /// A map of tags to assign to the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The ID of the accepter VPC.
        pub vpc_id: pulumi_gestalt_rust::Output<String>,
        /// The VPC Peering Connection ID to manage.
        pub vpc_peering_connection_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: VpcPeeringConnectionAccepterArgs,
    ) -> VpcPeeringConnectionAccepterResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let accepter_binding = args.accepter.get_output(context);
        let auto_accept_binding = args.auto_accept.get_output(context);
        let requester_binding = args.requester.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let vpc_peering_connection_id_binding = args
            .vpc_peering_connection_id
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ec2/vpcPeeringConnectionAccepter:VpcPeeringConnectionAccepter"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accepter".into(),
                    value: accepter_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "autoAccept".into(),
                    value: auto_accept_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "requester".into(),
                    value: requester_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vpcPeeringConnectionId".into(),
                    value: vpc_peering_connection_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        VpcPeeringConnectionAccepterResult {
            accept_status: o.get_field("acceptStatus"),
            accepter: o.get_field("accepter"),
            auto_accept: o.get_field("autoAccept"),
            peer_owner_id: o.get_field("peerOwnerId"),
            peer_region: o.get_field("peerRegion"),
            peer_vpc_id: o.get_field("peerVpcId"),
            requester: o.get_field("requester"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            vpc_id: o.get_field("vpcId"),
            vpc_peering_connection_id: o.get_field("vpcPeeringConnectionId"),
        }
    }
}
