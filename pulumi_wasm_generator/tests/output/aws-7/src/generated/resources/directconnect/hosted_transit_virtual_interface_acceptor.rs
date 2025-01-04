/// Provides a resource to manage the accepter's side of a Direct Connect hosted transit virtual interface.
/// This resource accepts ownership of a transit virtual interface created by another AWS account.
///
/// > **NOTE:** AWS allows a Direct Connect hosted transit virtual interface to be deleted from either the allocator's or accepter's side. However, this provider only allows the Direct Connect hosted transit virtual interface to be deleted from the allocator's side by removing the corresponding `aws.directconnect.HostedTransitVirtualInterface` resource from your configuration. Removing a `aws.directconnect.HostedTransitVirtualInterfaceAcceptor` resource from your configuration will remove it from your statefile and management, **but will not delete the Direct Connect virtual interface.**
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   # Creator's side of the VIF
///   creator:
///     type: aws:directconnect:HostedTransitVirtualInterface
///     properties:
///       connectionId: dxcon-zzzzzzzz
///       ownerAccountId: ${accepter.accountId}
///       name: tf-transit-vif-example
///       vlan: 4094
///       addressFamily: ipv4
///       bgpAsn: 65352 # The aws_dx_hosted_transit_virtual_interface
///       #   # must be destroyed before the aws_dx_gateway.
///     options:
///       dependsOn:
///         - ${example}
///   # Accepter's side of the VIF.
///   example:
///     type: aws:directconnect:Gateway
///     properties:
///       name: tf-dxg-example
///       amazonSideAsn: 64512
///   accepterHostedTransitVirtualInterfaceAcceptor:
///     type: aws:directconnect:HostedTransitVirtualInterfaceAcceptor
///     name: accepter
///     properties:
///       virtualInterfaceId: ${creator.id}
///       dxGatewayId: ${example.id}
///       tags:
///         Side: Accepter
/// variables:
///   accepter:
///     fn::invoke:
///       function: aws:getCallerIdentity
///       arguments: {}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Direct Connect hosted transit virtual interfaces using the VIF `id`. For example:
///
/// ```sh
/// $ pulumi import aws:directconnect/hostedTransitVirtualInterfaceAcceptor:HostedTransitVirtualInterfaceAcceptor test dxvif-33cc44dd
/// ```
pub mod hosted_transit_virtual_interface_acceptor {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct HostedTransitVirtualInterfaceAcceptorArgs {
        /// The ID of the Direct Connect gateway to which to connect the virtual interface.
        #[builder(into)]
        pub dx_gateway_id: pulumi_wasm_rust::Output<String>,
        /// A map of tags to assign to the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The ID of the Direct Connect virtual interface to accept.
        #[builder(into)]
        pub virtual_interface_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct HostedTransitVirtualInterfaceAcceptorResult {
        /// The ARN of the virtual interface.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The ID of the Direct Connect gateway to which to connect the virtual interface.
        pub dx_gateway_id: pulumi_wasm_rust::Output<String>,
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
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: HostedTransitVirtualInterfaceAcceptorArgs,
    ) -> HostedTransitVirtualInterfaceAcceptorResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let dx_gateway_id_binding = args.dx_gateway_id.get_inner();
        let tags_binding = args.tags.get_inner();
        let virtual_interface_id_binding = args.virtual_interface_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:directconnect/hostedTransitVirtualInterfaceAcceptor:HostedTransitVirtualInterfaceAcceptor"
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
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        HostedTransitVirtualInterfaceAcceptorResult {
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
        }
    }
}
