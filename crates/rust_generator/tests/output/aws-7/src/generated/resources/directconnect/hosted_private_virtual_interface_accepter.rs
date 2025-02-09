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
///       dependsOn:
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
///       function: aws:getCallerIdentity
///       arguments: {}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Direct Connect hosted private virtual interfaces using the VIF `id`. For example:
///
/// ```sh
/// $ pulumi import aws:directconnect/hostedPrivateVirtualInterfaceAccepter:HostedPrivateVirtualInterfaceAccepter test dxvif-33cc44dd
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod hosted_private_virtual_interface_accepter {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct HostedPrivateVirtualInterfaceAccepterArgs {
        /// The ID of the Direct Connect gateway to which to connect the virtual interface.
        #[builder(into, default)]
        pub dx_gateway_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A map of tags to assign to the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The ID of the Direct Connect virtual interface to accept.
        #[builder(into)]
        pub virtual_interface_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the virtual private gateway to which to connect the virtual interface.
        #[builder(into, default)]
        pub vpn_gateway_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct HostedPrivateVirtualInterfaceAccepterResult {
        /// The ARN of the virtual interface.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Direct Connect gateway to which to connect the virtual interface.
        pub dx_gateway_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// A map of tags to assign to the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The ID of the Direct Connect virtual interface to accept.
        pub virtual_interface_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the virtual private gateway to which to connect the virtual interface.
        pub vpn_gateway_id: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: HostedPrivateVirtualInterfaceAccepterArgs,
    ) -> HostedPrivateVirtualInterfaceAccepterResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let dx_gateway_id_binding = args.dx_gateway_id.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let virtual_interface_id_binding = args.virtual_interface_id.get_output(context);
        let vpn_gateway_id_binding = args.vpn_gateway_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:directconnect/hostedPrivateVirtualInterfaceAccepter:HostedPrivateVirtualInterfaceAccepter"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dxGatewayId".into(),
                    value: dx_gateway_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "virtualInterfaceId".into(),
                    value: virtual_interface_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vpnGatewayId".into(),
                    value: vpn_gateway_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        HostedPrivateVirtualInterfaceAccepterResult {
            arn: o.get_field("arn"),
            dx_gateway_id: o.get_field("dxGatewayId"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            virtual_interface_id: o.get_field("virtualInterfaceId"),
            vpn_gateway_id: o.get_field("vpnGatewayId"),
        }
    }
}
