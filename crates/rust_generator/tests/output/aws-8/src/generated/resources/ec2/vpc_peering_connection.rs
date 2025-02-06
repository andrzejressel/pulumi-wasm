/// Provides a resource to manage a VPC peering connection.
///
/// > **NOTE on VPC Peering Connections and VPC Peering Connection Options:** This provider provides
/// both a standalone VPC Peering Connection Options and a VPC Peering Connection
/// resource with `accepter` and `requester` attributes. Do not manage options for the same VPC peering
/// connection in both a VPC Peering Connection resource and a VPC Peering Connection Options resource.
/// Doing so will cause a conflict of options and will overwrite the options.
/// Using a VPC Peering Connection Options resource decouples management of the connection options from
/// management of the VPC Peering Connection and allows options to be set correctly in cross-account scenarios.
///
/// > **Note:** For cross-account (requester's AWS account differs from the accepter's AWS account) or inter-region
/// VPC Peering Connections use the `aws.ec2.VpcPeeringConnection` resource to manage the requester's side of the
/// connection and use the `aws.ec2.VpcPeeringConnectionAccepter` resource to manage the accepter's side of the connection.
///
/// > **Note:** Creating multiple `aws.ec2.VpcPeeringConnection` resources with the same `peer_vpc_id` and `vpc_id` will not produce an error. Instead, AWS will return the connection `id` that already exists, resulting in multiple `aws.ec2.VpcPeeringConnection` resources with the same `id`.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let foo = vpc_peering_connection::create(
///         "foo",
///         VpcPeeringConnectionArgs::builder()
///             .peer_owner_id("${peerOwnerId}")
///             .peer_vpc_id("${bar.id}")
///             .vpc_id("${fooAwsVpc.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// Basic usage with connection options:
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let foo = vpc_peering_connection::create(
///         "foo",
///         VpcPeeringConnectionArgs::builder()
///             .accepter(
///                 VpcPeeringConnectionAccepter::builder()
///                     .allowRemoteVpcDnsResolution(true)
///                     .build_struct(),
///             )
///             .peer_owner_id("${peerOwnerId}")
///             .peer_vpc_id("${bar.id}")
///             .requester(
///                 VpcPeeringConnectionRequester::builder()
///                     .allowRemoteVpcDnsResolution(true)
///                     .build_struct(),
///             )
///             .vpc_id("${fooAwsVpc.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// Basic usage with tags:
///
/// ```yaml
/// resources:
///   foo:
///     type: aws:ec2:VpcPeeringConnection
///     properties:
///       peerOwnerId: ${peerOwnerId}
///       peerVpcId: ${bar.id}
///       vpcId: ${fooVpc.id}
///       autoAccept: true
///       tags:
///         Name: VPC Peering between foo and bar
///   fooVpc:
///     type: aws:ec2:Vpc
///     name: foo
///     properties:
///       cidrBlock: 10.1.0.0/16
///   bar:
///     type: aws:ec2:Vpc
///     properties:
///       cidrBlock: 10.2.0.0/16
/// ```
///
/// Basic usage with region:
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let bar = vpc::create(
///         "bar",
///         VpcArgs::builder().cidr_block("10.2.0.0/16").build_struct(),
///     );
///     let foo = vpc_peering_connection::create(
///         "foo",
///         VpcPeeringConnectionArgs::builder()
///             .peer_owner_id("${peerOwnerId}")
///             .peer_region("us-east-1")
///             .peer_vpc_id("${bar.id}")
///             .vpc_id("${fooVpc.id}")
///             .build_struct(),
///     );
///     let fooVpc = vpc::create(
///         "fooVpc",
///         VpcArgs::builder().cidr_block("10.1.0.0/16").build_struct(),
///     );
/// }
/// ```
///
/// ## Notes
///
/// If both VPCs are not in the same AWS account and region do not enable the `auto_accept` attribute.
/// The accepter can manage its side of the connection using the `aws.ec2.VpcPeeringConnectionAccepter` resource
/// or accept the connection manually using the AWS Management Console, AWS CLI, through SDKs, etc.
///
/// ## Import
///
/// Using `pulumi import`, import VPC Peering resources using the VPC peering `id`. For example:
///
/// ```sh
/// $ pulumi import aws:ec2/vpcPeeringConnection:VpcPeeringConnection test_connection pcx-111aaa111
/// ```
pub mod vpc_peering_connection {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VpcPeeringConnectionArgs {
        /// An optional configuration block that allows for [VPC Peering Connection](https://docs.aws.amazon.com/vpc/latest/peering/what-is-vpc-peering.html) options to be set for the VPC that accepts
        /// the peering connection (a maximum of one).
        #[builder(into, default)]
        pub accepter: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::ec2::VpcPeeringConnectionAccepter>,
        >,
        /// Accept the peering (both VPCs need to be in the same AWS account and region).
        #[builder(into, default)]
        pub auto_accept: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The AWS account ID of the target peer VPC.
        /// Defaults to the account ID the [AWS provider][1] is currently connected to, so must be managed if connecting cross-account.
        #[builder(into, default)]
        pub peer_owner_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The region of the accepter VPC of the VPC Peering Connection. `auto_accept` must be `false`,
        /// and use the `aws.ec2.VpcPeeringConnectionAccepter` to manage the accepter side.
        #[builder(into, default)]
        pub peer_region: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the target VPC with which you are creating the VPC Peering Connection.
        #[builder(into)]
        pub peer_vpc_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A optional configuration block that allows for [VPC Peering Connection](https://docs.aws.amazon.com/vpc/latest/peering/what-is-vpc-peering.html) options to be set for the VPC that requests
        /// the peering connection (a maximum of one).
        #[builder(into, default)]
        pub requester: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::ec2::VpcPeeringConnectionRequester>,
        >,
        /// A map of tags to assign to the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The ID of the requester VPC.
        #[builder(into)]
        pub vpc_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct VpcPeeringConnectionResult {
        /// The status of the VPC Peering Connection request.
        pub accept_status: pulumi_gestalt_rust::Output<String>,
        /// An optional configuration block that allows for [VPC Peering Connection](https://docs.aws.amazon.com/vpc/latest/peering/what-is-vpc-peering.html) options to be set for the VPC that accepts
        /// the peering connection (a maximum of one).
        pub accepter: pulumi_gestalt_rust::Output<
            super::super::types::ec2::VpcPeeringConnectionAccepter,
        >,
        /// Accept the peering (both VPCs need to be in the same AWS account and region).
        pub auto_accept: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The AWS account ID of the target peer VPC.
        /// Defaults to the account ID the [AWS provider][1] is currently connected to, so must be managed if connecting cross-account.
        pub peer_owner_id: pulumi_gestalt_rust::Output<String>,
        /// The region of the accepter VPC of the VPC Peering Connection. `auto_accept` must be `false`,
        /// and use the `aws.ec2.VpcPeeringConnectionAccepter` to manage the accepter side.
        pub peer_region: pulumi_gestalt_rust::Output<String>,
        /// The ID of the target VPC with which you are creating the VPC Peering Connection.
        pub peer_vpc_id: pulumi_gestalt_rust::Output<String>,
        /// A optional configuration block that allows for [VPC Peering Connection](https://docs.aws.amazon.com/vpc/latest/peering/what-is-vpc-peering.html) options to be set for the VPC that requests
        /// the peering connection (a maximum of one).
        pub requester: pulumi_gestalt_rust::Output<
            super::super::types::ec2::VpcPeeringConnectionRequester,
        >,
        /// A map of tags to assign to the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The ID of the requester VPC.
        pub vpc_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: VpcPeeringConnectionArgs,
    ) -> VpcPeeringConnectionResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let accepter_binding = args.accepter.get_output(context).get_inner();
        let auto_accept_binding = args.auto_accept.get_output(context).get_inner();
        let peer_owner_id_binding = args.peer_owner_id.get_output(context).get_inner();
        let peer_region_binding = args.peer_region.get_output(context).get_inner();
        let peer_vpc_id_binding = args.peer_vpc_id.get_output(context).get_inner();
        let requester_binding = args.requester.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let vpc_id_binding = args.vpc_id.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ec2/vpcPeeringConnection:VpcPeeringConnection".into(),
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
                    name: "requester".into(),
                    value: &requester_binding,
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
        let o = register_interface::register(context.get_inner(), &request);
        VpcPeeringConnectionResult {
            accept_status: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("acceptStatus"),
            ),
            accepter: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("accepter"),
            ),
            auto_accept: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("autoAccept"),
            ),
            peer_owner_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("peerOwnerId"),
            ),
            peer_region: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("peerRegion"),
            ),
            peer_vpc_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("peerVpcId"),
            ),
            requester: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("requester"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            vpc_id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("vpcId")),
        }
    }
}
