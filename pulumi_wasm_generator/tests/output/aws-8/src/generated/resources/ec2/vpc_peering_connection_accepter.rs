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
pub mod vpc_peering_connection_accepter {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VpcPeeringConnectionAccepterArgs {
        /// A configuration block that describes [VPC Peering Connection]
        /// (https://docs.aws.amazon.com/vpc/latest/peering/what-is-vpc-peering.html) options set for the accepter VPC.
        #[builder(into, default)]
        pub accepter: pulumi_wasm_rust::Output<
            Option<super::super::types::ec2::VpcPeeringConnectionAccepterAccepter>,
        >,
        /// Whether or not to accept the peering request. Defaults to `false`.
        #[builder(into, default)]
        pub auto_accept: pulumi_wasm_rust::Output<Option<bool>>,
        /// A configuration block that describes [VPC Peering Connection]
        /// (https://docs.aws.amazon.com/vpc/latest/peering/what-is-vpc-peering.html) options set for the requester VPC.
        #[builder(into, default)]
        pub requester: pulumi_wasm_rust::Output<
            Option<super::super::types::ec2::VpcPeeringConnectionAccepterRequester>,
        >,
        /// A map of tags to assign to the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The VPC Peering Connection ID to manage.
        #[builder(into)]
        pub vpc_peering_connection_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct VpcPeeringConnectionAccepterResult {
        /// The status of the VPC Peering Connection request.
        pub accept_status: pulumi_wasm_rust::Output<String>,
        /// A configuration block that describes [VPC Peering Connection]
        /// (https://docs.aws.amazon.com/vpc/latest/peering/what-is-vpc-peering.html) options set for the accepter VPC.
        pub accepter: pulumi_wasm_rust::Output<
            super::super::types::ec2::VpcPeeringConnectionAccepterAccepter,
        >,
        /// Whether or not to accept the peering request. Defaults to `false`.
        pub auto_accept: pulumi_wasm_rust::Output<Option<bool>>,
        /// The AWS account ID of the owner of the requester VPC.
        pub peer_owner_id: pulumi_wasm_rust::Output<String>,
        /// The region of the accepter VPC.
        pub peer_region: pulumi_wasm_rust::Output<String>,
        /// The ID of the requester VPC.
        pub peer_vpc_id: pulumi_wasm_rust::Output<String>,
        /// A configuration block that describes [VPC Peering Connection]
        /// (https://docs.aws.amazon.com/vpc/latest/peering/what-is-vpc-peering.html) options set for the requester VPC.
        pub requester: pulumi_wasm_rust::Output<
            super::super::types::ec2::VpcPeeringConnectionAccepterRequester,
        >,
        /// A map of tags to assign to the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The ID of the accepter VPC.
        pub vpc_id: pulumi_wasm_rust::Output<String>,
        /// The VPC Peering Connection ID to manage.
        pub vpc_peering_connection_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: VpcPeeringConnectionAccepterArgs,
    ) -> VpcPeeringConnectionAccepterResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let accepter_binding = args.accepter.get_inner();
        let auto_accept_binding = args.auto_accept.get_inner();
        let requester_binding = args.requester.get_inner();
        let tags_binding = args.tags.get_inner();
        let vpc_peering_connection_id_binding = args
            .vpc_peering_connection_id
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ec2/vpcPeeringConnectionAccepter:VpcPeeringConnectionAccepter"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accepter".into(),
                    value: &accepter_binding,
                },
                register_interface::ObjectField {
                    name: "autoAccept".into(),
                    value: &auto_accept_binding,
                },
                register_interface::ObjectField {
                    name: "requester".into(),
                    value: &requester_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "vpcPeeringConnectionId".into(),
                    value: &vpc_peering_connection_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "acceptStatus".into(),
                },
                register_interface::ResultField {
                    name: "accepter".into(),
                },
                register_interface::ResultField {
                    name: "autoAccept".into(),
                },
                register_interface::ResultField {
                    name: "peerOwnerId".into(),
                },
                register_interface::ResultField {
                    name: "peerRegion".into(),
                },
                register_interface::ResultField {
                    name: "peerVpcId".into(),
                },
                register_interface::ResultField {
                    name: "requester".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "vpcId".into(),
                },
                register_interface::ResultField {
                    name: "vpcPeeringConnectionId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        VpcPeeringConnectionAccepterResult {
            accept_status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("acceptStatus").unwrap(),
            ),
            accepter: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accepter").unwrap(),
            ),
            auto_accept: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("autoAccept").unwrap(),
            ),
            peer_owner_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("peerOwnerId").unwrap(),
            ),
            peer_region: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("peerRegion").unwrap(),
            ),
            peer_vpc_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("peerVpcId").unwrap(),
            ),
            requester: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("requester").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            vpc_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpcId").unwrap(),
            ),
            vpc_peering_connection_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpcPeeringConnectionId").unwrap(),
            ),
        }
    }
}
