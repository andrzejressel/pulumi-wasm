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
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PluginArgs {
        /// Docker Plugin alias
        #[builder(into, default)]
        pub alias: pulumi_wasm_rust::Output<Option<String>>,
        /// HTTP client timeout to enable the plugin
        #[builder(into, default)]
        pub enable_timeout: pulumi_wasm_rust::Output<Option<i32>>,
        /// If `true` the plugin is enabled. Defaults to `true`
        #[builder(into, default)]
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The environment variables in the form of `KEY=VALUE`, e.g. `DEBUG=0`
        #[builder(into, default)]
        pub envs: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// If true, then the plugin is destroyed forcibly
        #[builder(into, default)]
        pub force_destroy: pulumi_wasm_rust::Output<Option<bool>>,
        /// If true, then the plugin is disabled forcibly
        #[builder(into, default)]
        pub force_disable: pulumi_wasm_rust::Output<Option<bool>>,
        /// If true, grant all permissions necessary to run the plugin
        #[builder(into, default)]
        pub grant_all_permissions: pulumi_wasm_rust::Output<Option<bool>>,
        /// Grant specific permissions only
        #[builder(into, default)]
        pub grant_permissions: pulumi_wasm_rust::Output<
            Option<Vec<super::types::PluginGrantPermission>>,
        >,
        /// The name of the permission
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct PluginResult {
        /// Docker Plugin alias
        pub alias: pulumi_wasm_rust::Output<String>,
        /// HTTP client timeout to enable the plugin
        pub enable_timeout: pulumi_wasm_rust::Output<Option<i32>>,
        /// If `true` the plugin is enabled. Defaults to `true`
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The environment variables in the form of `KEY=VALUE`, e.g. `DEBUG=0`
        pub envs: pulumi_wasm_rust::Output<Vec<String>>,
        /// If true, then the plugin is destroyed forcibly
        pub force_destroy: pulumi_wasm_rust::Output<Option<bool>>,
        /// If true, then the plugin is disabled forcibly
        pub force_disable: pulumi_wasm_rust::Output<Option<bool>>,
        /// If true, grant all permissions necessary to run the plugin
        pub grant_all_permissions: pulumi_wasm_rust::Output<Option<bool>>,
        /// Grant specific permissions only
        pub grant_permissions: pulumi_wasm_rust::Output<
            Option<Vec<super::types::PluginGrantPermission>>,
        >,
        /// The name of the permission
        pub name: pulumi_wasm_rust::Output<String>,
        /// Docker Plugin Reference
        pub plugin_reference: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: PluginArgs) -> PluginResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let alias_binding = args.alias.get_inner();
        let enable_timeout_binding = args.enable_timeout.get_inner();
        let enabled_binding = args.enabled.get_inner();
        let envs_binding = args.envs.get_inner();
        let force_destroy_binding = args.force_destroy.get_inner();
        let force_disable_binding = args.force_disable.get_inner();
        let grant_all_permissions_binding = args.grant_all_permissions.get_inner();
        let grant_permissions_binding = args.grant_permissions.get_inner();
        let name_binding = args.name.get_inner();
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "alias".into(),
                },
                register_interface::ResultField {
                    name: "enableTimeout".into(),
                },
                register_interface::ResultField {
                    name: "enabled".into(),
                },
                register_interface::ResultField {
                    name: "envs".into(),
                },
                register_interface::ResultField {
                    name: "forceDestroy".into(),
                },
                register_interface::ResultField {
                    name: "forceDisable".into(),
                },
                register_interface::ResultField {
                    name: "grantAllPermissions".into(),
                },
                register_interface::ResultField {
                    name: "grantPermissions".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "pluginReference".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        PluginResult {
            alias: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("alias").unwrap(),
            ),
            enable_timeout: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enableTimeout").unwrap(),
            ),
            enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enabled").unwrap(),
            ),
            envs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("envs").unwrap(),
            ),
            force_destroy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("forceDestroy").unwrap(),
            ),
            force_disable: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("forceDisable").unwrap(),
            ),
            grant_all_permissions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("grantAllPermissions").unwrap(),
            ),
            grant_permissions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("grantPermissions").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            plugin_reference: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pluginReference").unwrap(),
            ),
        }
    }
}
