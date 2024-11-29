//! Provides a Cloudflare DLP Profile resource. Data Loss Prevention profiles
//! are a set of entries that can be matched in HTTP bodies or files.
//! They are referenced in Zero Trust Gateway rules.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as cloudflare from "@pulumi/cloudflare";
//! 
//! // Predefined profile must be imported, cannot be created
//! const creds = new cloudflare.DlpProfile("creds", {
//!     accountId: "f037e56e89293a057740de681ac9abbe",
//!     name: "Credentials and Secrets",
//!     type: "predefined",
//!     allowedMatchCount: 3,
//!     entries: [
//!         {
//!             enabled: true,
//!             name: "Amazon AWS Access Key ID",
//!             id: "d8fcfc9c-773c-405e-8426-21ecbb67ba93",
//!         },
//!         {
//!             enabled: false,
//!             id: "2c0e33e1-71da-40c8-aad3-32e674ad3d96",
//!             name: "Amazon AWS Secret Access Key",
//!         },
//!         {
//!             enabled: true,
//!             id: "4e92c006-3802-4dff-bbe1-8e1513b1c92a",
//!             name: "Microsoft Azure Client Secret",
//!         },
//!         {
//!             enabled: false,
//!             id: "5c713294-2375-4904-abcf-e4a15be4d592",
//!             name: "SSH Private Key",
//!         },
//!         {
//!             enabled: true,
//!             id: "6c6579e4-d832-42d5-905c-8e53340930f2",
//!             name: "Google GCP API Key",
//!         },
//!     ],
//! });
//! // Custom profile
//! const exampleCustom = new cloudflare.DlpProfile("example_custom", {
//!     accountId: "f037e56e89293a057740de681ac9abbe",
//!     name: "Example Custom Profile",
//!     description: "A profile with example entries",
//!     type: "custom",
//!     allowedMatchCount: 0,
//!     entries: [
//!         {
//!             name: "Matches visa credit cards",
//!             enabled: true,
//!             pattern: {
//!                 regex: "4\\d{3}([-\\. ])?\\d{4}([-\\. ])?\\d{4}([-\\. ])?\\d{4}",
//!                 validation: "luhn",
//!             },
//!         },
//!         {
//!             name: "Matches diners club card",
//!             enabled: true,
//!             pattern: {
//!                 regex: "(?:0[0-5]|[68][0-9])[0-9]{11}",
//!                 validation: "luhn",
//!             },
//!         },
//!     ],
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//! 
//! # Predefined profile must be imported, cannot be created
//! creds = cloudflare.DlpProfile("creds",
//!     account_id="f037e56e89293a057740de681ac9abbe",
//!     name="Credentials and Secrets",
//!     type="predefined",
//!     allowed_match_count=3,
//!     entries=[
//!         {
//!             "enabled": True,
//!             "name": "Amazon AWS Access Key ID",
//!             "id": "d8fcfc9c-773c-405e-8426-21ecbb67ba93",
//!         },
//!         {
//!             "enabled": False,
//!             "id": "2c0e33e1-71da-40c8-aad3-32e674ad3d96",
//!             "name": "Amazon AWS Secret Access Key",
//!         },
//!         {
//!             "enabled": True,
//!             "id": "4e92c006-3802-4dff-bbe1-8e1513b1c92a",
//!             "name": "Microsoft Azure Client Secret",
//!         },
//!         {
//!             "enabled": False,
//!             "id": "5c713294-2375-4904-abcf-e4a15be4d592",
//!             "name": "SSH Private Key",
//!         },
//!         {
//!             "enabled": True,
//!             "id": "6c6579e4-d832-42d5-905c-8e53340930f2",
//!             "name": "Google GCP API Key",
//!         },
//!     ])
//! # Custom profile
//! example_custom = cloudflare.DlpProfile("example_custom",
//!     account_id="f037e56e89293a057740de681ac9abbe",
//!     name="Example Custom Profile",
//!     description="A profile with example entries",
//!     type="custom",
//!     allowed_match_count=0,
//!     entries=[
//!         {
//!             "name": "Matches visa credit cards",
//!             "enabled": True,
//!             "pattern": {
//!                 "regex": "4\\d{3}([-\\. ])?\\d{4}([-\\. ])?\\d{4}([-\\. ])?\\d{4}",
//!                 "validation": "luhn",
//!             },
//!         },
//!         {
//!             "name": "Matches diners club card",
//!             "enabled": True,
//!             "pattern": {
//!                 "regex": "(?:0[0-5]|[68][0-9])[0-9]{11}",
//!                 "validation": "luhn",
//!             },
//!         },
//!     ])
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
//!     // Predefined profile must be imported, cannot be created
//!     var creds = new Cloudflare.DlpProfile("creds", new()
//!     {
//!         AccountId = "f037e56e89293a057740de681ac9abbe",
//!         Name = "Credentials and Secrets",
//!         Type = "predefined",
//!         AllowedMatchCount = 3,
//!         Entries = new[]
//!         {
//!             new Cloudflare.Inputs.DlpProfileEntryArgs
//!             {
//!                 Enabled = true,
//!                 Name = "Amazon AWS Access Key ID",
//!                 Id = "d8fcfc9c-773c-405e-8426-21ecbb67ba93",
//!             },
//!             new Cloudflare.Inputs.DlpProfileEntryArgs
//!             {
//!                 Enabled = false,
//!                 Id = "2c0e33e1-71da-40c8-aad3-32e674ad3d96",
//!                 Name = "Amazon AWS Secret Access Key",
//!             },
//!             new Cloudflare.Inputs.DlpProfileEntryArgs
//!             {
//!                 Enabled = true,
//!                 Id = "4e92c006-3802-4dff-bbe1-8e1513b1c92a",
//!                 Name = "Microsoft Azure Client Secret",
//!             },
//!             new Cloudflare.Inputs.DlpProfileEntryArgs
//!             {
//!                 Enabled = false,
//!                 Id = "5c713294-2375-4904-abcf-e4a15be4d592",
//!                 Name = "SSH Private Key",
//!             },
//!             new Cloudflare.Inputs.DlpProfileEntryArgs
//!             {
//!                 Enabled = true,
//!                 Id = "6c6579e4-d832-42d5-905c-8e53340930f2",
//!                 Name = "Google GCP API Key",
//!             },
//!         },
//!     });
//! 
//!     // Custom profile
//!     var exampleCustom = new Cloudflare.DlpProfile("example_custom", new()
//!     {
//!         AccountId = "f037e56e89293a057740de681ac9abbe",
//!         Name = "Example Custom Profile",
//!         Description = "A profile with example entries",
//!         Type = "custom",
//!         AllowedMatchCount = 0,
//!         Entries = new[]
//!         {
//!             new Cloudflare.Inputs.DlpProfileEntryArgs
//!             {
//!                 Name = "Matches visa credit cards",
//!                 Enabled = true,
//!                 Pattern = new Cloudflare.Inputs.DlpProfileEntryPatternArgs
//!                 {
//!                     Regex = "4\\d{3}([-\\. ])?\\d{4}([-\\. ])?\\d{4}([-\\. ])?\\d{4}",
//!                     Validation = "luhn",
//!                 },
//!             },
//!             new Cloudflare.Inputs.DlpProfileEntryArgs
//!             {
//!                 Name = "Matches diners club card",
//!                 Enabled = true,
//!                 Pattern = new Cloudflare.Inputs.DlpProfileEntryPatternArgs
//!                 {
//!                     Regex = "(?:0[0-5]|[68][0-9])[0-9]{11}",
//!                     Validation = "luhn",
//!                 },
//!             },
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
//! 		// Predefined profile must be imported, cannot be created
//! 		_, err := cloudflare.NewDlpProfile(ctx, "creds", &cloudflare.DlpProfileArgs{
//! 			AccountId:         pulumi.String("f037e56e89293a057740de681ac9abbe"),
//! 			Name:              pulumi.String("Credentials and Secrets"),
//! 			Type:              pulumi.String("predefined"),
//! 			AllowedMatchCount: pulumi.Int(3),
//! 			Entries: cloudflare.DlpProfileEntryArray{
//! 				&cloudflare.DlpProfileEntryArgs{
//! 					Enabled: pulumi.Bool(true),
//! 					Name:    pulumi.String("Amazon AWS Access Key ID"),
//! 					Id:      pulumi.String("d8fcfc9c-773c-405e-8426-21ecbb67ba93"),
//! 				},
//! 				&cloudflare.DlpProfileEntryArgs{
//! 					Enabled: pulumi.Bool(false),
//! 					Id:      pulumi.String("2c0e33e1-71da-40c8-aad3-32e674ad3d96"),
//! 					Name:    pulumi.String("Amazon AWS Secret Access Key"),
//! 				},
//! 				&cloudflare.DlpProfileEntryArgs{
//! 					Enabled: pulumi.Bool(true),
//! 					Id:      pulumi.String("4e92c006-3802-4dff-bbe1-8e1513b1c92a"),
//! 					Name:    pulumi.String("Microsoft Azure Client Secret"),
//! 				},
//! 				&cloudflare.DlpProfileEntryArgs{
//! 					Enabled: pulumi.Bool(false),
//! 					Id:      pulumi.String("5c713294-2375-4904-abcf-e4a15be4d592"),
//! 					Name:    pulumi.String("SSH Private Key"),
//! 				},
//! 				&cloudflare.DlpProfileEntryArgs{
//! 					Enabled: pulumi.Bool(true),
//! 					Id:      pulumi.String("6c6579e4-d832-42d5-905c-8e53340930f2"),
//! 					Name:    pulumi.String("Google GCP API Key"),
//! 				},
//! 			},
//! 		})
//! 		if err != nil {
//! 			return err
//! 		}
//! 		// Custom profile
//! 		_, err = cloudflare.NewDlpProfile(ctx, "example_custom", &cloudflare.DlpProfileArgs{
//! 			AccountId:         pulumi.String("f037e56e89293a057740de681ac9abbe"),
//! 			Name:              pulumi.String("Example Custom Profile"),
//! 			Description:       pulumi.String("A profile with example entries"),
//! 			Type:              pulumi.String("custom"),
//! 			AllowedMatchCount: pulumi.Int(0),
//! 			Entries: cloudflare.DlpProfileEntryArray{
//! 				&cloudflare.DlpProfileEntryArgs{
//! 					Name:    pulumi.String("Matches visa credit cards"),
//! 					Enabled: pulumi.Bool(true),
//! 					Pattern: &cloudflare.DlpProfileEntryPatternArgs{
//! 						Regex:      pulumi.String("4\\d{3}([-\\. ])?\\d{4}([-\\. ])?\\d{4}([-\\. ])?\\d{4}"),
//! 						Validation: pulumi.String("luhn"),
//! 					},
//! 				},
//! 				&cloudflare.DlpProfileEntryArgs{
//! 					Name:    pulumi.String("Matches diners club card"),
//! 					Enabled: pulumi.Bool(true),
//! 					Pattern: &cloudflare.DlpProfileEntryPatternArgs{
//! 						Regex:      pulumi.String("(?:0[0-5]|[68][0-9])[0-9]{11}"),
//! 						Validation: pulumi.String("luhn"),
//! 					},
//! 				},
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
//! import com.pulumi.cloudflare.DlpProfile;
//! import com.pulumi.cloudflare.DlpProfileArgs;
//! import com.pulumi.cloudflare.inputs.DlpProfileEntryArgs;
//! import com.pulumi.cloudflare.inputs.DlpProfileEntryPatternArgs;
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
//!         // Predefined profile must be imported, cannot be created
//!         var creds = new DlpProfile("creds", DlpProfileArgs.builder()
//!             .accountId("f037e56e89293a057740de681ac9abbe")
//!             .name("Credentials and Secrets")
//!             .type("predefined")
//!             .allowedMatchCount(3)
//!             .entries(            
//!                 DlpProfileEntryArgs.builder()
//!                     .enabled(true)
//!                     .name("Amazon AWS Access Key ID")
//!                     .id("d8fcfc9c-773c-405e-8426-21ecbb67ba93")
//!                     .build(),
//!                 DlpProfileEntryArgs.builder()
//!                     .enabled(false)
//!                     .id("2c0e33e1-71da-40c8-aad3-32e674ad3d96")
//!                     .name("Amazon AWS Secret Access Key")
//!                     .build(),
//!                 DlpProfileEntryArgs.builder()
//!                     .enabled(true)
//!                     .id("4e92c006-3802-4dff-bbe1-8e1513b1c92a")
//!                     .name("Microsoft Azure Client Secret")
//!                     .build(),
//!                 DlpProfileEntryArgs.builder()
//!                     .enabled(false)
//!                     .id("5c713294-2375-4904-abcf-e4a15be4d592")
//!                     .name("SSH Private Key")
//!                     .build(),
//!                 DlpProfileEntryArgs.builder()
//!                     .enabled(true)
//!                     .id("6c6579e4-d832-42d5-905c-8e53340930f2")
//!                     .name("Google GCP API Key")
//!                     .build())
//!             .build());
//! 
//!         // Custom profile
//!         var exampleCustom = new DlpProfile("exampleCustom", DlpProfileArgs.builder()
//!             .accountId("f037e56e89293a057740de681ac9abbe")
//!             .name("Example Custom Profile")
//!             .description("A profile with example entries")
//!             .type("custom")
//!             .allowedMatchCount(0)
//!             .entries(            
//!                 DlpProfileEntryArgs.builder()
//!                     .name("Matches visa credit cards")
//!                     .enabled(true)
//!                     .pattern(DlpProfileEntryPatternArgs.builder()
//!                         .regex("4\\d{3}([-\\. ])?\\d{4}([-\\. ])?\\d{4}([-\\. ])?\\d{4}")
//!                         .validation("luhn")
//!                         .build())
//!                     .build(),
//!                 DlpProfileEntryArgs.builder()
//!                     .name("Matches diners club card")
//!                     .enabled(true)
//!                     .pattern(DlpProfileEntryPatternArgs.builder()
//!                         .regex("(?:0[0-5]|[68][0-9])[0-9]{11}")
//!                         .validation("luhn")
//!                         .build())
//!                     .build())
//!             .build());
//! 
//!     }
//! }
//! ```
//! ### YAML
//! ```yaml
//! resources:
//!   # Predefined profile must be imported, cannot be created
//!   creds:
//!     type: cloudflare:DlpProfile
//!     properties:
//!       accountId: f037e56e89293a057740de681ac9abbe
//!       name: Credentials and Secrets
//!       type: predefined
//!       allowedMatchCount: 3
//!       entries:
//!         - enabled: true
//!           name: Amazon AWS Access Key ID
//!           id: d8fcfc9c-773c-405e-8426-21ecbb67ba93
//!         - enabled: false
//!           id: 2c0e33e1-71da-40c8-aad3-32e674ad3d96
//!           name: Amazon AWS Secret Access Key
//!         - enabled: true
//!           id: 4e92c006-3802-4dff-bbe1-8e1513b1c92a
//!           name: Microsoft Azure Client Secret
//!         - enabled: false
//!           id: 5c713294-2375-4904-abcf-e4a15be4d592
//!           name: SSH Private Key
//!         - enabled: true
//!           id: 6c6579e4-d832-42d5-905c-8e53340930f2
//!           name: Google GCP API Key
//!   # Custom profile
//!   exampleCustom:
//!     type: cloudflare:DlpProfile
//!     name: example_custom
//!     properties:
//!       accountId: f037e56e89293a057740de681ac9abbe
//!       name: Example Custom Profile
//!       description: A profile with example entries
//!       type: custom
//!       allowedMatchCount: 0
//!       entries:
//!         - name: Matches visa credit cards
//!           enabled: true
//!           pattern:
//!             regex: 4\d{3}([-\. ])?\d{4}([-\. ])?\d{4}([-\. ])?\d{4}
//!             validation: luhn
//!         - name: Matches diners club card
//!           enabled: true
//!           pattern:
//!             regex: (?:0[0-5]|[68][0-9])[0-9]{11}
//!             validation: luhn
//! ```
//! <!--End PulumiCodeChooser -->
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/dlpProfile:DlpProfile example <account_id>/<dlp_profile_id>
//! ```
//! 

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct DlpProfileArgs {
    /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// Related DLP policies will trigger when the match count exceeds the number set.
    #[builder(into)]
    pub allowed_match_count: pulumi_wasm_rust::Output<i32>,
    /// Scan the context of predefined entries to only return matches surrounded by keywords.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub context_awareness: pulumi_wasm_rust::Output<Option<crate::types::DlpProfileContextAwareness>>,
    /// Brief summary of the profile and its intended use.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    /// List of entries to apply to the profile.
    #[builder(into)]
    pub entries: pulumi_wasm_rust::Output<Vec<crate::types::DlpProfileEntry>>,
    /// Name of the profile. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
    /// If true, scan images via OCR to determine if any text present matches filters.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub ocr_enabled: pulumi_wasm_rust::Output<Option<bool>>,
    /// The type of the profile. Available values: `custom`, `predefined`. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub type_: pulumi_wasm_rust::Output<String>,
}

