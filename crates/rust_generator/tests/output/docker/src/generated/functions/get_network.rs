#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_network {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetNetworkArgs {
        /// The name of the Docker network.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetNetworkResult {
        /// The driver of the Docker network. Possible values are `bridge`, `host`, `overlay`, `macvlan`. See [network docs](https://docs.docker.com/network/#network-drivers) for more details.
        pub driver: pulumi_gestalt_rust::Output<String>,
        /// The ID of this resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// If `true`, the network is internal.
        pub internal: pulumi_gestalt_rust::Output<bool>,
        /// The IPAM configuration options
        pub ipam_configs: pulumi_gestalt_rust::Output<
            Vec<super::super::types::GetNetworkIpamConfig>,
        >,
        /// The name of the Docker network.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Only available with bridge networks. See [bridge options docs](https://docs.docker.com/engine/reference/commandline/network_create/#bridge-driver-options) for more details.
        pub options: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Scope of the network. One of `swarm`, `global`, or `local`.
        pub scope: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetNetworkArgs,
    ) -> GetNetworkResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "docker:index/getNetwork:getNetwork".into(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetNetworkResult {
            driver: o.get_field("driver"),
            id: o.get_field("id"),
            internal: o.get_field("internal"),
            ipam_configs: o.get_field("ipamConfigs"),
            name: o.get_field("name"),
            options: o.get_field("options"),
            scope: o.get_field("scope"),
        }
    }
}
