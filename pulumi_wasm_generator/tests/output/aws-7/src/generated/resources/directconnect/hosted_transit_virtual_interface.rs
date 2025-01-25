/// Provides a Direct Connect hosted transit virtual interface resource.
/// This resource represents the allocator's side of the hosted virtual interface.
/// A hosted virtual interface is a virtual interface that is owned by another AWS account.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = hosted_transit_virtual_interface::create(
///         "example",
///         HostedTransitVirtualInterfaceArgs::builder()
///             .address_family("ipv4")
///             .bgp_asn(65352)
///             .connection_id("${exampleAwsDxConnection.id}")
///             .name("tf-transit-vif-example")
///             .vlan(4094)
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Direct Connect hosted transit virtual interfaces using the VIF `id`. For example:
///
/// ```sh
/// $ pulumi import aws:directconnect/hostedTransitVirtualInterface:HostedTransitVirtualInterface test dxvif-33cc44dd
/// ```
pub mod hosted_transit_virtual_interface {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct HostedTransitVirtualInterfaceArgs {
        /// The address family for the BGP peer. `ipv4 ` or `ipv6`.
        #[builder(into)]
        pub address_family: pulumi_wasm_rust::InputOrOutput<String>,
        /// The IPv4 CIDR address to use to send traffic to Amazon. Required for IPv4 BGP peers.
        #[builder(into, default)]
        pub amazon_address: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The autonomous system (AS) number for Border Gateway Protocol (BGP) configuration.
        #[builder(into)]
        pub bgp_asn: pulumi_wasm_rust::InputOrOutput<i32>,
        /// The authentication key for BGP configuration.
        #[builder(into, default)]
        pub bgp_auth_key: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The ID of the Direct Connect connection (or LAG) on which to create the virtual interface.
        #[builder(into)]
        pub connection_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The IPv4 CIDR destination address to which Amazon should send traffic. Required for IPv4 BGP peers.
        #[builder(into, default)]
        pub customer_address: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The maximum transmission unit (MTU) is the size, in bytes, of the largest permissible packet that can be passed over the connection. The MTU of a virtual transit interface can be either `1500` or `8500` (jumbo frames). Default is `1500`.
        #[builder(into, default)]
        pub mtu: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// The name for the virtual interface.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The AWS account that will own the new virtual interface.
        #[builder(into)]
        pub owner_account_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The VLAN ID.
        #[builder(into)]
        pub vlan: pulumi_wasm_rust::InputOrOutput<i32>,
    }
    #[allow(dead_code)]
    pub struct HostedTransitVirtualInterfaceResult {
        /// The address family for the BGP peer. `ipv4 ` or `ipv6`.
        pub address_family: pulumi_wasm_rust::Output<String>,
        /// The IPv4 CIDR address to use to send traffic to Amazon. Required for IPv4 BGP peers.
        pub amazon_address: pulumi_wasm_rust::Output<String>,
        pub amazon_side_asn: pulumi_wasm_rust::Output<String>,
        /// The ARN of the virtual interface.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The Direct Connect endpoint on which the virtual interface terminates.
        pub aws_device: pulumi_wasm_rust::Output<String>,
        /// The autonomous system (AS) number for Border Gateway Protocol (BGP) configuration.
        pub bgp_asn: pulumi_wasm_rust::Output<i32>,
        /// The authentication key for BGP configuration.
        pub bgp_auth_key: pulumi_wasm_rust::Output<String>,
        /// The ID of the Direct Connect connection (or LAG) on which to create the virtual interface.
        pub connection_id: pulumi_wasm_rust::Output<String>,
        /// The IPv4 CIDR destination address to which Amazon should send traffic. Required for IPv4 BGP peers.
        pub customer_address: pulumi_wasm_rust::Output<String>,
        /// Indicates whether jumbo frames (8500 MTU) are supported.
        pub jumbo_frame_capable: pulumi_wasm_rust::Output<bool>,
        /// The maximum transmission unit (MTU) is the size, in bytes, of the largest permissible packet that can be passed over the connection. The MTU of a virtual transit interface can be either `1500` or `8500` (jumbo frames). Default is `1500`.
        pub mtu: pulumi_wasm_rust::Output<Option<i32>>,
        /// The name for the virtual interface.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The AWS account that will own the new virtual interface.
        pub owner_account_id: pulumi_wasm_rust::Output<String>,
        /// The VLAN ID.
        pub vlan: pulumi_wasm_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: HostedTransitVirtualInterfaceArgs,
    ) -> HostedTransitVirtualInterfaceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let address_family_binding = args.address_family.get_output(context).get_inner();
        let amazon_address_binding = args.amazon_address.get_output(context).get_inner();
        let bgp_asn_binding = args.bgp_asn.get_output(context).get_inner();
        let bgp_auth_key_binding = args.bgp_auth_key.get_output(context).get_inner();
        let connection_id_binding = args.connection_id.get_output(context).get_inner();
        let customer_address_binding = args
            .customer_address
            .get_output(context)
            .get_inner();
        let mtu_binding = args.mtu.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let owner_account_id_binding = args
            .owner_account_id
            .get_output(context)
            .get_inner();
        let vlan_binding = args.vlan.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:directconnect/hostedTransitVirtualInterface:HostedTransitVirtualInterface"
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "addressFamily".into(),
                },
                register_interface::ResultField {
                    name: "amazonAddress".into(),
                },
                register_interface::ResultField {
                    name: "amazonSideAsn".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "awsDevice".into(),
                },
                register_interface::ResultField {
                    name: "bgpAsn".into(),
                },
                register_interface::ResultField {
                    name: "bgpAuthKey".into(),
                },
                register_interface::ResultField {
                    name: "connectionId".into(),
                },
                register_interface::ResultField {
                    name: "customerAddress".into(),
                },
                register_interface::ResultField {
                    name: "jumboFrameCapable".into(),
                },
                register_interface::ResultField {
                    name: "mtu".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "ownerAccountId".into(),
                },
                register_interface::ResultField {
                    name: "vlan".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        HostedTransitVirtualInterfaceResult {
            address_family: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("addressFamily").unwrap(),
            ),
            amazon_address: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("amazonAddress").unwrap(),
            ),
            amazon_side_asn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("amazonSideAsn").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            aws_device: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("awsDevice").unwrap(),
            ),
            bgp_asn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bgpAsn").unwrap(),
            ),
            bgp_auth_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bgpAuthKey").unwrap(),
            ),
            connection_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("connectionId").unwrap(),
            ),
            customer_address: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customerAddress").unwrap(),
            ),
            jumbo_frame_capable: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("jumboFrameCapable").unwrap(),
            ),
            mtu: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("mtu").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            owner_account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ownerAccountId").unwrap(),
            ),
            vlan: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vlan").unwrap(),
            ),
        }
    }
}
