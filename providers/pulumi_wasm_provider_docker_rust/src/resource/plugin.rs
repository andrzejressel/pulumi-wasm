pub struct PluginArgs {
    pub alias: pulumi_wasm_rust::Output<Option<String>>,
    pub enable_timeout: pulumi_wasm_rust::Output<Option<i32>>,
    pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
    pub envs: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    pub force_destroy: pulumi_wasm_rust::Output<Option<bool>>,
    pub force_disable: pulumi_wasm_rust::Output<Option<bool>>,
    pub grant_all_permissions: pulumi_wasm_rust::Output<Option<bool>>,
    pub grant_permissions:
        pulumi_wasm_rust::Output<Option<Vec<crate::types::PluginGrantPermission>>>,
    pub name: pulumi_wasm_rust::Output<Option<String>>,
}

pub struct PluginResult {
    pub alias: pulumi_wasm_rust::Output<String>,
    pub enable_timeout: pulumi_wasm_rust::Output<Option<i32>>,
    pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
    pub envs: pulumi_wasm_rust::Output<Vec<String>>,
    pub force_destroy: pulumi_wasm_rust::Output<Option<bool>>,
    pub force_disable: pulumi_wasm_rust::Output<Option<bool>>,
    pub grant_all_permissions: pulumi_wasm_rust::Output<Option<bool>>,
    pub grant_permissions:
        pulumi_wasm_rust::Output<Option<Vec<crate::types::PluginGrantPermission>>>,
    pub name: pulumi_wasm_rust::Output<String>,
    pub plugin_reference: pulumi_wasm_rust::Output<String>,
}

pub fn create(name: &str, args: PluginArgs) -> PluginResult {
    let result = crate::bindings::pulumi::docker::plugin::invoke(
        name,
        &crate::bindings::pulumi::docker::plugin::Args {
            alias: args.alias.get_inner(),
            enable_timeout: args.enable_timeout.get_inner(),
            enabled: args.enabled.get_inner(),
            envs: args.envs.get_inner(),
            force_destroy: args.force_destroy.get_inner(),
            force_disable: args.force_disable.get_inner(),
            grant_all_permissions: args.grant_all_permissions.get_inner(),
            grant_permissions: args.grant_permissions.get_inner(),
            name: args.name.get_inner(),
        },
    );

    PluginResult {
        alias: crate::into_domain(result.alias),
        enable_timeout: crate::into_domain(result.enable_timeout),
        enabled: crate::into_domain(result.enabled),
        envs: crate::into_domain(result.envs),
        force_destroy: crate::into_domain(result.force_destroy),
        force_disable: crate::into_domain(result.force_disable),
        grant_all_permissions: crate::into_domain(result.grant_all_permissions),
        grant_permissions: crate::into_domain(result.grant_permissions),
        name: crate::into_domain(result.name),
        plugin_reference: crate::into_domain(result.plugin_reference),
    }
}
