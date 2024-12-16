//! Reads the local Docker plugin. The plugin must be installed locally.
//! 
//! ## Example Usage
//! 
//! ### With alias
//! ```yaml
//! variables:
//!   byAlias:
//!     fn::docker:getPlugin:
//!       alias: "sample-volume-plugin:latest"
//! ```
//! 
//! ### With ID
//! ```yaml
//! variables:
//!   byId:
//!     fn::docker:getPlugin:
//!       id: "e9a9db917b3bfd6706b5d3a66d4bceb9f"
//! ```

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
pub fn invoke(
    args: GetPluginArgs
) -> GetPluginResult {

    let result = crate::bindings::pulumi::docker::get_plugin::invoke(
        &crate::bindings::pulumi::docker::get_plugin::Args {
                alias: &args.alias.get_inner(),
                id: &args.id.get_inner(),
        }
    );

    GetPluginResult {
        alias: crate::into_domain(result.alias),
        enabled: crate::into_domain(result.enabled),
        envs: crate::into_domain(result.envs),
        grant_all_permissions: crate::into_domain(result.grant_all_permissions),
        id: crate::into_domain(result.id),
        name: crate::into_domain(result.name),
        plugin_reference: crate::into_domain(result.plugin_reference),
    }
}
