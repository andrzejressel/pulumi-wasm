/// Provides a resource to manage the accepter's side of a Direct Connect hosted private virtual interface.
/// This resource accepts ownership of a private virtual interface created by another AWS account.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   # Creator's side of the VIF
///   creator:
///     type: aws:directconnect:HostedPrivateVirtualInterface
///     properties:
///       connectionId: dxcon-zzzzzzzz
///       ownerAccountId: ${accepter.accountId}
///       name: vif-foo
///       vlan: 4094
///       addressFamily: ipv4
///       bgpAsn: 65352 # The aws_dx_hosted_private_virtual_interface
///       #   # must be destroyed before the aws_vpn_gateway.
///     options:
///       dependson:
///         - ${vpnGw}
///   # Accepter's side of the VIF.
///   vpnGw:
///     type: aws:ec2:VpnGateway
///     name: vpn_gw
///   accepterHostedPrivateVirtualInterfaceAccepter:
///     type: aws:directconnect:HostedPrivateVirtualInterfaceAccepter
///     name: accepter
///     properties:
///       virtualInterfaceId: ${creator.id}
///       vpnGatewayId: ${vpnGw.id}
///       tags:
///         Side: Accepter
/// variables:
///   accepter:
///     fn::invoke:
///       Function: aws:getCallerIdentity
///       Arguments: {}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Direct Connect hosted private virtual interfaces using the VIF `id`. For example:
///
/// ```sh
/// $ pulumi import aws:directconnect/hostedPrivateVirtualInterfaceAccepter:HostedPrivateVirtualInterfaceAccepter test dxvif-33cc44dd
/// ```
pub mod hosted_private_virtual_interface_accepter {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct HostedPrivateVirtualInterfaceAccepterArgs {
        /// The ID of the Direct Connect gateway to which to connect the virtual interface.
        #[builder(into, default)]
        pub dx_gateway_id: pulumi_wasm_rust::Output<Option<String>>,
        /// A map of tags to assign to the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The ID of the Direct Connect virtual interface to accept.
        #[builder(into)]
        pub virtual_interface_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the virtual private gateway to which to connect the virtual interface.
        #[builder(into, default)]
        pub vpn_gateway_id: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct HostedPrivateVirtualInterfaceAccepterResult {
        /// The ARN of the virtual interface.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The ID of the Direct Connect gateway to which to connect the virtual interface.
        pub dx_gateway_id: pulumi_wasm_rust::Output<Option<String>>,
        /// A map of tags to assign to the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The ID of the Direct Connect virtual interface to accept.
        pub virtual_interface_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the virtual private gateway to which to connect the virtual interface.
        pub vpn_gateway_id: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: HostedPrivateVirtualInterfaceAccepterArgs,
    ) -> HostedPrivateVirtualInterfaceAccepterResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let dx_gateway_id_binding = args.dx_gateway_id.get_inner();
        let tags_binding = args.tags.get_inner();
        let virtual_interface_id_binding = args.virtual_interface_id.get_inner();
        let vpn_gateway_id_binding = args.vpn_gateway_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:directconnect/hostedPrivateVirtualInterfaceAccepter:HostedPrivateVirtualInterfaceAccepter"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "dxGatewayId".into(),
                    value: &dx_gateway_id_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "virtualInterfaceId".into(),
                    value: &virtual_interface_id_binding,
                },
                register_interface::ObjectField {
                    name: "vpnGatewayId".into(),
                    value: &vpn_gateway_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "dxGatewayId".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "virtualInterfaceId".into(),
                },
                register_interface::ResultField {
                    name: "vpnGatewayId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        HostedPrivateVirtualInterfaceAccepterResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            dx_gateway_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dxGatewayId").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            virtual_interface_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("virtualInterfaceId").unwrap(),
            ),
            vpn_gateway_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpnGatewayId").unwrap(),
            ),
        }
    }
}