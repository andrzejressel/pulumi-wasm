#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_tunnel {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetTunnelArgs {
        /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub account_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// If true, only include deleted tunnels. If false, exclude deleted tunnels. If empty, all tunnels will be included. **Modifying this attribute will force creation of a new resource.**
        #[builder(into, default)]
        pub is_deleted: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Name of the tunnel. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetTunnelResult {
        /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        pub account_id: pulumi_gestalt_rust::Output<String>,
        /// ID of the tunnel.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// If true, only include deleted tunnels. If false, exclude deleted tunnels. If empty, all tunnels will be included. **Modifying this attribute will force creation of a new resource.**
        pub is_deleted: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Name of the tunnel. **Modifying this attribute will force creation of a new resource.**
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Whether the tunnel can be configured remotely from the Zero Trust dashboard.
        pub remote_config: pulumi_gestalt_rust::Output<bool>,
        /// The status of the tunnel. Available values: `inactive`, `degraded`, `healthy`, `down`.
        pub status: pulumi_gestalt_rust::Output<String>,
        /// The type of the tunnel. Available values: `cfd_tunnel`, `warp_connector`.
        pub tunnel_type: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetTunnelArgs,
    ) -> GetTunnelResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let account_id_binding = args.account_id.get_output(context);
        let is_deleted_binding = args.is_deleted.get_output(context);
        let name_binding = args.name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "cloudflare:index/getTunnel:getTunnel".into(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountId".into(),
                    value: account_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "isDeleted".into(),
                    value: is_deleted_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetTunnelResult {
            account_id: o.get_field("accountId"),
            id: o.get_field("id"),
            is_deleted: o.get_field("isDeleted"),
            name: o.get_field("name"),
            remote_config: o.get_field("remoteConfig"),
            status: o.get_field("status"),
            tunnel_type: o.get_field("tunnelType"),
        }
    }
}