pub struct DlpProfileResult {
    /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// Related DLP policies will trigger when the match count exceeds the number set.
    pub allowed_match_count: pulumi_wasm_rust::Output<i32>,
    /// Scan the context of predefined entries to only return matches surrounded by keywords.
    pub context_awareness: pulumi_wasm_rust::Output<crate::types::DlpProfileContextAwareness>,
    /// Brief summary of the profile and its intended use.
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    /// List of entries to apply to the profile.
    pub entries: pulumi_wasm_rust::Output<Vec<crate::types::DlpProfileEntry>>,
    /// Name of the profile. **Modifying this attribute will force creation of a new resource.**
    pub name: pulumi_wasm_rust::Output<String>,
    /// If true, scan images via OCR to determine if any text present matches filters.
    pub ocr_enabled: pulumi_wasm_rust::Output<Option<bool>>,
    /// The type of the profile. Available values: `custom`, `predefined`. **Modifying this attribute will force creation of a new resource.**
    pub type_: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: DlpProfileArgs) -> DlpProfileResult {

    let result = crate::bindings::pulumi::cloudflare::dlp_profile::invoke(name, &crate::bindings::pulumi::cloudflare::dlp_profile::Args {
        account_id: &args.account_id.get_inner(),
        allowed_match_count: &args.allowed_match_count.get_inner(),
        context_awareness: &args.context_awareness.get_inner(),
        description: &args.description.get_inner(),
        entries: &args.entries.get_inner(),
        name: &args.name.get_inner(),
        ocr_enabled: &args.ocr_enabled.get_inner(),
        type_: &args.type_.get_inner(),
    });

    DlpProfileResult {
        account_id: crate::into_domain(result.account_id),
        allowed_match_count: crate::into_domain(result.allowed_match_count),
        context_awareness: crate::into_domain(result.context_awareness),
        description: crate::into_domain(result.description),
        entries: crate::into_domain(result.entries),
        name: crate::into_domain(result.name),
        ocr_enabled: crate::into_domain(result.ocr_enabled),
        type_: crate::into_domain(result.type_),
    }
}
