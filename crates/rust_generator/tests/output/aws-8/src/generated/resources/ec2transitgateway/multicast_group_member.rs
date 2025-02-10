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
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: MulticastGroupMemberArgs,
    ) -> MulticastGroupMemberResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let group_ip_address_binding = args.group_ip_address.get_output(context);
        let network_interface_id_binding = args.network_interface_id.get_output(context);
        let transit_gateway_multicast_domain_id_binding = args
            .transit_gateway_multicast_domain_id
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ec2transitgateway/multicastGroupMember:MulticastGroupMember"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "groupIpAddress".into(),
                    value: group_ip_address_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "networkInterfaceId".into(),
                    value: network_interface_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "transitGatewayMulticastDomainId".into(),
                    value: transit_gateway_multicast_domain_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        MulticastGroupMemberResult {
            group_ip_address: o.get_field("groupIpAddress"),
            network_interface_id: o.get_field("networkInterfaceId"),
            transit_gateway_multicast_domain_id: o
                .get_field("transitGatewayMulticastDomainId"),
        }
    }
}
