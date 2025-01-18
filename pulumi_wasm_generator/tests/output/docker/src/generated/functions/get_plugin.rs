pub mod get_plugin {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetPluginArgs {
        /// The alias of the Docker plugin. If the tag is omitted, `:latest` is complemented to the attribute value.
        #[builder(into, default)]
        pub alias: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the plugin, which has precedence over the `alias` of both are given
        #[builder(into, default)]
        pub id: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetPluginResult {
        /// The alias of the Docker plugin. If the tag is omitted, `:latest` is complemented to the attribute value.
        pub alias: pulumi_wasm_rust::Output<Option<String>>,
        /// If `true` the plugin is enabled
        pub enabled: pulumi_wasm_rust::Output<bool>,
        /// The environment variables in the form of `KEY=VALUE`, e.g. `DEBUG=0`
        pub envs: pulumi_wasm_rust::Output<Vec<String>>,
        /// If true, grant all permissions necessary to run the plugin
        pub grant_all_permissions: pulumi_wasm_rust::Output<bool>,
        /// The ID of the plugin, which has precedence over the `alias` of both are given
        pub id: pulumi_wasm_rust::Output<Option<String>>,
        /// The plugin name. If the tag is omitted, `:latest` is complemented to the attribute value.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The Docker Plugin Reference
        pub plugin_reference: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetPluginArgs) -> GetPluginResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let alias_binding = args.alias.get_inner();
        let id_binding = args.id.get_inner();
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "alias".into(),
                },
                register_interface::ResultField {
                    name: "enabled".into(),
                },
                register_interface::ResultField {
                    name: "envs".into(),
                },
                register_interface::ResultField {
                    name: "grantAllPermissions".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "pluginReference".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetPluginResult {
            alias: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("alias").unwrap(),
            ),
            enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enabled").unwrap(),
            ),
            envs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("envs").unwrap(),
            ),
            grant_all_permissions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("grantAllPermissions").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            plugin_reference: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pluginReference").unwrap(),
            ),
        }
    }
}
