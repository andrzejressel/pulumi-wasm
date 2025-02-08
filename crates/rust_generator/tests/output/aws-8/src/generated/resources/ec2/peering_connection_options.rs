/// Provides a resource to manage VPC peering connection options.
///
/// > **NOTE on VPC Peering Connections and VPC Peering Connection Options:** This provider provides
/// both a standalone VPC Peering Connection Options and a VPC Peering Connection
/// resource with `accepter` and `requester` attributes. Do not manage options for the same VPC peering
/// connection in both a VPC Peering Connection resource and a VPC Peering Connection Options resource.
/// Doing so will cause a conflict of options and will overwrite the options.
/// Using a VPC Peering Connection Options resource decouples management of the connection options from
/// management of the VPC Peering Connection and allows options to be set correctly in cross-region and
/// cross-account scenarios.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let bar = vpc::create(
///         "bar",
///         VpcArgs::builder().cidr_block("10.1.0.0/16").build_struct(),
///     );
///     let foo = vpc::create(
///         "foo",
///         VpcArgs::builder().cidr_block("10.0.0.0/16").build_struct(),
///     );
///     let fooPeeringConnectionOptions = peering_connection_options::create(
///         "fooPeeringConnectionOptions",
///         PeeringConnectionOptionsArgs::builder()
///             .accepter(
///                 PeeringConnectionOptionsAccepter::builder()
///                     .allowRemoteVpcDnsResolution(true)
///                     .build_struct(),
///             )
///             .vpc_peering_connection_id("${fooVpcPeeringConnection.id}")
///             .build_struct(),
///     );
///     let fooVpcPeeringConnection = vpc_peering_connection::create(
///         "fooVpcPeeringConnection",
///         VpcPeeringConnectionArgs::builder()
///             .auto_accept(true)
///             .peer_vpc_id("${bar.id}")
///             .vpc_id("${foo.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Cross-Account Usage
///
/// ```yaml
/// resources:
///   main:
///     type: aws:ec2:Vpc
///     properties:
///       cidrBlock: 10.0.0.0/16
///       enableDnsSupport: true
///       enableDnsHostnames: true
///   peerVpc:
///     type: aws:ec2:Vpc
///     name: peer
///     properties:
///       cidrBlock: 10.1.0.0/16
///       enableDnsSupport: true
///       enableDnsHostnames: true
///   # Requester's side of the connection.
///   peerVpcPeeringConnection:
///     type: aws:ec2:VpcPeeringConnection
///     name: peer
///     properties:
///       vpcId: ${main.id}
///       peerVpcId: ${peerVpc.id}
///       peerOwnerId: ${peer.accountId}
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
///   requester:
///     type: aws:ec2:PeeringConnectionOptions
///     properties:
///       vpcPeeringConnectionId: ${peerVpcPeeringConnectionAccepter.id}
///       requester:
///         allowRemoteVpcDnsResolution: true
///   accepter:
///     type: aws:ec2:PeeringConnectionOptions
///     properties:
///       vpcPeeringConnectionId: ${peerVpcPeeringConnectionAccepter.id}
///       accepter:
///         allowRemoteVpcDnsResolution: true
/// variables:
///   peer:
///     fn::invoke:
///       function: aws:getCallerIdentity
///       arguments: {}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import VPC Peering Connection Options using the VPC peering `id`. For example:
///
/// ```sh
/// $ pulumi import aws:ec2/peeringConnectionOptions:PeeringConnectionOptions foo pcx-111aaa111
/// ```
#[allow(clippy::doc_lazy_continuation)]
pub mod peering_connection_options {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PeeringConnectionOptionsArgs {
        /// An optional configuration block that allows for [VPC Peering Connection](https://docs.aws.amazon.com/vpc/latest/peering/what-is-vpc-peering.html) options to be set for the VPC that acceptsthe peering connection (a maximum of one).
        #[builder(into, default)]
        pub accepter: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::ec2::PeeringConnectionOptionsAccepter>,
        >,
        /// A optional configuration block that allows for [VPC Peering Connection](https://docs.aws.amazon.com/vpc/latest/peering/what-is-vpc-peering.html) options to be set for the VPC that requeststhe peering connection (a maximum of one).
        #[builder(into, default)]
        pub requester: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::ec2::PeeringConnectionOptionsRequester>,
        >,
        /// The ID of the requester VPC peering connection.
        #[builder(into)]
        pub vpc_peering_connection_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct PeeringConnectionOptionsResult {
        /// An optional configuration block that allows for [VPC Peering Connection](https://docs.aws.amazon.com/vpc/latest/peering/what-is-vpc-peering.html) options to be set for the VPC that acceptsthe peering connection (a maximum of one).
        pub accepter: pulumi_gestalt_rust::Output<
            super::super::types::ec2::PeeringConnectionOptionsAccepter,
        >,
        /// A optional configuration block that allows for [VPC Peering Connection](https://docs.aws.amazon.com/vpc/latest/peering/what-is-vpc-peering.html) options to be set for the VPC that requeststhe peering connection (a maximum of one).
        pub requester: pulumi_gestalt_rust::Output<
            super::super::types::ec2::PeeringConnectionOptionsRequester,
        >,
        /// The ID of the requester VPC peering connection.
        pub vpc_peering_connection_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: PeeringConnectionOptionsArgs,
    ) -> PeeringConnectionOptionsResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let accepter_binding = args.accepter.get_output(context).get_inner();
        let requester_binding = args.requester.get_output(context).get_inner();
        let vpc_peering_connection_id_binding = args
            .vpc_peering_connection_id
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ec2/peeringConnectionOptions:PeeringConnectionOptions".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accepter".into(),
                    value: &accepter_binding,
                },
                register_interface::ObjectField {
                    name: "requester".into(),
                    value: &requester_binding,
                },
                register_interface::ObjectField {
                    name: "vpcPeeringConnectionId".into(),
                    value: &vpc_peering_connection_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        PeeringConnectionOptionsResult {
            accepter: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("accepter"),
            ),
            requester: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("requester"),
            ),
            vpc_peering_connection_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("vpcPeeringConnectionId"),
            ),
        }
    }
}
