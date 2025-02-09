/// Provides a Direct Connect hosted private virtual interface resource. This resource represents the allocator's side of the hosted virtual interface.
/// A hosted virtual interface is a virtual interface that is owned by another AWS account.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let foo = hosted_private_virtual_interface::create(
///         "foo",
///         HostedPrivateVirtualInterfaceArgs::builder()
///             .address_family("ipv4")
///             .bgp_asn(65352)
///             .connection_id("dxcon-zzzzzzzz")
///             .name("vif-foo")
///             .vlan(4094)
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Direct Connect hosted private virtual interfaces using the VIF `id`. For example:
///
/// ```sh
/// $ pulumi import aws:directconnect/hostedPrivateVirtualInterface:HostedPrivateVirtualInterface test dxvif-33cc44dd
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod hosted_private_virtual_interface {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct HostedPrivateVirtualInterfaceArgs {
        /// The address family for the BGP peer. `ipv4 ` or `ipv6`.
        #[builder(into)]
        pub address_family: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The IPv4 CIDR address to use to send traffic to Amazon. Required for IPv4 BGP peers.
        #[builder(into, default)]
        pub amazon_address: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The autonomous system (AS) number for Border Gateway Protocol (BGP) configuration.
        #[builder(into)]
        pub bgp_asn: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// The authentication key for BGP configuration.
        #[builder(into, default)]
        pub bgp_auth_key: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the Direct Connect connection (or LAG) on which to create the virtual interface.
        #[builder(into)]
        pub connection_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The IPv4 CIDR destination address to which Amazon should send traffic. Required for IPv4 BGP peers.
        #[builder(into, default)]
        pub customer_address: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The maximum transmission unit (MTU) is the size, in bytes, of the largest permissible packet that can be passed over the connection. The MTU of a virtual private interface can be either `1500` or `9001` (jumbo frames). Default is `1500`.
        #[builder(into, default)]
        pub mtu: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The name for the virtual interface.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The AWS account that will own the new virtual interface.
        #[builder(into)]
        pub owner_account_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The VLAN ID.
        #[builder(into)]
        pub vlan: pulumi_gestalt_rust::InputOrOutput<i32>,
    }
    #[allow(dead_code)]
    pub struct HostedPrivateVirtualInterfaceResult {
        /// The address family for the BGP peer. `ipv4 ` or `ipv6`.
        pub address_family: pulumi_gestalt_rust::Output<String>,
        /// The IPv4 CIDR address to use to send traffic to Amazon. Required for IPv4 BGP peers.
        pub amazon_address: pulumi_gestalt_rust::Output<String>,
        pub amazon_side_asn: pulumi_gestalt_rust::Output<String>,
        /// The ARN of the virtual interface.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The Direct Connect endpoint on which the virtual interface terminates.
        pub aws_device: pulumi_gestalt_rust::Output<String>,
        /// The autonomous system (AS) number for Border Gateway Protocol (BGP) configuration.
        pub bgp_asn: pulumi_gestalt_rust::Output<i32>,
        /// The authentication key for BGP configuration.
        pub bgp_auth_key: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Direct Connect connection (or LAG) on which to create the virtual interface.
        pub connection_id: pulumi_gestalt_rust::Output<String>,
        /// The IPv4 CIDR destination address to which Amazon should send traffic. Required for IPv4 BGP peers.
        pub customer_address: pulumi_gestalt_rust::Output<String>,
        /// Indicates whether jumbo frames (9001 MTU) are supported.
        pub jumbo_frame_capable: pulumi_gestalt_rust::Output<bool>,
        /// The maximum transmission unit (MTU) is the size, in bytes, of the largest permissible packet that can be passed over the connection. The MTU of a virtual private interface can be either `1500` or `9001` (jumbo frames). Default is `1500`.
        pub mtu: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The name for the virtual interface.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The AWS account that will own the new virtual interface.
        pub owner_account_id: pulumi_gestalt_rust::Output<String>,
        /// The VLAN ID.
        pub vlan: pulumi_gestalt_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: HostedPrivateVirtualInterfaceArgs,
    ) -> HostedPrivateVirtualInterfaceResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let address_family_binding_1 = args.address_family.get_output(context);
        let address_family_binding = address_family_binding_1.get_inner();
        let amazon_address_binding_1 = args.amazon_address.get_output(context);
        let amazon_address_binding = amazon_address_binding_1.get_inner();
        let bgp_asn_binding_1 = args.bgp_asn.get_output(context);
        let bgp_asn_binding = bgp_asn_binding_1.get_inner();
        let bgp_auth_key_binding_1 = args.bgp_auth_key.get_output(context);
        let bgp_auth_key_binding = bgp_auth_key_binding_1.get_inner();
        let connection_id_binding_1 = args.connection_id.get_output(context);
        let connection_id_binding = connection_id_binding_1.get_inner();
        let customer_address_binding_1 = args.customer_address.get_output(context);
        let customer_address_binding = customer_address_binding_1.get_inner();
        let mtu_binding_1 = args.mtu.get_output(context);
        let mtu_binding = mtu_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let owner_account_id_binding_1 = args.owner_account_id.get_output(context);
        let owner_account_id_binding = owner_account_id_binding_1.get_inner();
        let vlan_binding_1 = args.vlan.get_output(context);
        let vlan_binding = vlan_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:directconnect/hostedPrivateVirtualInterface:HostedPrivateVirtualInterface"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "addressFamily".into(),
                    value: &address_family_binding,
                },
                register_interface::ObjectField {
                    name: "amazonAddress".into(),
                    value: &amazon_address_binding,
                },
                register_interface::ObjectField {
                    name: "bgpAsn".into(),
                    value: &bgp_asn_binding,
                },
                register_interface::ObjectField {
                    name: "bgpAuthKey".into(),
                    value: &bgp_auth_key_binding,
                },
                register_interface::ObjectField {
                    name: "connectionId".into(),
                    value: &connection_id_binding,
                },
                register_interface::ObjectField {
                    name: "customerAddress".into(),
                    value: &customer_address_binding,
                },
                register_interface::ObjectField {
                    name: "mtu".into(),
                    value: &mtu_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "ownerAccountId".into(),
                    value: &owner_account_id_binding,
                },
                register_interface::ObjectField {
                    name: "vlan".into(),
                    value: &vlan_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        HostedPrivateVirtualInterfaceResult {
            address_family: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("addressFamily"),
            ),
            amazon_address: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("amazonAddress"),
            ),
            amazon_side_asn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("amazonSideAsn"),
            ),
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            aws_device: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("awsDevice"),
            ),
            bgp_asn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("bgpAsn"),
            ),
            bgp_auth_key: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("bgpAuthKey"),
            ),
            connection_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("connectionId"),
            ),
            customer_address: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("customerAddress"),
            ),
            jumbo_frame_capable: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("jumboFrameCapable"),
            ),
            mtu: pulumi_gestalt_rust::__private::into_domain(o.extract_field("mtu")),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            owner_account_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ownerAccountId"),
            ),
            vlan: pulumi_gestalt_rust::__private::into_domain(o.extract_field("vlan")),
        }
    }
}
