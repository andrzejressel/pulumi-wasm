#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct GetPluginArgs {
    /// The alias of the Docker plugin. If the tag is omitted, `:latest` is complemented to the attribute value.
    #[builder(into, default)]
    pub alias: pulumi_wasm_rust::Output<Option<String>>,
    /// The ID of the plugin, which has precedence over the `alias` of both are given
    #[builder(into, default)]
    pub id: pulumi_wasm_rust::Output<Option<String>>,
}
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
pub fn invoke(args: GetPluginArgs) -> GetPluginResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let alias_binding = args.alias.get_inner();
    let id_binding = args.id.get_inner();
    let request = register_interface::ResourceInvokeRequest {
        token: "docker:index/getPlugin:getPlugin".into(),
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
        results: vec![
            register_interface::ResultField { name : "alias".into() },
            register_interface::ResultField { name : "enabled".into() },
            register_interface::ResultField { name : "envs".into() },
            register_interface::ResultField { name : "grantAllPermissions".into() },
            register_interface::ResultField { name : "id".into() },
            register_interface::ResultField { name : "name".into() },
            register_interface::ResultField { name : "pluginReference".into() },
        ],
    };
    fn into_domain<F: serde::Serialize>(output: WitOutput) -> Output<F> {
        unsafe { Output::<F>::new_from_handle(output) }
    }
    let o = register_interface::invoke(&request);
    let mut hashmap: HashMap<String, _> = o
        .fields
        .into_iter()
        .map(|f| (f.name, f.output))
        .collect();
    GetPluginResult {
        alias: into_domain(hashmap.remove("alias").unwrap()),
        enabled: into_domain(hashmap.remove("enabled").unwrap()),
        envs: into_domain(hashmap.remove("envs").unwrap()),
        grant_all_permissions: into_domain(
            hashmap.remove("grantAllPermissions").unwrap(),
        ),
        id: into_domain(hashmap.remove("id").unwrap()),
        name: into_domain(hashmap.remove("name").unwrap()),
        plugin_reference: into_domain(hashmap.remove("pluginReference").unwrap()),
    }
}
