#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_plugin {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetPluginArgs {
        /// The alias of the Docker plugin. If the tag is omitted, `:latest` is complemented to the attribute value.
        #[builder(into, default)]
        pub alias: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the plugin, which has precedence over the `alias` of both are given
        #[builder(into, default)]
        pub id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetPluginResult {
        /// The alias of the Docker plugin. If the tag is omitted, `:latest` is complemented to the attribute value.
        pub alias: pulumi_gestalt_rust::Output<Option<String>>,
        /// If `true` the plugin is enabled
        pub enabled: pulumi_gestalt_rust::Output<bool>,
        /// The environment variables in the form of `KEY=VALUE`, e.g. `DEBUG=0`
        pub envs: pulumi_gestalt_rust::Output<Vec<String>>,
        /// If true, grant all permissions necessary to run the plugin
        pub grant_all_permissions: pulumi_gestalt_rust::Output<bool>,
        /// The ID of the plugin, which has precedence over the `alias` of both are given
        pub id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The plugin name. If the tag is omitted, `:latest` is complemented to the attribute value.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The Docker Plugin Reference
        pub plugin_reference: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetPluginArgs,
    ) -> GetPluginResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let alias_binding = args.alias.get_output(context).get_inner();
        let id_binding = args.id.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "docker:index/getPlugin:getPlugin".into(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "alias".into(),
                    value: &alias_binding,
                },
                register_interface::ObjectField {
                    name: "id".into(),
                    value: &id_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetPluginResult {
            alias: pulumi_gestalt_rust::__private::into_domain(o.extract_field("alias")),
            enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("enabled"),
            ),
            envs: pulumi_gestalt_rust::__private::into_domain(o.extract_field("envs")),
            grant_all_permissions: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("grantAllPermissions"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            plugin_reference: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("pluginReference"),
            ),
        }
    }
}
