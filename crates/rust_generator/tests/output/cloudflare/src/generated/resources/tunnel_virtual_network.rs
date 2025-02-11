/// Provides a resource, that manages Cloudflare tunnel virtual networks
/// for Zero Trust. Tunnel virtual networks are used for segregation of
/// Tunnel IP Routes via Virtualized Networks to handle overlapping
/// private IPs in your origins.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = tunnel_virtual_network::create(
///         "example",
///         TunnelVirtualNetworkArgs::builder()
///             .account_id("f037e56e89293a057740de681ac9abbe")
///             .comment("New tunnel virtual network for documentation")
///             .name("vnet-for-documentation")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// ```sh
/// $ pulumi import cloudflare:index/tunnelVirtualNetwork:TunnelVirtualNetwork example <account_id>/<vnet_id>
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod tunnel_virtual_network {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TunnelVirtualNetworkArgs {
        /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub account_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Description of the tunnel virtual network.
        #[builder(into, default)]
        pub comment: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Whether this virtual network is the default one for the account. This means IP Routes belong to this virtual network and Teams Clients in the account route through this virtual network, unless specified otherwise for each case.
        #[builder(into, default)]
        pub is_default_network: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// A user-friendly name chosen when the virtual network is created.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct TunnelVirtualNetworkResult {
        /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        pub account_id: pulumi_gestalt_rust::Output<String>,
        /// Description of the tunnel virtual network.
        pub comment: pulumi_gestalt_rust::Output<Option<String>>,
        /// Whether this virtual network is the default one for the account. This means IP Routes belong to this virtual network and Teams Clients in the account route through this virtual network, unless specified otherwise for each case.
        pub is_default_network: pulumi_gestalt_rust::Output<Option<bool>>,
        /// A user-friendly name chosen when the virtual network is created.
        pub name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: TunnelVirtualNetworkArgs,
    ) -> TunnelVirtualNetworkResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let account_id_binding = args.account_id.get_output(context);
        let comment_binding = args.comment.get_output(context);
        let is_default_network_binding = args.is_default_network.get_output(context);
        let name_binding = args.name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "cloudflare:index/tunnelVirtualNetwork:TunnelVirtualNetwork".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "comment".into(),
                    value: &comment_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "isDefaultNetwork".into(),
                    value: &is_default_network_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        TunnelVirtualNetworkResult {
            account_id: o.get_field("accountId"),
            comment: o.get_field("comment"),
            is_default_network: o.get_field("isDefaultNetwork"),
            name: o.get_field("name"),
        }
    }
}
