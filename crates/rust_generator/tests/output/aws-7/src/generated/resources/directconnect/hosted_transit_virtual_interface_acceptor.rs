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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod hosted_transit_virtual_interface_acceptor {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct HostedTransitVirtualInterfaceAcceptorArgs {
        /// The ID of the Direct Connect gateway to which to connect the virtual interface.
        #[builder(into)]
        pub dx_gateway_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A map of tags to assign to the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The ID of the Direct Connect virtual interface to accept.
        #[builder(into)]
        pub virtual_interface_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct HostedTransitVirtualInterfaceAcceptorResult {
        /// The ARN of the virtual interface.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Direct Connect gateway to which to connect the virtual interface.
        pub dx_gateway_id: pulumi_gestalt_rust::Output<String>,
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
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: HostedTransitVirtualInterfaceAcceptorArgs,
    ) -> HostedTransitVirtualInterfaceAcceptorResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let dx_gateway_id_binding = args.dx_gateway_id.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let virtual_interface_id_binding = args.virtual_interface_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:directconnect/hostedTransitVirtualInterfaceAcceptor:HostedTransitVirtualInterfaceAcceptor"
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
            ],
        };
        let o = context.register_resource(request);
        HostedTransitVirtualInterfaceAcceptorResult {
            arn: o.get_field("arn"),
            dx_gateway_id: o.get_field("dxGatewayId"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            virtual_interface_id: o.get_field("virtualInterfaceId"),
        }
    }
}
