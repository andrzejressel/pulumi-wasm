//! <!-- Bug: Type and Name are switched -->
//! Manages the lifecycle of a Docker plugin.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ```yaml
//! resources:
//!   sample-volume-plugin:
//!     type: docker:Plugin
//!     properties:
//!       alias: sample-volume-plugin
//!       enableTimeout: 60
//!       enabled: false
//!       envs:
//!         - DEBUG=1
//!       forceDestroy: true
//!       forceDisable: true
//!       grantAllPermissions: true
//! ```
//! <!--End PulumiCodeChooser -->
//! 
//! ## Import
//! 
//! #!/bin/bash
//! 
//! ```sh
//! $ pulumi import docker:index/plugin:Plugin sample-volume-plugin "$(docker plugin inspect -f {{.ID}} tiborvass/sample-volume-plugin:latest)"
//! ```
//! 

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct PluginArgs {
    /// Docker Plugin alias
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub alias: pulumi_wasm_rust::Output<Option<String>>,
    /// HTTP client timeout to enable the plugin
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub enable_timeout: pulumi_wasm_rust::Output<Option<i32>>,
    /// If `true` the plugin is enabled. Defaults to `true`
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
    /// The environment variables in the form of `KEY=VALUE`, e.g. `DEBUG=0`
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub envs: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    /// If true, then the plugin is destroyed forcibly
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub force_destroy: pulumi_wasm_rust::Output<Option<bool>>,
    /// If true, then the plugin is disabled forcibly
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub force_disable: pulumi_wasm_rust::Output<Option<bool>>,
    /// If true, grant all permissions necessary to run the plugin
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub grant_all_permissions: pulumi_wasm_rust::Output<Option<bool>>,
    /// Grant specific permissions only
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub grant_permissions: pulumi_wasm_rust::Output<Option<Vec<crate::types::PluginGrantPermission>>>,
    /// The name of the permission
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub name: pulumi_wasm_rust::Output<Option<String>>,
}

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
    pub grant_permissions: pulumi_wasm_rust::Output<Option<Vec<crate::types::PluginGrantPermission>>>,
    /// The name of the permission
    pub name: pulumi_wasm_rust::Output<String>,
    /// Docker Plugin Reference
    pub plugin_reference: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: PluginArgs) -> PluginResult {

    let result = crate::bindings::pulumi::docker::plugin::invoke(name, &crate::bindings::pulumi::docker::plugin::Args {
        alias: &args.alias.get_inner(),
        enable_timeout: &args.enable_timeout.get_inner(),
        enabled: &args.enabled.get_inner(),
        envs: &args.envs.get_inner(),
        force_destroy: &args.force_destroy.get_inner(),
        force_disable: &args.force_disable.get_inner(),
        grant_all_permissions: &args.grant_all_permissions.get_inner(),
        grant_permissions: &args.grant_permissions.get_inner(),
        name: &args.name.get_inner(),
    });

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
