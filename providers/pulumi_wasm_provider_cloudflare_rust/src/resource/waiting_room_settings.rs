//! Configure zone-wide settings for Cloudflare waiting rooms.
//!
//! ## Example Usage
//!
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as cloudflare from "@pulumi/cloudflare";
//!
//! const example = new cloudflare.WaitingRoomSettings("example", {
//!     searchEngineCrawlerBypass: true,
//!     zoneId: "0da42c8d2132a9ddaf714f9e7c920711",
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//!
//! example = cloudflare.WaitingRoomSettings("example",
//!     search_engine_crawler_bypass=True,
//!     zone_id="0da42c8d2132a9ddaf714f9e7c920711")
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
//!     var example = new Cloudflare.WaitingRoomSettings("example", new()
//!     {
//!         SearchEngineCrawlerBypass = true,
//!         ZoneId = "0da42c8d2132a9ddaf714f9e7c920711",
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
//! 		_, err := cloudflare.NewWaitingRoomSettings(ctx, "example", &cloudflare.WaitingRoomSettingsArgs{
//! 			SearchEngineCrawlerBypass: pulumi.Bool(true),
//! 			ZoneId:                    pulumi.String("0da42c8d2132a9ddaf714f9e7c920711"),
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
//! import com.pulumi.cloudflare.WaitingRoomSettings;
//! import com.pulumi.cloudflare.WaitingRoomSettingsArgs;
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
//!         var example = new WaitingRoomSettings("example", WaitingRoomSettingsArgs.builder()        
//!             .searchEngineCrawlerBypass(true)
//!             .zoneId("0da42c8d2132a9ddaf714f9e7c920711")
//!             .build());
//!
//!     }
//! }
//! ```
//! ### YAML
//! ```yaml
//! resources:
//!   example:
//!     type: cloudflare:WaitingRoomSettings
//!     properties:
//!       searchEngineCrawlerBypass: true
//!       zoneId: 0da42c8d2132a9ddaf714f9e7c920711
//! ```
//! <!--End PulumiCodeChooser -->
//!
//! ## Import
//!
//! ```sh
//! $ pulumi import cloudflare:index/waitingRoomSettings:WaitingRoomSettings example <zone_id>
//! ```
//!

pub struct WaitingRoomSettingsArgs {
    /// Whether to allow verified search engine crawlers to bypass all waiting rooms on this zone. Defaults to `false`.
    pub search_engine_crawler_bypass: pulumi_wasm_rust::Output<Option<bool>>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct WaitingRoomSettingsResult {
    /// Whether to allow verified search engine crawlers to bypass all waiting rooms on this zone. Defaults to `false`.
    pub search_engine_crawler_bypass: pulumi_wasm_rust::Output<Option<bool>>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: WaitingRoomSettingsArgs) -> WaitingRoomSettingsResult {
    let result = crate::bindings::pulumi::cloudflare::waiting_room_settings::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::waiting_room_settings::Args {
            search_engine_crawler_bypass: &args.search_engine_crawler_bypass.get_inner(),
            zone_id: &args.zone_id.get_inner(),
        },
    );

    WaitingRoomSettingsResult {
        search_engine_crawler_bypass: crate::into_domain(result.search_engine_crawler_bypass),
        zone_id: crate::into_domain(result.zone_id),
    }
}
