/// Registers members (network interfaces) with the transit gateway multicast group.
/// A member is a network interface associated with a supported EC2 instance that receives multicast traffic.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = multicast_group_member::create(
///         "example",
///         MulticastGroupMemberArgs::builder()
///             .group_ip_address("224.0.0.1")
///             .network_interface_id("${exampleAwsNetworkInterface.id}")
///             .transit_gateway_multicast_domain_id(
///                 "${exampleAwsEc2TransitGatewayMulticastDomain.id}",
///             )
///             .build_struct(),
///     );
/// }
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod multicast_group_member {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MulticastGroupMemberArgs {
        /// The IP address assigned to the transit gateway multicast group.
        #[builder(into)]
        pub group_ip_address: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The group members' network interface ID to register with the transit gateway multicast group.
        #[builder(into)]
        pub network_interface_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the transit gateway multicast domain.
        #[builder(into)]
        pub transit_gateway_multicast_domain_id: pulumi_gestalt_rust::InputOrOutput<
            String,
        >,
    }
    #[allow(dead_code)]
    pub struct MulticastGroupMemberResult {
        /// The IP address assigned to the transit gateway multicast group.
        pub group_ip_address: pulumi_gestalt_rust::Output<String>,
        /// The group members' network interface ID to register with the transit gateway multicast group.
        pub network_interface_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the transit gateway multicast domain.
        pub transit_gateway_multicast_domain_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: MulticastGroupMemberArgs,
    ) -> MulticastGroupMemberResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let group_ip_address_binding_1 = args.group_ip_address.get_output(context);
        let group_ip_address_binding = group_ip_address_binding_1.get_inner();
        let network_interface_id_binding_1 = args
            .network_interface_id
            .get_output(context);
        let network_interface_id_binding = network_interface_id_binding_1.get_inner();
        let transit_gateway_multicast_domain_id_binding_1 = args
            .transit_gateway_multicast_domain_id
            .get_output(context);
        let transit_gateway_multicast_domain_id_binding = transit_gateway_multicast_domain_id_binding_1
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ec2transitgateway/multicastGroupMember:MulticastGroupMember"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "groupIpAddress".into(),
                    value: &group_ip_address_binding,
                },
                register_interface::ObjectField {
                    name: "networkInterfaceId".into(),
                    value: &network_interface_id_binding,
                },
                register_interface::ObjectField {
                    name: "transitGatewayMulticastDomainId".into(),
                    value: &transit_gateway_multicast_domain_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        MulticastGroupMemberResult {
            group_ip_address: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("groupIpAddress"),
            ),
            network_interface_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("networkInterfaceId"),
            ),
            transit_gateway_multicast_domain_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("transitGatewayMulticastDomainId"),
            ),
        }
    }
}
