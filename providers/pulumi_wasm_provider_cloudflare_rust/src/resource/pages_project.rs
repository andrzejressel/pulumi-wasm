//! Provides a resource which manages Cloudflare Pages projects.
//! 
//! > If you are using a `source` block configuration, you must first have a
//!    connected GitHub or GitLab account connected to Cloudflare. See the
//!    [Getting Started with Pages] documentation on how to link your accounts.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as cloudflare from "@pulumi/cloudflare";
//! 
//! // Direct upload Pages project
//! const basicProject = new cloudflare.PagesProject("basicProject", {
//!     accountId: "f037e56e89293a057740de681ac9abbe",
//!     name: "this-is-my-project-01",
//!     productionBranch: "main",
//! });
//! // Pages project with managing build config
//! const buildConfig = new cloudflare.PagesProject("buildConfig", {
//!     accountId: "f037e56e89293a057740de681ac9abbe",
//!     buildConfig: {
//!         buildCommand: "npm run build",
//!         destinationDir: "build",
//!         rootDir: "",
//!         webAnalyticsTag: "cee1c73f6e4743d0b5e6bb1a0bcaabcc",
//!         webAnalyticsToken: "021e1057c18547eca7b79f2516f06o7x",
//!     },
//!     name: "this-is-my-project-01",
//!     productionBranch: "main",
//! });
//! // Pages project managing project source
//! const sourceConfig = new cloudflare.PagesProject("sourceConfig", {
//!     accountId: "f037e56e89293a057740de681ac9abbe",
//!     name: "this-is-my-project-01",
//!     productionBranch: "main",
//!     source: {
//!         config: {
//!             deploymentsEnabled: true,
//!             owner: "cloudflare",
//!             prCommentsEnabled: true,
//!             previewBranchExcludes: [
//!                 "main",
//!                 "prod",
//!             ],
//!             previewBranchIncludes: [
//!                 "dev",
//!                 "preview",
//!             ],
//!             previewDeploymentSetting: "custom",
//!             productionBranch: "main",
//!             productionDeploymentEnabled: true,
//!             repoName: "ninjakittens",
//!         },
//!         type: "github",
//!     },
//! });
//! // Pages project managing all configs
//! const deploymentConfigs = new cloudflare.PagesProject("deploymentConfigs", {
//!     accountId: "f037e56e89293a057740de681ac9abbe",
//!     buildConfig: {
//!         buildCommand: "npm run build",
//!         destinationDir: "build",
//!         rootDir: "",
//!         webAnalyticsTag: "cee1c73f6e4743d0b5e6bb1a0bcaabcc",
//!         webAnalyticsToken: "021e1057c18547eca7b79f2516f06o7x",
//!     },
//!     deploymentConfigs: {
//!         preview: {
//!             compatibilityDate: "2022-08-15",
//!             compatibilityFlags: ["nodejs_compat"],
//!             d1Databases: {
//!                 D1BINDING: "445e2955-951a-4358-a35b-a4d0c813f63",
//!             },
//!             durableObjectNamespaces: {
//!                 DOBINDING: "5eb63bbbe01eeed093cb22bb8f5acdc3",
//!             },
//!             environmentVariables: {
//!                 ENVIRONMENT: "preview",
//!             },
//!             kvNamespaces: {
//!                 KVBINDING: "5eb63bbbe01eeed093cb22bb8f5acdc3",
//!             },
//!             r2Buckets: {
//!                 R2BINDING: "some-bucket",
//!             },
//!             secrets: {
//!                 TURNSTILESECRET: "1x0000000000000000000000000000000AA",
//!             },
//!         },
//!         production: {
//!             compatibilityDate: "2022-08-16",
//!             compatibilityFlags: [
//!                 "nodejs_compat",
//!                 "streams_enable_constructors",
//!             ],
//!             d1Databases: {
//!                 D1BINDING1: "445e2955-951a-4358-a35b-a4d0c813f63",
//!                 D1BINDING2: "a399414b-c697-409a-a688-377db6433cd9",
//!             },
//!             durableObjectNamespaces: {
//!                 DOBINDING1: "5eb63bbbe01eeed093cb22bb8f5acdc3",
//!                 DOBINDING2: "3cdca5f8bb22bc390deee10ebbb36be5",
//!             },
//!             environmentVariables: {
//!                 ENVIRONMENT: "production",
//!                 OTHERVALUE: "other value",
//!             },
//!             kvNamespaces: {
//!                 KVBINDING1: "5eb63bbbe01eeed093cb22bb8f5acdc3",
//!                 KVBINDING2: "3cdca5f8bb22bc390deee10ebbb36be5",
//!             },
//!             r2Buckets: {
//!                 R2BINDING1: "some-bucket",
//!                 R2BINDING2: "other-bucket",
//!             },
//!             secrets: {
//!                 TURNSTILEINVISSECRET: "2x0000000000000000000000000000000AA",
//!                 TURNSTILESECRET: "1x0000000000000000000000000000000AA",
//!             },
//!         },
//!     },
//!     name: "this-is-my-project-01",
//!     productionBranch: "main",
//!     source: {
//!         config: {
//!             deploymentsEnabled: true,
//!             owner: "cloudflare",
//!             prCommentsEnabled: true,
//!             previewBranchExcludes: [
//!                 "main",
//!                 "prod",
//!             ],
//!             previewBranchIncludes: [
//!                 "dev",
//!                 "preview",
//!             ],
//!             previewDeploymentSetting: "custom",
//!             productionBranch: "main",
//!             productionDeploymentEnabled: true,
//!             repoName: "ninjakittens",
//!         },
//!         type: "github",
//!     },
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//! 
//! # Direct upload Pages project
//! basic_project = cloudflare.PagesProject("basicProject",
//!     account_id="f037e56e89293a057740de681ac9abbe",
//!     name="this-is-my-project-01",
//!     production_branch="main")
//! # Pages project with managing build config
//! build_config = cloudflare.PagesProject("buildConfig",
//!     account_id="f037e56e89293a057740de681ac9abbe",
//!     build_config=cloudflare.PagesProjectBuildConfigArgs(
//!         build_command="npm run build",
//!         destination_dir="build",
//!         root_dir="",
//!         web_analytics_tag="cee1c73f6e4743d0b5e6bb1a0bcaabcc",
//!         web_analytics_token="021e1057c18547eca7b79f2516f06o7x",
//!     ),
//!     name="this-is-my-project-01",
//!     production_branch="main")
//! # Pages project managing project source
//! source_config = cloudflare.PagesProject("sourceConfig",
//!     account_id="f037e56e89293a057740de681ac9abbe",
//!     name="this-is-my-project-01",
//!     production_branch="main",
//!     source=cloudflare.PagesProjectSourceArgs(
//!         config=cloudflare.PagesProjectSourceConfigArgs(
//!             deployments_enabled=True,
//!             owner="cloudflare",
//!             pr_comments_enabled=True,
//!             preview_branch_excludes=[
//!                 "main",
//!                 "prod",
//!             ],
//!             preview_branch_includes=[
//!                 "dev",
//!                 "preview",
//!             ],
//!             preview_deployment_setting="custom",
//!             production_branch="main",
//!             production_deployment_enabled=True,
//!             repo_name="ninjakittens",
//!         ),
//!         type="github",
//!     ))
//! # Pages project managing all configs
//! deployment_configs = cloudflare.PagesProject("deploymentConfigs",
//!     account_id="f037e56e89293a057740de681ac9abbe",
//!     build_config=cloudflare.PagesProjectBuildConfigArgs(
//!         build_command="npm run build",
//!         destination_dir="build",
//!         root_dir="",
//!         web_analytics_tag="cee1c73f6e4743d0b5e6bb1a0bcaabcc",
//!         web_analytics_token="021e1057c18547eca7b79f2516f06o7x",
//!     ),
//!     deployment_configs=cloudflare.PagesProjectDeploymentConfigsArgs(
//!         preview=cloudflare.PagesProjectDeploymentConfigsPreviewArgs(
//!             compatibility_date="2022-08-15",
//!             compatibility_flags=["nodejs_compat"],
//!             d1_databases={
//!                 "D1BINDING": "445e2955-951a-4358-a35b-a4d0c813f63",
//!             },
//!             durable_object_namespaces={
//!                 "DOBINDING": "5eb63bbbe01eeed093cb22bb8f5acdc3",
//!             },
//!             environment_variables={
//!                 "ENVIRONMENT": "preview",
//!             },
//!             kv_namespaces={
//!                 "KVBINDING": "5eb63bbbe01eeed093cb22bb8f5acdc3",
//!             },
//!             r2_buckets={
//!                 "R2BINDING": "some-bucket",
//!             },
//!             secrets={
//!                 "TURNSTILESECRET": "1x0000000000000000000000000000000AA",
//!             },
//!         ),
//!         production=cloudflare.PagesProjectDeploymentConfigsProductionArgs(
//!             compatibility_date="2022-08-16",
//!             compatibility_flags=[
//!                 "nodejs_compat",
//!                 "streams_enable_constructors",
//!             ],
//!             d1_databases={
//!                 "D1BINDING1": "445e2955-951a-4358-a35b-a4d0c813f63",
//!                 "D1BINDING2": "a399414b-c697-409a-a688-377db6433cd9",
//!             },
//!             durable_object_namespaces={
//!                 "DOBINDING1": "5eb63bbbe01eeed093cb22bb8f5acdc3",
//!                 "DOBINDING2": "3cdca5f8bb22bc390deee10ebbb36be5",
//!             },
//!             environment_variables={
//!                 "ENVIRONMENT": "production",
//!                 "OTHERVALUE": "other value",
//!             },
//!             kv_namespaces={
//!                 "KVBINDING1": "5eb63bbbe01eeed093cb22bb8f5acdc3",
//!                 "KVBINDING2": "3cdca5f8bb22bc390deee10ebbb36be5",
//!             },
//!             r2_buckets={
//!                 "R2BINDING1": "some-bucket",
//!                 "R2BINDING2": "other-bucket",
//!             },
//!             secrets={
//!                 "TURNSTILEINVISSECRET": "2x0000000000000000000000000000000AA",
//!                 "TURNSTILESECRET": "1x0000000000000000000000000000000AA",
//!             },
//!         ),
//!     ),
//!     name="this-is-my-project-01",
//!     production_branch="main",
//!     source=cloudflare.PagesProjectSourceArgs(
//!         config=cloudflare.PagesProjectSourceConfigArgs(
//!             deployments_enabled=True,
//!             owner="cloudflare",
//!             pr_comments_enabled=True,
//!             preview_branch_excludes=[
//!                 "main",
//!                 "prod",
//!             ],
//!             preview_branch_includes=[
//!                 "dev",
//!                 "preview",
//!             ],
//!             preview_deployment_setting="custom",
//!             production_branch="main",
//!             production_deployment_enabled=True,
//!             repo_name="ninjakittens",
//!         ),
//!         type="github",
//!     ))
//! ```
//! ### C#
//! ```csharp
//! using System.Collections.Generic;
//! using System.Linq;
//! using Pulumi;
//! using Cloudflare = Pulumi.Cloudflare;
//! 
//! return await Deployment.RunAsync(() => 
//! {
//!     // Direct upload Pages project
//!     var basicProject = new Cloudflare.PagesProject("basicProject", new()
//!     {
//!         AccountId = "f037e56e89293a057740de681ac9abbe",
//!         Name = "this-is-my-project-01",
//!         ProductionBranch = "main",
//!     });
//! 
//!     // Pages project with managing build config
//!     var buildConfig = new Cloudflare.PagesProject("buildConfig", new()
//!     {
//!         AccountId = "f037e56e89293a057740de681ac9abbe",
//!         BuildConfig = new Cloudflare.Inputs.PagesProjectBuildConfigArgs
//!         {
//!             BuildCommand = "npm run build",
//!             DestinationDir = "build",
//!             RootDir = "",
//!             WebAnalyticsTag = "cee1c73f6e4743d0b5e6bb1a0bcaabcc",
//!             WebAnalyticsToken = "021e1057c18547eca7b79f2516f06o7x",
//!         },
//!         Name = "this-is-my-project-01",
//!         ProductionBranch = "main",
//!     });
//! 
//!     // Pages project managing project source
//!     var sourceConfig = new Cloudflare.PagesProject("sourceConfig", new()
//!     {
//!         AccountId = "f037e56e89293a057740de681ac9abbe",
//!         Name = "this-is-my-project-01",
//!         ProductionBranch = "main",
//!         Source = new Cloudflare.Inputs.PagesProjectSourceArgs
//!         {
//!             Config = new Cloudflare.Inputs.PagesProjectSourceConfigArgs
//!             {
//!                 DeploymentsEnabled = true,
//!                 Owner = "cloudflare",
//!                 PrCommentsEnabled = true,
//!                 PreviewBranchExcludes = new[]
//!                 {
//!                     "main",
//!                     "prod",
//!                 },
//!                 PreviewBranchIncludes = new[]
//!                 {
//!                     "dev",
//!                     "preview",
//!                 },
//!                 PreviewDeploymentSetting = "custom",
//!                 ProductionBranch = "main",
//!                 ProductionDeploymentEnabled = true,
//!                 RepoName = "ninjakittens",
//!             },
//!             Type = "github",
//!         },
//!     });
//! 
//!     // Pages project managing all configs
//!     var deploymentConfigs = new Cloudflare.PagesProject("deploymentConfigs", new()
//!     {
//!         AccountId = "f037e56e89293a057740de681ac9abbe",
//!         BuildConfig = new Cloudflare.Inputs.PagesProjectBuildConfigArgs
//!         {
//!             BuildCommand = "npm run build",
//!             DestinationDir = "build",
//!             RootDir = "",
//!             WebAnalyticsTag = "cee1c73f6e4743d0b5e6bb1a0bcaabcc",
//!             WebAnalyticsToken = "021e1057c18547eca7b79f2516f06o7x",
//!         },
//!         DeploymentConfigs = new Cloudflare.Inputs.PagesProjectDeploymentConfigsArgs
//!         {
//!             Preview = new Cloudflare.Inputs.PagesProjectDeploymentConfigsPreviewArgs
//!             {
//!                 CompatibilityDate = "2022-08-15",
//!                 CompatibilityFlags = new[]
//!                 {
//!                     "nodejs_compat",
//!                 },
//!                 D1Databases = 
//!                 {
//!                     { "D1BINDING", "445e2955-951a-4358-a35b-a4d0c813f63" },
//!                 },
//!                 DurableObjectNamespaces = 
//!                 {
//!                     { "DOBINDING", "5eb63bbbe01eeed093cb22bb8f5acdc3" },
//!                 },
//!                 EnvironmentVariables = 
//!                 {
//!                     { "ENVIRONMENT", "preview" },
//!                 },
//!                 KvNamespaces = 
//!                 {
//!                     { "KVBINDING", "5eb63bbbe01eeed093cb22bb8f5acdc3" },
//!                 },
//!                 R2Buckets = 
//!                 {
//!                     { "R2BINDING", "some-bucket" },
//!                 },
//!                 Secrets = 
//!                 {
//!                     { "TURNSTILESECRET", "1x0000000000000000000000000000000AA" },
//!                 },
//!             },
//!             Production = new Cloudflare.Inputs.PagesProjectDeploymentConfigsProductionArgs
//!             {
//!                 CompatibilityDate = "2022-08-16",
//!                 CompatibilityFlags = new[]
//!                 {
//!                     "nodejs_compat",
//!                     "streams_enable_constructors",
//!                 },
//!                 D1Databases = 
//!                 {
//!                     { "D1BINDING1", "445e2955-951a-4358-a35b-a4d0c813f63" },
//!                     { "D1BINDING2", "a399414b-c697-409a-a688-377db6433cd9" },
//!                 },
//!                 DurableObjectNamespaces = 
//!                 {
//!                     { "DOBINDING1", "5eb63bbbe01eeed093cb22bb8f5acdc3" },
//!                     { "DOBINDING2", "3cdca5f8bb22bc390deee10ebbb36be5" },
//!                 },
//!                 EnvironmentVariables = 
//!                 {
//!                     { "ENVIRONMENT", "production" },
//!                     { "OTHERVALUE", "other value" },
//!                 },
//!                 KvNamespaces = 
//!                 {
//!                     { "KVBINDING1", "5eb63bbbe01eeed093cb22bb8f5acdc3" },
//!                     { "KVBINDING2", "3cdca5f8bb22bc390deee10ebbb36be5" },
//!                 },
//!                 R2Buckets = 
//!                 {
//!                     { "R2BINDING1", "some-bucket" },
//!                     { "R2BINDING2", "other-bucket" },
//!                 },
//!                 Secrets = 
//!                 {
//!                     { "TURNSTILEINVISSECRET", "2x0000000000000000000000000000000AA" },
//!                     { "TURNSTILESECRET", "1x0000000000000000000000000000000AA" },
//!                 },
//!             },
//!         },
//!         Name = "this-is-my-project-01",
//!         ProductionBranch = "main",
//!         Source = new Cloudflare.Inputs.PagesProjectSourceArgs
//!         {
//!             Config = new Cloudflare.Inputs.PagesProjectSourceConfigArgs
//!             {
//!                 DeploymentsEnabled = true,
//!                 Owner = "cloudflare",
//!                 PrCommentsEnabled = true,
//!                 PreviewBranchExcludes = new[]
//!                 {
//!                     "main",
//!                     "prod",
//!                 },
//!                 PreviewBranchIncludes = new[]
//!                 {
//!                     "dev",
//!                     "preview",
//!                 },
//!                 PreviewDeploymentSetting = "custom",
//!                 ProductionBranch = "main",
//!                 ProductionDeploymentEnabled = true,
//!                 RepoName = "ninjakittens",
//!             },
//!             Type = "github",
//!         },
//!     });
//! 
//! });
//! ```
//! ### Go
//! ```go
//! package main
//! 
//! import (
//! 	"github.com/pulumi/pulumi-cloudflare/sdk/v5/go/cloudflare"
//! 	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
//! )
//! 
//! func main() {
//! 	pulumi.Run(func(ctx *pulumi.Context) error {
//! 		// Direct upload Pages project
//! 		_, err := cloudflare.NewPagesProject(ctx, "basicProject", &cloudflare.PagesProjectArgs{
//! 			AccountId:        pulumi.String("f037e56e89293a057740de681ac9abbe"),
//! 			Name:             pulumi.String("this-is-my-project-01"),
//! 			ProductionBranch: pulumi.String("main"),
//! 		})
//! 		if err != nil {
//! 			return err
//! 		}
//! 		// Pages project with managing build config
//! 		_, err = cloudflare.NewPagesProject(ctx, "buildConfig", &cloudflare.PagesProjectArgs{
//! 			AccountId: pulumi.String("f037e56e89293a057740de681ac9abbe"),
//! 			BuildConfig: &cloudflare.PagesProjectBuildConfigArgs{
//! 				BuildCommand:      pulumi.String("npm run build"),
//! 				DestinationDir:    pulumi.String("build"),
//! 				RootDir:           pulumi.String(""),
//! 				WebAnalyticsTag:   pulumi.String("cee1c73f6e4743d0b5e6bb1a0bcaabcc"),
//! 				WebAnalyticsToken: pulumi.String("021e1057c18547eca7b79f2516f06o7x"),
//! 			},
//! 			Name:             pulumi.String("this-is-my-project-01"),
//! 			ProductionBranch: pulumi.String("main"),
//! 		})
//! 		if err != nil {
//! 			return err
//! 		}
//! 		// Pages project managing project source
//! 		_, err = cloudflare.NewPagesProject(ctx, "sourceConfig", &cloudflare.PagesProjectArgs{
//! 			AccountId:        pulumi.String("f037e56e89293a057740de681ac9abbe"),
//! 			Name:             pulumi.String("this-is-my-project-01"),
//! 			ProductionBranch: pulumi.String("main"),
//! 			Source: &cloudflare.PagesProjectSourceArgs{
//! 				Config: &cloudflare.PagesProjectSourceConfigArgs{
//! 					DeploymentsEnabled: pulumi.Bool(true),
//! 					Owner:              pulumi.String("cloudflare"),
//! 					PrCommentsEnabled:  pulumi.Bool(true),
//! 					PreviewBranchExcludes: pulumi.StringArray{
//! 						pulumi.String("main"),
//! 						pulumi.String("prod"),
//! 					},
//! 					PreviewBranchIncludes: pulumi.StringArray{
//! 						pulumi.String("dev"),
//! 						pulumi.String("preview"),
//! 					},
//! 					PreviewDeploymentSetting:    pulumi.String("custom"),
//! 					ProductionBranch:            pulumi.String("main"),
//! 					ProductionDeploymentEnabled: pulumi.Bool(true),
//! 					RepoName:                    pulumi.String("ninjakittens"),
//! 				},
//! 				Type: pulumi.String("github"),
//! 			},
//! 		})
//! 		if err != nil {
//! 			return err
//! 		}
//! 		// Pages project managing all configs
//! 		_, err = cloudflare.NewPagesProject(ctx, "deploymentConfigs", &cloudflare.PagesProjectArgs{
//! 			AccountId: pulumi.String("f037e56e89293a057740de681ac9abbe"),
//! 			BuildConfig: &cloudflare.PagesProjectBuildConfigArgs{
//! 				BuildCommand:      pulumi.String("npm run build"),
//! 				DestinationDir:    pulumi.String("build"),
//! 				RootDir:           pulumi.String(""),
//! 				WebAnalyticsTag:   pulumi.String("cee1c73f6e4743d0b5e6bb1a0bcaabcc"),
//! 				WebAnalyticsToken: pulumi.String("021e1057c18547eca7b79f2516f06o7x"),
//! 			},
//! 			DeploymentConfigs: &cloudflare.PagesProjectDeploymentConfigsArgs{
//! 				Preview: &cloudflare.PagesProjectDeploymentConfigsPreviewArgs{
//! 					CompatibilityDate: pulumi.String("2022-08-15"),
//! 					CompatibilityFlags: pulumi.StringArray{
//! 						pulumi.String("nodejs_compat"),
//! 					},
//! 					D1Databases: pulumi.Map{
//! 						"D1BINDING": pulumi.Any("445e2955-951a-4358-a35b-a4d0c813f63"),
//! 					},
//! 					DurableObjectNamespaces: pulumi.Map{
//! 						"DOBINDING": pulumi.Any("5eb63bbbe01eeed093cb22bb8f5acdc3"),
//! 					},
//! 					EnvironmentVariables: pulumi.Map{
//! 						"ENVIRONMENT": pulumi.Any("preview"),
//! 					},
//! 					KvNamespaces: pulumi.Map{
//! 						"KVBINDING": pulumi.Any("5eb63bbbe01eeed093cb22bb8f5acdc3"),
//! 					},
//! 					R2Buckets: pulumi.Map{
//! 						"R2BINDING": pulumi.Any("some-bucket"),
//! 					},
//! 					Secrets: pulumi.Map{
//! 						"TURNSTILESECRET": pulumi.Any("1x0000000000000000000000000000000AA"),
//! 					},
//! 				},
//! 				Production: &cloudflare.PagesProjectDeploymentConfigsProductionArgs{
//! 					CompatibilityDate: pulumi.String("2022-08-16"),
//! 					CompatibilityFlags: pulumi.StringArray{
//! 						pulumi.String("nodejs_compat"),
//! 						pulumi.String("streams_enable_constructors"),
//! 					},
//! 					D1Databases: pulumi.Map{
//! 						"D1BINDING1": pulumi.Any("445e2955-951a-4358-a35b-a4d0c813f63"),
//! 						"D1BINDING2": pulumi.Any("a399414b-c697-409a-a688-377db6433cd9"),
//! 					},
//! 					DurableObjectNamespaces: pulumi.Map{
//! 						"DOBINDING1": pulumi.Any("5eb63bbbe01eeed093cb22bb8f5acdc3"),
//! 						"DOBINDING2": pulumi.Any("3cdca5f8bb22bc390deee10ebbb36be5"),
//! 					},
//! 					EnvironmentVariables: pulumi.Map{
//! 						"ENVIRONMENT": pulumi.Any("production"),
//! 						"OTHERVALUE":  pulumi.Any("other value"),
//! 					},
//! 					KvNamespaces: pulumi.Map{
//! 						"KVBINDING1": pulumi.Any("5eb63bbbe01eeed093cb22bb8f5acdc3"),
//! 						"KVBINDING2": pulumi.Any("3cdca5f8bb22bc390deee10ebbb36be5"),
//! 					},
//! 					R2Buckets: pulumi.Map{
//! 						"R2BINDING1": pulumi.Any("some-bucket"),
//! 						"R2BINDING2": pulumi.Any("other-bucket"),
//! 					},
//! 					Secrets: pulumi.Map{
//! 						"TURNSTILEINVISSECRET": pulumi.Any("2x0000000000000000000000000000000AA"),
//! 						"TURNSTILESECRET":      pulumi.Any("1x0000000000000000000000000000000AA"),
//! 					},
//! 				},
//! 			},
//! 			Name:             pulumi.String("this-is-my-project-01"),
//! 			ProductionBranch: pulumi.String("main"),
//! 			Source: &cloudflare.PagesProjectSourceArgs{
//! 				Config: &cloudflare.PagesProjectSourceConfigArgs{
//! 					DeploymentsEnabled: pulumi.Bool(true),
//! 					Owner:              pulumi.String("cloudflare"),
//! 					PrCommentsEnabled:  pulumi.Bool(true),
//! 					PreviewBranchExcludes: pulumi.StringArray{
//! 						pulumi.String("main"),
//! 						pulumi.String("prod"),
//! 					},
//! 					PreviewBranchIncludes: pulumi.StringArray{
//! 						pulumi.String("dev"),
//! 						pulumi.String("preview"),
//! 					},
//! 					PreviewDeploymentSetting:    pulumi.String("custom"),
//! 					ProductionBranch:            pulumi.String("main"),
//! 					ProductionDeploymentEnabled: pulumi.Bool(true),
//! 					RepoName:                    pulumi.String("ninjakittens"),
//! 				},
//! 				Type: pulumi.String("github"),
//! 			},
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
//! import com.pulumi.cloudflare.PagesProject;
//! import com.pulumi.cloudflare.PagesProjectArgs;
//! import com.pulumi.cloudflare.inputs.PagesProjectBuildConfigArgs;
//! import com.pulumi.cloudflare.inputs.PagesProjectSourceArgs;
//! import com.pulumi.cloudflare.inputs.PagesProjectSourceConfigArgs;
//! import com.pulumi.cloudflare.inputs.PagesProjectDeploymentConfigsArgs;
//! import com.pulumi.cloudflare.inputs.PagesProjectDeploymentConfigsPreviewArgs;
//! import com.pulumi.cloudflare.inputs.PagesProjectDeploymentConfigsProductionArgs;
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
//!         // Direct upload Pages project
//!         var basicProject = new PagesProject("basicProject", PagesProjectArgs.builder()        
//!             .accountId("f037e56e89293a057740de681ac9abbe")
//!             .name("this-is-my-project-01")
//!             .productionBranch("main")
//!             .build());
//! 
//!         // Pages project with managing build config
//!         var buildConfig = new PagesProject("buildConfig", PagesProjectArgs.builder()        
//!             .accountId("f037e56e89293a057740de681ac9abbe")
//!             .buildConfig(PagesProjectBuildConfigArgs.builder()
//!                 .buildCommand("npm run build")
//!                 .destinationDir("build")
//!                 .rootDir("")
//!                 .webAnalyticsTag("cee1c73f6e4743d0b5e6bb1a0bcaabcc")
//!                 .webAnalyticsToken("021e1057c18547eca7b79f2516f06o7x")
//!                 .build())
//!             .name("this-is-my-project-01")
//!             .productionBranch("main")
//!             .build());
//! 
//!         // Pages project managing project source
//!         var sourceConfig = new PagesProject("sourceConfig", PagesProjectArgs.builder()        
//!             .accountId("f037e56e89293a057740de681ac9abbe")
//!             .name("this-is-my-project-01")
//!             .productionBranch("main")
//!             .source(PagesProjectSourceArgs.builder()
//!                 .config(PagesProjectSourceConfigArgs.builder()
//!                     .deploymentsEnabled(true)
//!                     .owner("cloudflare")
//!                     .prCommentsEnabled(true)
//!                     .previewBranchExcludes(                    
//!                         "main",
//!                         "prod")
//!                     .previewBranchIncludes(                    
//!                         "dev",
//!                         "preview")
//!                     .previewDeploymentSetting("custom")
//!                     .productionBranch("main")
//!                     .productionDeploymentEnabled(true)
//!                     .repoName("ninjakittens")
//!                     .build())
//!                 .type("github")
//!                 .build())
//!             .build());
//! 
//!         // Pages project managing all configs
//!         var deploymentConfigs = new PagesProject("deploymentConfigs", PagesProjectArgs.builder()        
//!             .accountId("f037e56e89293a057740de681ac9abbe")
//!             .buildConfig(PagesProjectBuildConfigArgs.builder()
//!                 .buildCommand("npm run build")
//!                 .destinationDir("build")
//!                 .rootDir("")
//!                 .webAnalyticsTag("cee1c73f6e4743d0b5e6bb1a0bcaabcc")
//!                 .webAnalyticsToken("021e1057c18547eca7b79f2516f06o7x")
//!                 .build())
//!             .deploymentConfigs(PagesProjectDeploymentConfigsArgs.builder()
//!                 .preview(PagesProjectDeploymentConfigsPreviewArgs.builder()
//!                     .compatibilityDate("2022-08-15")
//!                     .compatibilityFlags("nodejs_compat")
//!                     .d1Databases(Map.of("D1BINDING", "445e2955-951a-4358-a35b-a4d0c813f63"))
//!                     .durableObjectNamespaces(Map.of("DOBINDING", "5eb63bbbe01eeed093cb22bb8f5acdc3"))
//!                     .environmentVariables(Map.of("ENVIRONMENT", "preview"))
//!                     .kvNamespaces(Map.of("KVBINDING", "5eb63bbbe01eeed093cb22bb8f5acdc3"))
//!                     .r2Buckets(Map.of("R2BINDING", "some-bucket"))
//!                     .secrets(Map.of("TURNSTILESECRET", "1x0000000000000000000000000000000AA"))
//!                     .build())
//!                 .production(PagesProjectDeploymentConfigsProductionArgs.builder()
//!                     .compatibilityDate("2022-08-16")
//!                     .compatibilityFlags(                    
//!                         "nodejs_compat",
//!                         "streams_enable_constructors")
//!                     .d1Databases(Map.ofEntries(
//!                         Map.entry("D1BINDING1", "445e2955-951a-4358-a35b-a4d0c813f63"),
//!                         Map.entry("D1BINDING2", "a399414b-c697-409a-a688-377db6433cd9")
//!                     ))
//!                     .durableObjectNamespaces(Map.ofEntries(
//!                         Map.entry("DOBINDING1", "5eb63bbbe01eeed093cb22bb8f5acdc3"),
//!                         Map.entry("DOBINDING2", "3cdca5f8bb22bc390deee10ebbb36be5")
//!                     ))
//!                     .environmentVariables(Map.ofEntries(
//!                         Map.entry("ENVIRONMENT", "production"),
//!                         Map.entry("OTHERVALUE", "other value")
//!                     ))
//!                     .kvNamespaces(Map.ofEntries(
//!                         Map.entry("KVBINDING1", "5eb63bbbe01eeed093cb22bb8f5acdc3"),
//!                         Map.entry("KVBINDING2", "3cdca5f8bb22bc390deee10ebbb36be5")
//!                     ))
//!                     .r2Buckets(Map.ofEntries(
//!                         Map.entry("R2BINDING1", "some-bucket"),
//!                         Map.entry("R2BINDING2", "other-bucket")
//!                     ))
//!                     .secrets(Map.ofEntries(
//!                         Map.entry("TURNSTILEINVISSECRET", "2x0000000000000000000000000000000AA"),
//!                         Map.entry("TURNSTILESECRET", "1x0000000000000000000000000000000AA")
//!                     ))
//!                     .build())
//!                 .build())
//!             .name("this-is-my-project-01")
//!             .productionBranch("main")
//!             .source(PagesProjectSourceArgs.builder()
//!                 .config(PagesProjectSourceConfigArgs.builder()
//!                     .deploymentsEnabled(true)
//!                     .owner("cloudflare")
//!                     .prCommentsEnabled(true)
//!                     .previewBranchExcludes(                    
//!                         "main",
//!                         "prod")
//!                     .previewBranchIncludes(                    
//!                         "dev",
//!                         "preview")
//!                     .previewDeploymentSetting("custom")
//!                     .productionBranch("main")
//!                     .productionDeploymentEnabled(true)
//!                     .repoName("ninjakittens")
//!                     .build())
//!                 .type("github")
//!                 .build())
//!             .build());
//! 
//!     }
//! }
//! ```
//! ### YAML
//! ```yaml
//! resources:
//!   # Direct upload Pages project
//!   basicProject:
//!     type: cloudflare:PagesProject
//!     properties:
//!       accountId: f037e56e89293a057740de681ac9abbe
//!       name: this-is-my-project-01
//!       productionBranch: main
//!   # Pages project with managing build config
//!   buildConfig:
//!     type: cloudflare:PagesProject
//!     properties:
//!       accountId: f037e56e89293a057740de681ac9abbe
//!       buildConfig:
//!         buildCommand: npm run build
//!         destinationDir: build
//!         rootDir:
//!         webAnalyticsTag: cee1c73f6e4743d0b5e6bb1a0bcaabcc
//!         webAnalyticsToken: 021e1057c18547eca7b79f2516f06o7x
//!       name: this-is-my-project-01
//!       productionBranch: main
//!   # Pages project managing project source
//!   sourceConfig:
//!     type: cloudflare:PagesProject
//!     properties:
//!       accountId: f037e56e89293a057740de681ac9abbe
//!       name: this-is-my-project-01
//!       productionBranch: main
//!       source:
//!         config:
//!           deploymentsEnabled: true
//!           owner: cloudflare
//!           prCommentsEnabled: true
//!           previewBranchExcludes:
//!             - main
//!             - prod
//!           previewBranchIncludes:
//!             - dev
//!             - preview
//!           previewDeploymentSetting: custom
//!           productionBranch: main
//!           productionDeploymentEnabled: true
//!           repoName: ninjakittens
//!         type: github
//!   # Pages project managing all configs
//!   deploymentConfigs:
//!     type: cloudflare:PagesProject
//!     properties:
//!       accountId: f037e56e89293a057740de681ac9abbe
//!       buildConfig:
//!         buildCommand: npm run build
//!         destinationDir: build
//!         rootDir:
//!         webAnalyticsTag: cee1c73f6e4743d0b5e6bb1a0bcaabcc
//!         webAnalyticsToken: 021e1057c18547eca7b79f2516f06o7x
//!       deploymentConfigs:
//!         preview:
//!           compatibilityDate: 2022-08-15
//!           compatibilityFlags:
//!             - nodejs_compat
//!           d1Databases:
//!             D1BINDING: 445e2955-951a-4358-a35b-a4d0c813f63
//!           durableObjectNamespaces:
//!             DOBINDING: 5eb63bbbe01eeed093cb22bb8f5acdc3
//!           environmentVariables:
//!             ENVIRONMENT: preview
//!           kvNamespaces:
//!             KVBINDING: 5eb63bbbe01eeed093cb22bb8f5acdc3
//!           r2Buckets:
//!             R2BINDING: some-bucket
//!           secrets:
//!             TURNSTILESECRET: 1x0000000000000000000000000000000AA
//!         production:
//!           compatibilityDate: 2022-08-16
//!           compatibilityFlags:
//!             - nodejs_compat
//!             - streams_enable_constructors
//!           d1Databases:
//!             D1BINDING1: 445e2955-951a-4358-a35b-a4d0c813f63
//!             D1BINDING2: a399414b-c697-409a-a688-377db6433cd9
//!           durableObjectNamespaces:
//!             DOBINDING1: 5eb63bbbe01eeed093cb22bb8f5acdc3
//!             DOBINDING2: 3cdca5f8bb22bc390deee10ebbb36be5
//!           environmentVariables:
//!             ENVIRONMENT: production
//!             OTHERVALUE: other value
//!           kvNamespaces:
//!             KVBINDING1: 5eb63bbbe01eeed093cb22bb8f5acdc3
//!             KVBINDING2: 3cdca5f8bb22bc390deee10ebbb36be5
//!           r2Buckets:
//!             R2BINDING1: some-bucket
//!             R2BINDING2: other-bucket
//!           secrets:
//!             TURNSTILEINVISSECRET: 2x0000000000000000000000000000000AA
//!             TURNSTILESECRET: 1x0000000000000000000000000000000AA
//!       name: this-is-my-project-01
//!       productionBranch: main
//!       source:
//!         config:
//!           deploymentsEnabled: true
//!           owner: cloudflare
//!           prCommentsEnabled: true
//!           previewBranchExcludes:
//!             - main
//!             - prod
//!           previewBranchIncludes:
//!             - dev
//!             - preview
//!           previewDeploymentSetting: custom
//!           productionBranch: main
//!           productionDeploymentEnabled: true
//!           repoName: ninjakittens
//!         type: github
//! ```
//! <!--End PulumiCodeChooser -->
//! 
//! ## Import
//! 
//! !> It is not possible to import a pages project with secret environment variables. If you have a secret environment variable, you must remove it from your project before importing it.
//! 
//! ```sh
//! $ pulumi import cloudflare:index/pagesProject:PagesProject example <account_id>/<project_name>
//! ```
//! 

