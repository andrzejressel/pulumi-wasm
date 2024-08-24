//! The [Turnstile Widget](https://developers.cloudflare.com/turnstile/) resource allows you to manage Cloudflare Turnstile Widgets.
//!
//! ## Example Usage
//!
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as cloudflare from "@pulumi/cloudflare";
//!
//! const example = new cloudflare.TurnstileWidget("example", {
//!     accountId: "f037e56e89293a057740de681ac9abbe",
//!     botFightMode: false,
//!     domains: ["example.com"],
//!     mode: "invisible",
//!     name: "example widget",
//!     region: "world",
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//!
//! example = cloudflare.TurnstileWidget("example",
//!     account_id="f037e56e89293a057740de681ac9abbe",
//!     bot_fight_mode=False,
//!     domains=["example.com"],
//!     mode="invisible",
//!     name="example widget",
//!     region="world")
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
//!     var example = new Cloudflare.TurnstileWidget("example", new()
//!     {
//!         AccountId = "f037e56e89293a057740de681ac9abbe",
//!         BotFightMode = false,
//!         Domains = new[]
//!         {
//!             "example.com",
//!         },
//!         Mode = "invisible",
//!         Name = "example widget",
//!         Region = "world",
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
//! 		_, err := cloudflare.NewTurnstileWidget(ctx, "example", &cloudflare.TurnstileWidgetArgs{
//! 			AccountId:    pulumi.String("f037e56e89293a057740de681ac9abbe"),
//! 			BotFightMode: pulumi.Bool(false),
//! 			Domains: pulumi.StringArray{
//! 				pulumi.String("example.com"),
//! 			},
//! 			Mode:   pulumi.String("invisible"),
//! 			Name:   pulumi.String("example widget"),
//! 			Region: pulumi.String("world"),
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
//! import com.pulumi.cloudflare.TurnstileWidget;
//! import com.pulumi.cloudflare.TurnstileWidgetArgs;
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
//!         var example = new TurnstileWidget("example", TurnstileWidgetArgs.builder()        
//!             .accountId("f037e56e89293a057740de681ac9abbe")
//!             .botFightMode(false)
//!             .domains("example.com")
//!             .mode("invisible")
//!             .name("example widget")
//!             .region("world")
//!             .build());
//!
//!     }
//! }
//! ```
//! ### YAML
//! ```yaml
//! resources:
//!   example:
//!     type: cloudflare:TurnstileWidget
//!     properties:
//!       accountId: f037e56e89293a057740de681ac9abbe
//!       botFightMode: false
//!       domains:
//!         - example.com
//!       mode: invisible
//!       name: example widget
//!       region: world
//! ```
//! <!--End PulumiCodeChooser -->
//!
//! ## Import
//!
//! ```sh
//! $ pulumi import cloudflare:index/turnstileWidget:TurnstileWidget example <account_id>/<site_key>
//! ```
//!

pub struct TurnstileWidgetArgs {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// If bot*fight*mode is set to true, Cloudflare issues computationally expensive challenges in response to malicious bots (Enterprise only).
    pub bot_fight_mode: pulumi_wasm_rust::Output<Option<bool>>,
    /// Domains where the widget is deployed
    pub domains: pulumi_wasm_rust::Output<Vec<String>>,
    /// Widget Mode. Available values: `non-interactive`, `invisible`, `managed`
    pub mode: pulumi_wasm_rust::Output<String>,
    /// Human readable widget name.
    pub name: pulumi_wasm_rust::Output<String>,
    /// Do not show any Cloudflare branding on the widget (Enterprise only).
    pub offlabel: pulumi_wasm_rust::Output<Option<bool>>,
    /// Region where this widget can be used.
    pub region: pulumi_wasm_rust::Output<Option<String>>,
}

pub struct TurnstileWidgetResult {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// If bot*fight*mode is set to true, Cloudflare issues computationally expensive challenges in response to malicious bots (Enterprise only).
    pub bot_fight_mode: pulumi_wasm_rust::Output<bool>,
    /// Domains where the widget is deployed
    pub domains: pulumi_wasm_rust::Output<Vec<String>>,
    /// Widget Mode. Available values: `non-interactive`, `invisible`, `managed`
    pub mode: pulumi_wasm_rust::Output<String>,
    /// Human readable widget name.
    pub name: pulumi_wasm_rust::Output<String>,
    /// Do not show any Cloudflare branding on the widget (Enterprise only).
    pub offlabel: pulumi_wasm_rust::Output<bool>,
    /// Region where this widget can be used.
    pub region: pulumi_wasm_rust::Output<String>,
    /// Secret key for this widget.
    pub secret: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: TurnstileWidgetArgs) -> TurnstileWidgetResult {
    let result = crate::bindings::pulumi::cloudflare::turnstile_widget::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::turnstile_widget::Args {
            account_id: &args.account_id.get_inner(),
            bot_fight_mode: &args.bot_fight_mode.get_inner(),
            domains: &args.domains.get_inner(),
            mode: &args.mode.get_inner(),
            name: &args.name.get_inner(),
            offlabel: &args.offlabel.get_inner(),
            region: &args.region.get_inner(),
        },
    );

    TurnstileWidgetResult {
        account_id: crate::into_domain(result.account_id),
        bot_fight_mode: crate::into_domain(result.bot_fight_mode),
        domains: crate::into_domain(result.domains),
        mode: crate::into_domain(result.mode),
        name: crate::into_domain(result.name),
        offlabel: crate::into_domain(result.offlabel),
        region: crate::into_domain(result.region),
        secret: crate::into_domain(result.secret),
    }
}
