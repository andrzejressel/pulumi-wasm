/// Provides a Direct Connect BGP peer resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let peer = bgp_peer::create(
///         "peer",
///         BgpPeerArgs::builder()
///             .address_family("ipv6")
///             .bgp_asn(65351)
///             .virtual_interface_id("${foo.id}")
///             .build_struct(),
///     );
/// }
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod bgp_peer {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BgpPeerArgs {
        /// The address family for the BGP peer. `ipv4 ` or `ipv6`.
        #[builder(into)]
        pub address_family: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The IPv4 CIDR address to use to send traffic to Amazon.
        /// Required for IPv4 BGP peers on public virtual interfaces.
        #[builder(into, default)]
        pub amazon_address: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The autonomous system (AS) number for Border Gateway Protocol (BGP) configuration.
        #[builder(into)]
        pub bgp_asn: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// The authentication key for BGP configuration.
        #[builder(into, default)]
        pub bgp_auth_key: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The IPv4 CIDR destination address to which Amazon should send traffic.
        /// Required for IPv4 BGP peers on public virtual interfaces.
        #[builder(into, default)]
        pub customer_address: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the Direct Connect virtual interface on which to create the BGP peer.
        #[builder(into)]
        pub virtual_interface_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct BgpPeerResult {
        /// The address family for the BGP peer. `ipv4 ` or `ipv6`.
        pub address_family: pulumi_gestalt_rust::Output<String>,
        /// The IPv4 CIDR address to use to send traffic to Amazon.
        /// Required for IPv4 BGP peers on public virtual interfaces.
        pub amazon_address: pulumi_gestalt_rust::Output<String>,
        /// The Direct Connect endpoint on which the BGP peer terminates.
        pub aws_device: pulumi_gestalt_rust::Output<String>,
        /// The autonomous system (AS) number for Border Gateway Protocol (BGP) configuration.
        pub bgp_asn: pulumi_gestalt_rust::Output<i32>,
        /// The authentication key for BGP configuration.
        pub bgp_auth_key: pulumi_gestalt_rust::Output<String>,
        /// The ID of the BGP peer.
        pub bgp_peer_id: pulumi_gestalt_rust::Output<String>,
        /// The Up/Down state of the BGP peer.
        pub bgp_status: pulumi_gestalt_rust::Output<String>,
        /// The IPv4 CIDR destination address to which Amazon should send traffic.
        /// Required for IPv4 BGP peers on public virtual interfaces.
        pub customer_address: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Direct Connect virtual interface on which to create the BGP peer.
        pub virtual_interface_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: BgpPeerArgs,
    ) -> BgpPeerResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let address_family_binding = args.address_family.get_output(context);
        let amazon_address_binding = args.amazon_address.get_output(context);
        let bgp_asn_binding = args.bgp_asn.get_output(context);
        let bgp_auth_key_binding = args.bgp_auth_key.get_output(context);
        let customer_address_binding = args.customer_address.get_output(context);
        let virtual_interface_id_binding = args.virtual_interface_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:directconnect/bgpPeer:BgpPeer".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "addressFamily".into(),
                    value: address_family_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "amazonAddress".into(),
                    value: amazon_address_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "bgpAsn".into(),
                    value: bgp_asn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "bgpAuthKey".into(),
                    value: bgp_auth_key_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "customerAddress".into(),
                    value: customer_address_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "virtualInterfaceId".into(),
                    value: virtual_interface_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        BgpPeerResult {
            address_family: o.get_field("addressFamily"),
            amazon_address: o.get_field("amazonAddress"),
            aws_device: o.get_field("awsDevice"),
            bgp_asn: o.get_field("bgpAsn"),
            bgp_auth_key: o.get_field("bgpAuthKey"),
            bgp_peer_id: o.get_field("bgpPeerId"),
            bgp_status: o.get_field("bgpStatus"),
            customer_address: o.get_field("customerAddress"),
            virtual_interface_id: o.get_field("virtualInterfaceId"),
        }
    }
}
