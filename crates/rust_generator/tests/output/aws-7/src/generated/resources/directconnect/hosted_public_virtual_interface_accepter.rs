/// Provides a resource to manage the accepter's side of a Direct Connect hosted public virtual interface.
/// This resource accepts ownership of a public virtual interface created by another AWS account.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   # Creator's side of the VIF
///   creator:
///     type: aws:directconnect:HostedPublicVirtualInterface
///     properties:
///       connectionId: dxcon-zzzzzzzz
///       ownerAccountId: ${accepter.accountId}
///       name: vif-foo
///       vlan: 4094
///       addressFamily: ipv4
///       bgpAsn: 65352
///       customerAddress: 175.45.176.1/30
///       amazonAddress: 175.45.176.2/30
///       routeFilterPrefixes:
///         - 210.52.109.0/24
///         - 175.45.176.0/22
///   # Accepter's side of the VIF.
///   accepterHostedPublicVirtualInterfaceAccepter:
///     type: aws:directconnect:HostedPublicVirtualInterfaceAccepter
///     name: accepter
///     properties:
///       virtualInterfaceId: ${creator.id}
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
/// Using `pulumi import`, import Direct Connect hosted public virtual interfaces using the VIF `id`. For example:
///
/// ```sh
/// $ pulumi import aws:directconnect/hostedPublicVirtualInterfaceAccepter:HostedPublicVirtualInterfaceAccepter test dxvif-33cc44dd
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod hosted_public_virtual_interface_accepter {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct HostedPublicVirtualInterfaceAccepterArgs {
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
    pub struct HostedPublicVirtualInterfaceAccepterResult {
        /// The ARN of the virtual interface.
        pub arn: pulumi_gestalt_rust::Output<String>,
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
        args: HostedPublicVirtualInterfaceAccepterArgs,
    ) -> HostedPublicVirtualInterfaceAccepterResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let tags_binding = args.tags.get_output(context);
        let virtual_interface_id_binding = args.virtual_interface_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:directconnect/hostedPublicVirtualInterfaceAccepter:HostedPublicVirtualInterfaceAccepter"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "virtualInterfaceId".into(),
                    value: &virtual_interface_id_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        HostedPublicVirtualInterfaceAccepterResult {
            arn: o.get_field("arn"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            virtual_interface_id: o.get_field("virtualInterfaceId"),
        }
    }
}