pub struct PagesProjectArgs {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// Configuration for the project build process. Read more about the build configuration in the [developer documentation](https://developers.cloudflare.com/pages/platform/build-configuration).
    pub build_config: pulumi_wasm_rust::Output<Option<crate::types::PagesProjectBuildConfig>>,
    /// Configuration for deployments in a project.
    pub deployment_configs: pulumi_wasm_rust::Output<Option<crate::types::PagesProjectDeploymentConfigs>>,
    /// The global variable for the binding in your Worker code.
    pub name: pulumi_wasm_rust::Output<String>,
    /// Project production branch name.
    pub production_branch: pulumi_wasm_rust::Output<String>,
    /// Configuration for the project source. Read more about the source configuration in the [developer documentation](https://developers.cloudflare.com/pages/platform/branch-build-controls/).
    pub source: pulumi_wasm_rust::Output<Option<crate::types::PagesProjectSource>>,
}

pub struct PagesProjectResult {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// Configuration for the project build process. Read more about the build configuration in the [developer documentation](https://developers.cloudflare.com/pages/platform/build-configuration).
    pub build_config: pulumi_wasm_rust::Output<Option<crate::types::PagesProjectBuildConfig>>,
    /// When the project was created.
    pub created_on: pulumi_wasm_rust::Output<String>,
    /// Configuration for deployments in a project.
    pub deployment_configs: pulumi_wasm_rust::Output<crate::types::PagesProjectDeploymentConfigs>,
    /// A list of associated custom domains for the project.
    pub domains: pulumi_wasm_rust::Output<Vec<String>>,
    /// The global variable for the binding in your Worker code.
    pub name: pulumi_wasm_rust::Output<String>,
    /// Project production branch name.
    pub production_branch: pulumi_wasm_rust::Output<String>,
    /// Configuration for the project source. Read more about the source configuration in the [developer documentation](https://developers.cloudflare.com/pages/platform/branch-build-controls/).
    pub source: pulumi_wasm_rust::Output<Option<crate::types::PagesProjectSource>>,
    /// The Cloudflare subdomain associated with the project.
    pub subdomain: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: PagesProjectArgs) -> PagesProjectResult {

    let result = crate::bindings::pulumi::cloudflare::pages_project::invoke(name, &crate::bindings::pulumi::cloudflare::pages_project::Args {
        account_id: args.account_id.get_inner(),
        build_config: args.build_config.get_inner(),
        deployment_configs: args.deployment_configs.get_inner(),
        name: args.name.get_inner(),
        production_branch: args.production_branch.get_inner(),
        source: args.source.get_inner(),
    });

    PagesProjectResult {
        account_id: crate::into_domain(result.account_id),
        build_config: crate::into_domain(result.build_config),
        created_on: crate::into_domain(result.created_on),
        deployment_configs: crate::into_domain(result.deployment_configs),
        domains: crate::into_domain(result.domains),
        name: crate::into_domain(result.name),
        production_branch: crate::into_domain(result.production_branch),
        source: crate::into_domain(result.source),
        subdomain: crate::into_domain(result.subdomain),
    }
}
