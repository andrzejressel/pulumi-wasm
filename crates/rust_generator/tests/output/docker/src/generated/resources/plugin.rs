/// <!-- Bug: Type and Name are switched -->
/// Manages the lifecycle of a Docker plugin.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   sample-volume-plugin:
///     type: docker:Plugin
///     properties:
///       alias: sample-volume-plugin
///       enableTimeout: 60
///       enabled: false
///       envs:
///         - DEBUG=1
///       forceDestroy: true
///       forceDisable: true
///       grantAllPermissions: true
/// ```
///
/// ## Import
///
/// #!/bin/bash
///
/// ```sh
/// $ pulumi import docker:index/plugin:Plugin sample-volume-plugin "$(docker plugin inspect -f {{.ID}} tiborvass/sample-volume-plugin:latest)"
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod plugin {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PluginArgs {
        /// Docker Plugin alias
        #[builder(into, default)]
        pub alias: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// HTTP client timeout to enable the plugin
        #[builder(into, default)]
        pub enable_timeout: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// If `true` the plugin is enabled. Defaults to `true`
        #[builder(into, default)]
        pub enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The environment variables in the form of `KEY=VALUE`, e.g. `DEBUG=0`
        #[builder(into, default)]
        pub envs: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// If true, then the plugin is destroyed forcibly
        #[builder(into, default)]
        pub force_destroy: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// If true, then the plugin is disabled forcibly
        #[builder(into, default)]
        pub force_disable: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// If true, grant all permissions necessary to run the plugin
        #[builder(into, default)]
        pub grant_all_permissions: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Grant specific permissions only
        #[builder(into, default)]
        pub grant_permissions: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::types::PluginGrantPermission>>,
        >,
        /// The name of the permission
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct PluginResult {
        /// Docker Plugin alias
        pub alias: pulumi_gestalt_rust::Output<String>,
        /// HTTP client timeout to enable the plugin
        pub enable_timeout: pulumi_gestalt_rust::Output<Option<i32>>,
        /// If `true` the plugin is enabled. Defaults to `true`
        pub enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The environment variables in the form of `KEY=VALUE`, e.g. `DEBUG=0`
        pub envs: pulumi_gestalt_rust::Output<Vec<String>>,
        /// If true, then the plugin is destroyed forcibly
        pub force_destroy: pulumi_gestalt_rust::Output<Option<bool>>,
        /// If true, then the plugin is disabled forcibly
        pub force_disable: pulumi_gestalt_rust::Output<Option<bool>>,
        /// If true, grant all permissions necessary to run the plugin
        pub grant_all_permissions: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Grant specific permissions only
        pub grant_permissions: pulumi_gestalt_rust::Output<
            Option<Vec<super::types::PluginGrantPermission>>,
        >,
        /// The name of the permission
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Docker Plugin Reference
        pub plugin_reference: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: PluginArgs,
    ) -> PluginResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let alias_binding = args.alias.get_output(context);
        let enable_timeout_binding = args.enable_timeout.get_output(context);
        let enabled_binding = args.enabled.get_output(context);
        let envs_binding = args.envs.get_output(context);
        let force_destroy_binding = args.force_destroy.get_output(context);
        let force_disable_binding = args.force_disable.get_output(context);
        let grant_all_permissions_binding = args
            .grant_all_permissions
            .get_output(context);
        let grant_permissions_binding = args.grant_permissions.get_output(context);
        let name_binding = args.name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "docker:index/plugin:Plugin".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "alias".into(),
                    value: alias_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enableTimeout".into(),
                    value: enable_timeout_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enabled".into(),
                    value: enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "envs".into(),
                    value: envs_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "forceDestroy".into(),
                    value: force_destroy_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "forceDisable".into(),
                    value: force_disable_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "grantAllPermissions".into(),
                    value: grant_all_permissions_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "grantPermissions".into(),
                    value: grant_permissions_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        PluginResult {
            alias: o.get_field("alias"),
            enable_timeout: o.get_field("enableTimeout"),
            enabled: o.get_field("enabled"),
            envs: o.get_field("envs"),
            force_destroy: o.get_field("forceDestroy"),
            force_disable: o.get_field("forceDisable"),
            grant_all_permissions: o.get_field("grantAllPermissions"),
            grant_permissions: o.get_field("grantPermissions"),
            name: o.get_field("name"),
            plugin_reference: o.get_field("pluginReference"),
        }
    }
}
