//! <!-- Bug: Type and Name are switched -->
//! Manages the lifecycle of a Docker plugin.
//!
//! ## Example Usage
//!
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as docker from "@pulumi/docker";
//!
//! const sample_volume_plugin = new docker.Plugin("sample-volume-plugin", {
//!     alias: "sample-volume-plugin",
//!     enableTimeout: 60,
//!     enabled: false,
//!     envs: ["DEBUG=1"],
//!     forceDestroy: true,
//!     forceDisable: true,
//!     grantAllPermissions: true,
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_docker as docker
//!
//! sample_volume_plugin = docker.Plugin("sample-volume-plugin",
//!     alias="sample-volume-plugin",
//!     enable_timeout=60,
//!     enabled=False,
//!     envs=["DEBUG=1"],
//!     force_destroy=True,
//!     force_disable=True,
//!     grant_all_permissions=True)
//! ```
//! ### C#
//! ```csharp
//! using System.Collections.Generic;
//! using System.Linq;
//! using Pulumi;
//! using Docker = Pulumi.Docker;
//!
//! return await Deployment.RunAsync(() =>
//! {
//!     var sample_volume_plugin = new Docker.Plugin("sample-volume-plugin", new()
//!     {
//!         Alias = "sample-volume-plugin",
//!         EnableTimeout = 60,
//!         Enabled = false,
//!         Envs = new[]
//!         {
//!             "DEBUG=1",
//!         },
//!         ForceDestroy = true,
//!         ForceDisable = true,
//!         GrantAllPermissions = true,
//!     });
//!
//! });
//! ```
//! ### Go
//! ```go
//! package main
//!
//! import (
//! 	"github.com/pulumi/pulumi-docker/sdk/v4/go/docker"
//! 	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
//! )
//!
//! func main() {
//! 	pulumi.Run(func(ctx *pulumi.Context) error {
//! 		_, err := docker.NewPlugin(ctx, "sample-volume-plugin", &docker.PluginArgs{
//! 			Alias:         pulumi.String("sample-volume-plugin"),
//! 			EnableTimeout: pulumi.Int(60),
//! 			Enabled:       pulumi.Bool(false),
//! 			Envs: pulumi.StringArray{
//! 				pulumi.String("DEBUG=1"),
//! 			},
//! 			ForceDestroy:        pulumi.Bool(true),
//! 			ForceDisable:        pulumi.Bool(true),
//! 			GrantAllPermissions: pulumi.Bool(true),
//! 		})
//! 		if err != nil {
//! 			return err
//! 		}
//! 		return nil
//! 	})
//! }
//! ```
//! ### Java
//! ```java
//! package generated_program;
//!
//! import com.pulumi.Context;
//! import com.pulumi.Pulumi;
//! import com.pulumi.core.Output;
//! import com.pulumi.docker.Plugin;
//! import com.pulumi.docker.PluginArgs;
//! import java.util.List;
//! import java.util.ArrayList;
//! import java.util.Map;
//! import java.io.File;
//! import java.nio.file.Files;
//! import java.nio.file.Paths;
//!
//! public class App {
//!     public static void main(String[] args) {
//!         Pulumi.run(App::stack);
//!     }
//!
//!     public static void stack(Context ctx) {
//!         var sample_volume_plugin = new Plugin("sample-volume-plugin", PluginArgs.builder()        
//!             .alias("sample-volume-plugin")
//!             .enableTimeout(60)
//!             .enabled(false)
//!             .envs("DEBUG=1")
//!             .forceDestroy(true)
//!             .forceDisable(true)
//!             .grantAllPermissions(true)
//!             .build());
//!
//!     }
//! }
//! ```
//! ### YAML
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

pub struct PluginArgs {
    /// Docker Plugin alias
    pub alias: pulumi_wasm_rust::Output<Option<String>>,
    /// HTTP client timeout to enable the plugin
    pub enable_timeout: pulumi_wasm_rust::Output<Option<i32>>,
    /// If `true` the plugin is enabled. Defaults to `true`
    pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
    /// The environment variables in the form of `KEY=VALUE`, e.g. `DEBUG=0`
    pub envs: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    /// If true, then the plugin is destroyed forcibly
    pub force_destroy: pulumi_wasm_rust::Output<Option<bool>>,
    /// If true, then the plugin is disabled forcibly
    pub force_disable: pulumi_wasm_rust::Output<Option<bool>>,
    /// If true, grant all permissions necessary to run the plugin
    pub grant_all_permissions: pulumi_wasm_rust::Output<Option<bool>>,
    /// Grant specific permissions only
    pub grant_permissions:
        pulumi_wasm_rust::Output<Option<Vec<crate::types::PluginGrantPermission>>>,
    /// The name of the permission
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
    pub grant_permissions:
        pulumi_wasm_rust::Output<Option<Vec<crate::types::PluginGrantPermission>>>,
    /// The name of the permission
    pub name: pulumi_wasm_rust::Output<String>,
    /// Docker Plugin Reference
    pub plugin_reference: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: PluginArgs) -> PluginResult {
    let result = crate::bindings::pulumi::docker::plugin::invoke(
        name,
        &crate::bindings::pulumi::docker::plugin::Args {
            alias: &args.alias.get_inner(),
            enable_timeout: &args.enable_timeout.get_inner(),
            enabled: &args.enabled.get_inner(),
            envs: &args.envs.get_inner(),
            force_destroy: &args.force_destroy.get_inner(),
            force_disable: &args.force_disable.get_inner(),
            grant_all_permissions: &args.grant_all_permissions.get_inner(),
            grant_permissions: &args.grant_permissions.get_inner(),
            name: &args.name.get_inner(),
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
