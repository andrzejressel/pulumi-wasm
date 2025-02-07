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
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: PluginArgs,
    ) -> PluginResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let alias_binding = args.alias.get_output(context).get_inner();
        let enable_timeout_binding = args.enable_timeout.get_output(context).get_inner();
        let enabled_binding = args.enabled.get_output(context).get_inner();
        let envs_binding = args.envs.get_output(context).get_inner();
        let force_destroy_binding = args.force_destroy.get_output(context).get_inner();
        let force_disable_binding = args.force_disable.get_output(context).get_inner();
        let grant_all_permissions_binding = args
            .grant_all_permissions
            .get_output(context)
            .get_inner();
        let grant_permissions_binding = args
            .grant_permissions
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "docker:index/plugin:Plugin".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "alias".into(),
                    value: &alias_binding,
                },
                register_interface::ObjectField {
                    name: "enableTimeout".into(),
                    value: &enable_timeout_binding,
                },
                register_interface::ObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding,
                },
                register_interface::ObjectField {
                    name: "envs".into(),
                    value: &envs_binding,
                },
                register_interface::ObjectField {
                    name: "forceDestroy".into(),
                    value: &force_destroy_binding,
                },
                register_interface::ObjectField {
                    name: "forceDisable".into(),
                    value: &force_disable_binding,
                },
                register_interface::ObjectField {
                    name: "grantAllPermissions".into(),
                    value: &grant_all_permissions_binding,
                },
                register_interface::ObjectField {
                    name: "grantPermissions".into(),
                    value: &grant_permissions_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        PluginResult {
            alias: pulumi_gestalt_rust::__private::into_domain(o.extract_field("alias")),
            enable_timeout: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("enableTimeout"),
            ),
            enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("enabled"),
            ),
            envs: pulumi_gestalt_rust::__private::into_domain(o.extract_field("envs")),
            force_destroy: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("forceDestroy"),
            ),
            force_disable: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("forceDisable"),
            ),
            grant_all_permissions: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("grantAllPermissions"),
            ),
            grant_permissions: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("grantPermissions"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            plugin_reference: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("pluginReference"),
            ),
        }
    }
}
